/** Appels à l'API de jeu. */

import { postJson } from './client'
import type { NouvellePartie, ResultatEssai } from '@/types/game'

/** Démarre une nouvelle partie. */
export function creerPartie(): Promise<NouvellePartie> {
  return postJson<NouvellePartie>('/nouvelle-partie')
}

/** Soumet un essai pour la partie donnée. */
export function soumettreEssai(idPartie: string, mot: string): Promise<ResultatEssai> {
  return postJson<ResultatEssai>('/essai', { id_partie: idPartie, mot })
}
