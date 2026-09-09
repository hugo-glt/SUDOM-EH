//! Configuration lue depuis l'environnement (voir `.envrc` à la racine du dépôt).

use std::env;

/// Paramètres d'exécution du serveur.
pub struct Config {
    /// Port d'écoute HTTP.
    pub port: u16,
    /// Origine autorisée pour le CORS (l'URL du front).
    pub origine_front: String,
}

impl Config {
    /// Construit la configuration à partir des variables d'environnement, en
    /// retombant sur des valeurs par défaut adaptées au développement local.
    ///
    /// - `BACK_PORT` (défaut : `3000`)
    /// - `FRONT_ORIGIN` (défaut : `http://localhost:5173`)
    pub fn depuis_env() -> Self {
        let port = env::var("BACK_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3000);

        let origine_front =
            env::var("FRONT_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());

        Self {
            port,
            origine_front,
        }
    }
}
