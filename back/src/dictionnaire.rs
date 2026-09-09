//! Chargement et filtrage de la liste de mots jouables.

use std::collections::HashSet;

use rand::Rng;

/// Longueurs de mots acceptées (bornes incluses), à la manière de SUTOM.
const LONGUEUR_MIN: usize = 6;
const LONGUEUR_MAX: usize = 9;

/// Liste de mots jouables : tirage aléatoire + validation des essais.
pub struct Dictionnaire {
    /// Mots triés, pour le tirage.
    mots: Vec<String>,
    /// Même contenu, pour une validation en O(1).
    index: HashSet<String>,
}

impl Dictionnaire {
    /// Charge le dictionnaire embarqué dans le binaire (`back/data/mots.csv`).
    pub fn charger() -> Self {
        Self::depuis_texte(include_str!("../data/mots.csv"))
    }

    fn depuis_texte(texte: &str) -> Self {
        let index: HashSet<String> = texte.lines().filter_map(normaliser).collect();
        let mut mots: Vec<String> = index.iter().cloned().collect();
        mots.sort_unstable();
        Self { mots, index }
    }

    pub fn len(&self) -> usize {
        self.mots.len()
    }

    pub fn est_vide(&self) -> bool {
        self.mots.is_empty()
    }

    /// Indique si `mot` (déjà en minuscules) fait partie du dictionnaire.
    pub fn contient(&self, mot: &str) -> bool {
        self.index.contains(mot)
    }

    /// Tire un mot au hasard. Suppose le dictionnaire non vide (vérifié au démarrage).
    pub fn mot_aleatoire(&self) -> &str {
        let i = rand::rng().random_range(0..self.mots.len());
        &self.mots[i]
    }
}

/// Nettoie une ligne du CSV et ne garde que les mots réellement jouables :
/// guillemets retirés, uniquement des lettres (accents compris), longueur bornée,
/// ni acronymes ni noms propres (présence d'une majuscule à la source).
fn normaliser(ligne: &str) -> Option<String> {
    let brut = ligne.trim().trim_matches('"').trim();

    if brut.is_empty() || brut.chars().any(char::is_uppercase) {
        return None;
    }
    if !brut.chars().all(char::is_alphabetic) {
        return None;
    }
    if !(LONGUEUR_MIN..=LONGUEUR_MAX).contains(&brut.chars().count()) {
        return None;
    }

    Some(brut.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalise_un_mot_simple() {
        assert_eq!(normaliser("\"maison\""), Some("maison".to_string()));
        assert_eq!(normaliser("  fenetre "), Some("fenetre".to_string()));
    }

    #[test]
    fn garde_les_accents() {
        assert_eq!(normaliser("\"éléphant\""), Some("éléphant".to_string()));
    }

    #[test]
    fn rejette_les_entrees_non_jouables() {
        assert_eq!(normaliser("\"1000e\""), None); // chiffres
        assert_eq!(normaliser("\"5 à 7\""), None); // espaces
        assert_eq!(normaliser("\"AAAAA\""), None); // acronyme
        assert_eq!(normaliser("\"Paris\""), None); // nom propre
        assert_eq!(normaliser("\"chat\""), None); // trop court
        assert_eq!(normaliser("\"anticonstitutionnel\""), None); // trop long
        assert_eq!(normaliser(""), None);
    }

    #[test]
    fn le_dictionnaire_embarque_est_utilisable() {
        let dico = Dictionnaire::charger();
        assert!(!dico.est_vide());
        let mot = dico.mot_aleatoire();
        assert!(dico.contient(mot));
        assert!((LONGUEUR_MIN..=LONGUEUR_MAX).contains(&mot.chars().count()));
    }
}
