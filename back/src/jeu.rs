//! Règle du jeu : comparaison d'un essai au mot secret.

use std::collections::HashMap;

use serde::Serialize;

/// Statut d'une lettre dans un essai.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Statut {
    /// Bonne lettre, bonne position.
    Correct,
    /// Bonne lettre, mauvaise position.
    Present,
    /// Lettre absente (ou déjà toute « consommée » ailleurs).
    Absent,
}

/// Compare `essai` à `secret`, lettre à lettre, façon Wordle/SUTOM.
///
/// Les deux tranches doivent avoir la même longueur (garanti par l'appelant).
/// Les lettres bien placées sont résolues en premier, puis les lettres
/// présentes sont attribuées dans la limite des occurrences restantes du secret,
/// ce qui gère correctement les lettres en double.
pub fn comparer(essai: &[char], secret: &[char]) -> Vec<Statut> {
    debug_assert_eq!(essai.len(), secret.len());

    let mut resultat = vec![Statut::Absent; essai.len()];
    let mut restantes: HashMap<char, usize> = HashMap::new();

    // 1re passe : lettres bien placées ; on compte les autres lettres du secret.
    for (i, (&e, &s)) in essai.iter().zip(secret).enumerate() {
        if e == s {
            resultat[i] = Statut::Correct;
        } else {
            *restantes.entry(s).or_insert(0) += 1;
        }
    }

    // 2e passe : lettres présentes mais mal placées.
    for (i, &e) in essai.iter().enumerate() {
        if resultat[i] == Statut::Correct {
            continue;
        }
        if let Some(n) = restantes.get_mut(&e)
            && *n > 0
        {
            resultat[i] = Statut::Present;
            *n -= 1;
        }
    }

    resultat
}

#[cfg(test)]
mod tests {
    use super::Statut::*;
    use super::*;

    fn compare(essai: &str, secret: &str) -> Vec<Statut> {
        comparer(
            &essai.chars().collect::<Vec<_>>(),
            &secret.chars().collect::<Vec<_>>(),
        )
    }

    #[test]
    fn mot_exact() {
        assert_eq!(compare("maison", "maison"), vec![Correct; 6]);
    }

    #[test]
    fn aucune_lettre_commune() {
        assert_eq!(compare("abcdef", "ghijkl"), vec![Absent; 6]);
    }

    #[test]
    fn anagramme_tout_present() {
        assert_eq!(compare("niche", "chien"), vec![Present; 5]);
    }

    #[test]
    fn lettre_en_double_dans_lessai_unique_dans_le_secret() {
        // "eleves" vs "cheval" : le seul 'e' du secret est bien placé (index 2),
        // donc tous les autres 'e' de l'essai sont absents ; le 'l' est présent.
        assert_eq!(
            compare("eleves", "cheval"),
            vec![Absent, Present, Correct, Correct, Absent, Absent]
        );
    }

    #[test]
    fn priorite_aux_lettres_bien_placees() {
        // "llama" vs "koala" : un seul 'l' disponible (l'autre est déjà Correct
        // ailleurs), donc le 1er 'l' est présent et le 2e absent.
        assert_eq!(
            compare("llama", "koala"),
            vec![Present, Absent, Correct, Absent, Correct]
        );
    }
}
