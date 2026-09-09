//! Erreurs de l'API, converties en réponses HTTP JSON.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, PartialEq, Eq)]
pub enum ApiError {
    /// Aucune partie ne correspond à l'identifiant fourni.
    PartieIntrouvable,
    /// La partie est déjà gagnée ou perdue.
    PartieTerminee,
    /// Le mot proposé n'est pas dans le dictionnaire.
    MotInconnu,
    /// Le mot proposé n'a pas la bonne longueur.
    MauvaiseLongueur { attendue: usize },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            ApiError::PartieIntrouvable => {
                (StatusCode::NOT_FOUND, "partie introuvable".to_string())
            }
            ApiError::PartieTerminee => {
                (StatusCode::CONFLICT, "la partie est terminée".to_string())
            }
            ApiError::MotInconnu => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "mot absent du dictionnaire".to_string(),
            ),
            ApiError::MauvaiseLongueur { attendue } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("le mot doit faire {attendue} lettres"),
            ),
        };

        (code, Json(json!({ "erreur": message }))).into_response()
    }
}
