# SUDOM-EH — vue d'ensemble du projet

Jeu de mots type **Motus / SUTOM** : deviner un mot en 6 essais, chaque lettre
renvoyée comme *bien placée*, *mal placée* ou *absente*.

Le projet est un mono-dépôt à deux parties :

| Dossier | Stack | Rôle |
|---|---|---|
| `back/` | Rust + Axum + Tokio | API HTTP, tire le mot secret, tient l'état des parties, applique les règles |
| `front/` | Vue 3 + TypeScript + Vite + Tailwind 4 | Interface : grille, saisie clavier, affichage des résultats |

> État des branches (au moment de l'écriture) : le back modularisé vit sur la
> branche `back`, la logique de jeu du front sur `feat/front-jeu-etat`. `main`
> ne contient qu'une ébauche. Voir aussi [front/docs/jeu-front.md](../front/docs/jeu-front.md)
> et [back/docs/back-rust.md](../back/docs/back-rust.md).

## Principe directeur : le backend fait autorité

Toute la logique de jeu qui compte est côté serveur :

- choix du mot secret et tirage aléatoire,
- validité du mot proposé (présent dans le dictionnaire, bonne longueur),
- comptage des essais, détection de la victoire / défaite,
- comparaison lettre par lettre.

Le front ne connaît jamais le mot secret (sauf en cas de défaite, où le serveur
le révèle). Il se contente de collecter la saisie et d'afficher ce que renvoie
l'API. Impossible de tricher depuis le navigateur.

## Le contrat d'API

Deux routes, en JSON, servies par le back sur `http://localhost:3000` par défaut.

### `POST /nouvelle-partie`

Démarre une partie. Corps vide.

```jsonc
// réponse
{
  "id_partie": "uuid",
  "longueur": 7,            // nombre de lettres du mot secret
  "premiere_lettre": "m",   // imposée dès le départ (comme au SUTOM)
  "essais_restants": 6
}
```

Le serveur range la partie en mémoire sous `id_partie` ; le front garde cet id
pour les essais suivants.

### `POST /essai`

Soumet une tentative.

```jsonc
// requête
{ "id_partie": "uuid", "mot": "maville" }

// réponse
{
  "resultat": ["correct", "present", "absent", ...],  // un statut par lettre
  "essais_restants": 5,
  "termine": false,
  "gagne": false,
  "mot_secret": "maisons"   // présent UNIQUEMENT si la partie est perdue
}
```

Statut d'une lettre : `correct` (bonne lettre, bonne place), `present` (bonne
lettre, mauvaise place), `absent`.

### Erreurs

Réponse `{ "erreur": "message" }` avec un code HTTP :

| Code | Cas |
|---|---|
| `404` | `id_partie` inconnu |
| `409` | la partie est déjà terminée |
| `422` | mot absent du dictionnaire, ou mauvaise longueur |
| `0` (côté front) | serveur injoignable (pas une vraie réponse HTTP) |

## Déroulé d'une partie

```
Navigateur (front)                          Serveur (back)
------------------                          --------------
au montage du composant
  useGame() → demarrer()
      │  POST /nouvelle-partie ───────────▶ tire un mot du dictionnaire
      │                                     crée Partie{secret, 6 essais}
      │  ◀─── longueur, 1re lettre, uuid    la range dans AppState.parties
  préremplit la saisie avec la 1re lettre
  phase = "en-cours"

l'utilisateur tape (clavier physique)
  taper() / effacer() modifient `saisie`
  (une seule ligne "active" dans la grille)

Entrée → valider()
      │  POST /essai {uuid, mot} ─────────▶ mot dans le dictionnaire ? (sinon 422)
      │                                     partie trouvée ? terminée ? (404/409)
      │                                     bonne longueur ? (sinon 422)
      │                                     comparer(essai, secret)
      │                                     décrémente essais, calcule gagne/termine
      │  ◀─── resultat[], essais_restants,  révèle mot_secret si perdu
      │        termine, gagne, mot_secret?
  ajoute la tentative à la grille (colorée)
  gagne  → phase "gagne"
  termine→ phase "perdu" (affiche mot_secret)
  sinon  → réinitialise la saisie à la 1re lettre

partie finie → bouton "Rejouer" → demarrer()
```

## Règle de comparaison (`back/src/jeu.rs`)

Algorithme en deux passes, identique à Wordle / SUTOM, qui gère les lettres en
double :

1. **Passe 1** — marque toutes les lettres à la bonne position (`correct`) et
   compte, pour le reste, combien de fois chaque lettre apparaît encore dans le
   secret.
