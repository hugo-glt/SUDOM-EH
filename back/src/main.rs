use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use http::{HeaderValue, Method, header};
use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};
use tower_http::cors::CorsLayer;
use uuid::Uuid;

// --- État partagé du serveur ---

struct AppState {
    dictionnaire: HashSet<String>,
    parties: Mutex<HashMap<Uuid, GameState>>,
}

struct GameState {
    mot_secret: String,
    tentatives_restantes: u8,
}

// --- Formats des requêtes/réponses ---

#[derive(Serialize)]
struct NouvellePartieResponse {
    id_partie: Uuid,
}

#[derive(Deserialize)]
struct EssaiRequest {
    id_partie: Uuid,
    mot: String,
}

#[derive(Serialize)]
struct EssaiResponse {
    resultat: Vec<Statut>,
}

#[derive(Serialize, Clone, Copy)]
enum Statut {
    Correct,
    Present,
    Absent,
}

// --- Chargement du dictionnaire ---

fn charger_dictionnaire() -> HashSet<String> {
    let contenu = include_str!("../../front/src/data/noun.csv");
    contenu
        .lines()
        .skip(1) // retire cette ligne si pas d'en-tête
        .map(|l| l.trim().to_lowercase())
        .filter(|l| !l.is_empty())
        .collect()
}

// --- Handlers ---

async fn nouvelle_partie(State(state): State<Arc<AppState>>) -> Json<NouvellePartieResponse> {
    let mot_secret = state
        .dictionnaire
        .iter()
        .nth(rand_index(state.dictionnaire.len()))
        .unwrap()
        .clone();

    let id_partie = Uuid::new_v4();
    state.parties.lock().unwrap().insert(
        id_partie,
        GameState { mot_secret, tentatives_restantes: 6 },
    );

    Json(NouvellePartieResponse { id_partie })
}

async fn essai(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EssaiRequest>,
) -> Json<EssaiResponse> {
    let mut parties = state.parties.lock().unwrap();
    let partie = parties.get_mut(&payload.id_partie).expect("partie introuvable");

    let resultat = comparer(&payload.mot.to_lowercase(), &partie.mot_secret);
    partie.tentatives_restantes = partie.tentatives_restantes.saturating_sub(1);

    Json(EssaiResponse { resultat })
}

fn comparer(essai: &str, mot_secret: &str) -> Vec<Statut> {
    let essai: Vec<char> = essai.chars().collect();
    let secret: Vec<char> = mot_secret.chars().collect();
    let mut resultat = vec![Statut::Absent; essai.len()];
    let mut freq: HashMap<char, i32> = HashMap::new();

    for &c in &secret {
        *freq.entry(c).or_insert(0) += 1;
    }

    // Première passe : lettres bien placées
    for i in 0..essai.len().min(secret.len()) {
        if essai[i] == secret[i] {
            resultat[i] = Statut::Correct;
            *freq.get_mut(&essai[i]).unwrap() -= 1;
        }
    }

    // Deuxième passe : lettres mal placées
    for i in 0..essai.len() {
        if let Statut::Correct = resultat[i] {
            continue;
        }
        if let Some(count) = freq.get_mut(&essai[i]) {
            if *count > 0 {
                resultat[i] = Statut::Present;
                *count -= 1;
            }
        }
    }

    resultat
}

fn rand_index(len: usize) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    (nanos as usize) % len
}

// --- Point d'entrée ---

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        dictionnaire: charger_dictionnaire(),
        parties: Mutex::new(HashMap::new()),
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(|| async { "Motus backend running" }))
        .route("/nouvelle-partie", post(nouvelle_partie))
        .route("/essai", post(essai))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Serveur lancé sur http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}