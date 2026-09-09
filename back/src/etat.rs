//! État partagé du serveur et modèle d'une partie en cours.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use uuid::Uuid;

use crate::dictionnaire::Dictionnaire;

/// Nombre d'essais accordés par partie.
pub const ESSAIS_MAX: u8 = 6;

/// Durée au-delà de laquelle une partie inactive est purgée.
pub const TTL_PARTIE: Duration = Duration::from_secs(60 * 60);

/// État global, partagé entre tous les handlers via `Arc`.
pub struct AppState {
    pub dictionnaire: Dictionnaire,
    pub parties: Mutex<HashMap<Uuid, Partie>>,
}

impl AppState {
    pub fn new(dictionnaire: Dictionnaire) -> Self {
        Self {
            dictionnaire,
            parties: Mutex::new(HashMap::new()),
        }
    }

    /// Retire les parties terminées ou expirées. Renvoie le nombre supprimé.
    pub fn purger(&self) -> usize {
        let mut parties = self.parties.lock().expect("mutex parties empoisonné");
        let avant = parties.len();
        parties.retain(|_, p| !p.termine && p.creee_a.elapsed() < TTL_PARTIE);
        avant - parties.len()
    }
}

/// Une partie en cours : mot secret et progression.
pub struct Partie {
    secret: Vec<char>,
    pub essais_restants: u8,
    pub termine: bool,
    pub gagne: bool,
    pub creee_a: Instant,
}

impl Partie {
    pub fn nouvelle(secret: &str) -> Self {
        Self {
            secret: secret.chars().collect(),
            essais_restants: ESSAIS_MAX,
            termine: false,
            gagne: false,
            creee_a: Instant::now(),
        }
    }

    pub fn longueur(&self) -> usize {
        self.secret.len()
    }

    pub fn premiere_lettre(&self) -> char {
        self.secret[0]
    }

    pub fn secret_texte(&self) -> String {
        self.secret.iter().collect()
    }

    /// Enregistre un essai (déjà validé : bon dictionnaire, bonne longueur) et
    /// renvoie le statut de chaque lettre.
    pub fn jouer(&mut self, essai: &[char]) -> Vec<crate::jeu::Statut> {
        use crate::jeu::{Statut, comparer};

        let resultat = comparer(essai, &self.secret);
        self.essais_restants = self.essais_restants.saturating_sub(1);
        self.gagne = resultat.iter().all(|s| *s == Statut::Correct);
        self.termine = self.gagne || self.essais_restants == 0;
        resultat
    }
}