2. **Passe 2** — pour les positions non résolues, attribue `present` tant qu'il
   reste des occurrences disponibles de cette lettre, sinon `absent`.

Ainsi une lettre tapée deux fois mais présente une seule fois dans le secret
n'est colorée qu'une fois.

## Le dictionnaire

- Source : `back/data/mots.csv` (~29 700 lignes brutes, entre guillemets).
- **Embarqué dans le binaire** via `include_str!` — rien à déployer à côté.
- Filtré au chargement (`dictionnaire::normaliser`) : on ne garde que les mots
  de 6 à 9 lettres, en minuscules, uniquement alphabétiques (accents compris),
  sans majuscule à la source (élimine acronymes et noms propres).
- Sert à deux choses : tirer le mot secret, et valider chaque essai en O(1).

> `front/src/data/noun.csv` et le `main.ts` d'origine étaient une tentative de
> dictionnaire *côté client* ; c'est le back qui fait foi désormais.

## Côté back — organisation (branche `back`)

Serveur Axom découpé en modules :

| Fichier | Contenu |
|---|---|
| `src/main.rs` | démarrage : config, dictionnaire, état partagé, CORS, logs `tracing`, serveur, arrêt gracieux (Ctrl-C), tâche de purge |
| `src/config.rs` | `Config::depuis_env()` — `BACK_PORT`, `FRONT_ORIGIN` |
| `src/dictionnaire.rs` | chargement + filtrage des mots, tirage aléatoire (`rand`), validation |
| `src/etat.rs` | `AppState` (`Mutex<HashMap<Uuid, Partie>>` + `purger()`), `Partie` (secret, essais, `jouer()`) |
| `src/jeu.rs` | `comparer()` + `enum Statut` |
| `src/routes.rs` | routes HTTP et structs de requête/réponse |
| `src/erreur.rs` | `ApiError` → réponse HTTP JSON |

Points notables : parties **en mémoire uniquement** (pas de base de données) ;
purge automatique toutes les 5 min des parties terminées ou inactives depuis
plus d'1 h (TTL) ; `Cargo.lock` versionné (c'est un binaire).

## Côté front — organisation (branche `feat/front-jeu-etat`)

Vue 3 `<script setup>` + Tailwind 4.

| Fichier | Contenu |
|---|---|
| `src/api/client.ts` | `postJson()` générique + `ApiError` ; base URL = `VITE_API_BASE_URL` |
| `src/api/game.ts` | `creerPartie()`, `soumettreEssai()` |
| `src/types/game.ts` | types partagés (`Statut`, `Phase`, réponses API, `LigneGrille`…) |
| `src/composables/useGame.ts` | tout l'état d'une partie + actions `demarrer` / `taper` / `effacer` / `valider` ; construit la `grille` à afficher |
| `src/components/game/GameBoard.vue` | branche le clavier physique, affiche grille + messages + bouton « Rejouer » |
| `src/components/game/GameRow.vue` | une ligne = liste de `GameTile` |
| `src/components/game/GameTile.vue` | une case : lettre + couleur selon le statut |

Le clavier est **physique** (écouteur `keydown` sur `window`) : pas de clavier à
l'écran. `Entrée` valide, `Backspace` efface, les lettres s'ajoutent ; la
première lettre imposée ne peut pas être effacée.

> `GameBoard.vue` n'est pas encore branché dans `App.vue` sur la branche (App
> affiche toujours `HelloWorld`). C'est l'étape d'intégration restante.

## Lancer le projet en local

Config centralisée dans `.envrc` (via [direnv](https://direnv.net/)) :
`BACK_PORT=3000`, `FRONT_PORT=5173`, `VITE_API_BASE_URL=http://localhost:3000`,
`RUST_LOG=debug`. Toolchains épinglées : Rust `stable` (`rust-toolchain.toml`),
Node `20` (`.nvmrc`).

```bash
# terminal 1 — backend
cd back && cargo run

# terminal 2 — frontend
cd front && npm install && npm run dev
```

Le front (`:5173`) appelle le back (`:3000`) ; le back n'autorise en CORS que
l'origine `FRONT_ORIGIN`.

## Choix d'architecture, en bref

- **Serveur autoritaire** : aucune règle exploitable côté client.
- **État en mémoire** : simple, suffisant pour un jeu sans comptes ; les parties
  sont éphémères et purgées.
- **Dictionnaire embarqué** : déploiement = un seul binaire.
- **Contrat JSON minimal** : deux routes, des messages d'erreur explicites.
- **Séparation front nette** : `api/` (transport) → `composables/` (état/logique)
  → `components/` (affichage pur).
