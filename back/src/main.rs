use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;
use http::{HeaderValue, Method, header};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(|| async { "Motus backend running" }))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Serveur lancé sur http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

use std::collections::HashSet;

fn charger_dictionnaire() -> HashSet<String> {
    let contenu = include_str!("../../front/src/data/noun.csv");
    contenu
        .lines()
        .skip(1) // si la première ligne est un en-tête, sinon retire ce .skip(1)
        .map(|l| l.trim().to_lowercase())
        .filter(|l| !l.is_empty())
        .collect()
}