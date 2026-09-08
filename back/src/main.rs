mod config;
mod dictionnaire;
mod erreur;
mod etat;
mod jeu;
mod routes;

use std::sync::Arc;
use std::time::Duration;

use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::dictionnaire::Dictionnaire;
use crate::etat::AppState;

/// Intervalle entre deux passes de nettoyage des parties.
const INTERVALLE_PURGE: Duration = Duration::from_secs(300);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::depuis_env();

    let dictionnaire = Dictionnaire::charger();
    assert!(
        !dictionnaire.est_vide(),
        "dictionnaire vide : vérifie back/data/mots.csv"
    );
    tracing::info!(mots = dictionnaire.len(), "dictionnaire chargé");

    let state = Arc::new(AppState::new(dictionnaire));
    tokio::spawn(boucle_purge(Arc::clone(&state)));

    let cors = CorsLayer::new()
        .allow_origin(
            config
                .origine_front
                .parse::<HeaderValue>()
                .expect("FRONT_ORIGIN n'est pas une origine valide"),
        )
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let app = routes::router(Arc::clone(&state))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let adresse = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&adresse)
        .await
        .unwrap_or_else(|e| panic!("impossible d'écouter sur {adresse} : {e}"));

    tracing::info!("serveur lancé sur http://{adresse}");
    axum::serve(listener, app)
        .with_graceful_shutdown(arret_gracieux())
        .await
        .expect("le serveur s'est arrêté sur une erreur");
}

/// Purge périodiquement les parties terminées ou expirées.
async fn boucle_purge(state: Arc<AppState>) {
    let mut tick = tokio::time::interval(INTERVALLE_PURGE);
    loop {
        tick.tick().await;
        let retirees = state.purger();
        if retirees > 0 {
            tracing::debug!(retirees, "parties purgées");
        }
    }
}

/// Attend Ctrl-C pour un arrêt propre.
async fn arret_gracieux() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("arrêt demandé, fermeture du serveur");
}
