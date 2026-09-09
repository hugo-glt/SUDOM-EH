//! Définition des routes HTTP et des formats de requête/réponse.

use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::erreur::ApiError;
use crate::etat::{AppState, Partie};
use crate::jeu::Statut;

/// Construit le routeur de l'application avec son état.
pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Motus backend running" }))
        .route("/nouvelle-partie", post(nouvelle_partie))
        .route("/essai", post(essai))
        .with_state(state)
}

#[derive(Serialize)]
struct NouvellePartieResponse {
    id_partie: Uuid,
    longueur: usize,
    premiere_lettre: char,
    essais_restants: u8,
}

async fn nouvelle_partie(State(state): State<Arc<AppState>>) -> Json<NouvellePartieResponse> {
    let secret = state.dictionnaire.mot_aleatoire().to_string();
    let partie = Partie::nouvelle(&secret);
    let id_partie = Uuid::new_v4();

    let reponse = NouvellePartieResponse {
        id_partie,
        longueur: partie.longueur(),
        premiere_lettre: partie.premiere_lettre(),
        essais_restants: partie.essais_restants,
    };

    state
        .parties
        .lock()
        .expect("mutex parties empoisonné")
        .insert(id_partie, partie);

    tracing::debug!(%id_partie, secret, "nouvelle partie");
    Json(reponse)
}

#[derive(Deserialize)]
struct EssaiRequest {
    id_partie: Uuid,
    mot: String,
}

#[derive(Serialize)]
struct EssaiResponse {
    resultat: Vec<Statut>,
    essais_restants: u8,
    termine: bool,
    gagne: bool,
    /// Rempli uniquement quand la partie est perdue.
    #[serde(skip_serializing_if = "Option::is_none")]
    mot_secret: Option<String>,
}

async fn essai(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EssaiRequest>,
) -> Result<Json<EssaiResponse>, ApiError> {
    let mot = payload.mot.trim().to_lowercase();

    // Validation qui ne dépend pas de l'état : faite hors du verrou.
    if !state.dictionnaire.contient(&mot) {
        return Err(ApiError::MotInconnu);
    }
    let lettres: Vec<char> = mot.chars().collect();

    let mut parties = state.parties.lock().expect("mutex parties empoisonné");
    let partie = parties
        .get_mut(&payload.id_partie)
        .ok_or(ApiError::PartieIntrouvable)?;

    if partie.termine {
        return Err(ApiError::PartieTerminee);
    }
    if lettres.len() != partie.longueur() {
        return Err(ApiError::MauvaiseLongueur {
            attendue: partie.longueur(),
        });
    }

    let resultat = partie.jouer(&lettres);
    let mot_secret = (partie.termine && !partie.gagne).then(|| partie.secret_texte());

    Ok(Json(EssaiResponse {
        resultat,
        essais_restants: partie.essais_restants,
        termine: partie.termine,
        gagne: partie.gagne,
        mot_secret,
    }))
}
