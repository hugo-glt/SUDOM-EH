/** Appels à l'API de jeu. */

import { postJson } from './client'
import type { NouvellePartie, ResultatEssai } from '@/types/game'

export type Statut = 'correct' | 'present' | 'absent' | 'vide'

export interface Case {
  lettre: string
  statut: Statut
}

export type LigneGrille = Case[]

// Ce que le back doit fournir à la création de partie
export interface Partie {
  id: string
  longueurMot: number   // <-- clé pour la grille adaptative
  maxTentatives: number
}

/** Démarre une nouvelle partie. */
export function creerPartie(): Promise<NouvellePartie> {
  return postJson<NouvellePartie>('/nouvelle-partie')
}

/** Soumet un essai pour la partie donnée. */
export function soumettreEssai(idPartie: string, mot: string): Promise<ResultatEssai> {
  return postJson<ResultatEssai>('/essai', { id_partie: idPartie, mot })
}
