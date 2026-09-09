/** Statut d'une lettre renvoyé par le backend (voir back/src/jeu.rs). */
export type Statut = 'correct' | 'present' | 'absent'

/** Réponse de `POST /nouvelle-partie`. */
export interface NouvellePartie {
  id_partie: string
  longueur: number
  premiere_lettre: string
  essais_restants: number
}

/** Réponse de `POST /essai`. */
export interface ResultatEssai {
  resultat: Statut[]
  essais_restants: number
  termine: boolean
  gagne: boolean
  /** Présent uniquement quand la partie est perdue. */
  mot_secret?: string
}

/** Phase courante de la partie, côté front. */
export type Phase = 'chargement' | 'en-cours' | 'gagne' | 'perdu' | 'erreur'

/** Une tentative déjà soumise et évaluée. */
export interface Tentative {
  lettres: string[]
  statuts: Statut[]
}

/** Une ligne prête à afficher dans la grille. */
export interface LigneGrille {
  lettres: string[]
  statuts: Statut[] | null
  active: boolean
}
