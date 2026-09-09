/**
 * Petit client HTTP pour l'API du backend.
 * L'URL de base vient de `VITE_API_BASE_URL` (voir `.envrc`), défaut : localhost:3000.
 */

const BASE_URL =
  (import.meta.env.VITE_API_BASE_URL as string | undefined)?.replace(/\/$/, '') ??
  'http://localhost:3000'

/** Erreur renvoyée par l'API (corps `{ "erreur": "..." }`) ou réseau. */
export class ApiError extends Error {
  /** Code HTTP, ou `0` si le serveur est injoignable. */
  statut: number

  constructor(message: string, statut: number) {
    super(message)
    this.name = 'ApiError'
    this.statut = statut
  }
}

/** Envoie un POST JSON et renvoie le corps typé, ou lève une `ApiError`. */
export async function postJson<T>(chemin: string, corps: unknown = {}): Promise<T> {
  let reponse: Response
  try {
    reponse = await fetch(`${BASE_URL}${chemin}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(corps),
    })
  } catch {
    throw new ApiError('Serveur injoignable', 0)
  }

  const donnees: unknown = await reponse.json().catch(() => null)

  if (!reponse.ok) {
    const message =
      donnees && typeof donnees === 'object' && 'erreur' in donnees
        ? String((donnees as { erreur: unknown }).erreur)
        : `Erreur ${reponse.status}`
    throw new ApiError(message, reponse.status)
  }

  return donnees as T
}
