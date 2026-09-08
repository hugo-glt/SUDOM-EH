/**
 * État et logique d'une partie de SUTOM, côté client.
 *
 * Le backend fait autorité : validité du mot, longueur, comptage des essais,
 * détection de la victoire. Ce composable ne fait que gérer la saisie et
 * refléter les réponses de l'API.
 */

import { computed, onMounted, ref } from 'vue'

import { ApiError } from '@/api/client'
import { creerPartie, soumettreEssai } from '@/api/game'
import type { LigneGrille, Phase, Tentative } from '@/types/game'

/** N'accepte qu'un unique caractère alphabétique (accents compris). */
const EST_LETTRE = /^\p{L}$/u

export function useGame() {
  const phase = ref<Phase>('chargement')
  const longueur = ref(0)
  const premiereLettre = ref('')
  const essaisMax = ref(6)
  const essaisRestants = ref(6)
  const tentatives = ref<Tentative[]>([])
  const saisie = ref('')
  const motSecret = ref<string | null>(null)
  const erreur = ref<string | null>(null)

  let idPartie = ''
  const envoiEnCours = ref(false)

  const enCours = computed(() => phase.value === 'en-cours')
  const terminee = computed(() => phase.value === 'gagne' || phase.value === 'perdu')
  const peutValider = computed(
    () => enCours.value && !envoiEnCours.value && saisie.value.length === longueur.value,
  )

  /** Toutes les lignes de la grille, prêtes à afficher. */
  const grille = computed<LigneGrille[]>(() => {
    const lignes: LigneGrille[] = tentatives.value.map((t) => ({
      lettres: t.lettres,
      statuts: t.statuts,
      active: false,
    }))

    if (enCours.value) {
      const lettres = saisie.value.split('')
      while (lettres.length < longueur.value) lettres.push('')
      lignes.push({ lettres, statuts: null, active: true })
    }

    while (lignes.length < essaisMax.value) {
      lignes.push({
        lettres: Array<string>(longueur.value).fill(''),
        statuts: null,
        active: false,
      })
    }

    return lignes
  })

  /** Démarre (ou redémarre) une partie. */
  async function demarrer(): Promise<void> {
    phase.value = 'chargement'
    erreur.value = null
    tentatives.value = []
    motSecret.value = null

    try {
      const partie = await creerPartie()
      idPartie = partie.id_partie
      longueur.value = partie.longueur
      premiereLettre.value = partie.premiere_lettre.toLowerCase()
      essaisMax.value = partie.essais_restants
      essaisRestants.value = partie.essais_restants
      saisie.value = premiereLettre.value
      phase.value = 'en-cours'
    } catch (e) {
      phase.value = 'erreur'
      erreur.value = messageDErreur(e, 'Impossible de démarrer la partie')
    }
  }

  /** Ajoute une lettre à la saisie courante. */
  function taper(caractere: string): void {
    if (!enCours.value || envoiEnCours.value) return
    if (!EST_LETTRE.test(caractere)) return
    if (saisie.value.length >= longueur.value) return
    saisie.value += caractere.toLowerCase()
  }

  /** Efface la dernière lettre (la première lettre imposée reste). */
  function effacer(): void {
    if (!enCours.value || envoiEnCours.value) return
    if (saisie.value.length > premiereLettre.value.length) {
      saisie.value = saisie.value.slice(0, -1)
    }
  }

  /** Soumet la saisie courante au backend. */
  async function valider(): Promise<void> {
    if (!peutValider.value) return

    envoiEnCours.value = true
    erreur.value = null

    try {
      const mot = saisie.value
      const resultat = await soumettreEssai(idPartie, mot)

      tentatives.value.push({ lettres: mot.split(''), statuts: resultat.resultat })
      essaisRestants.value = resultat.essais_restants

      if (resultat.gagne) {
        phase.value = 'gagne'
      } else if (resultat.termine) {
        phase.value = 'perdu'
        motSecret.value = resultat.mot_secret ?? null
      } else {
        saisie.value = premiereLettre.value
      }
    } catch (e) {
      // 422 (mot inconnu / mauvaise longueur) : on garde la saisie pour corriger.
      erreur.value = messageDErreur(e, "Échec de l'envoi de l'essai")
    } finally {
      envoiEnCours.value = false
    }
  }

  onMounted(demarrer)

  return {
    // état (lecture seule côté consommateur)
    phase,
    longueur,
    premiereLettre,
    essaisMax,
    essaisRestants,
    tentatives,
    saisie,
    motSecret,
    erreur,
    envoiEnCours,
    // dérivés
    enCours,
    terminee,
    peutValider,
    grille,
    // actions
    demarrer,
    taper,
    effacer,
    valider,
  }
}

function messageDErreur(e: unknown, defaut: string): string {
  if (e instanceof ApiError) return e.message
  if (e instanceof Error) return e.message
  return defaut
}
