# SUDOM-EH

Un **MOTUS / SUTOM** : deviner un mot en 6 essais, chaque lettre renvoyée comme
*bien placée*, *mal placée* ou *absente*.

Projet réalisé par **Emery et Hugo**.

## Stack technique

- **Front** : TypeScript + Vue + Vite + Tailwind
- **Back** : Rust + Axum

## Concept

Un seul dépôt GitHub, avec à la racine un dossier `back/` et un dossier `front/`.

- Le **back** assure l'API, le choix du mot secret, tient l'état des parties
  (en cours, gagnée, perdue) et applique les règles.
- Le **front** gère toute l'interface : la grille, la saisie, l'affichage des
  résultats.

> ⚠️ **Le front ne connaît jamais le mot secret** (sauf en cas de défaite, où le
> serveur le révèle). Il se contente de collecter la saisie et d'afficher ce que
> renvoie l'API. **Impossible de tricher depuis le navigateur.**

## API

Deux routes, servies par le back sur le port **3000** en localhost.

### `POST /nouvelle-partie`

```jsonc
// réponse
{
  "id_partie": "uuid",
  "longueur": 7,            // nombre de lettres du mot secret
  "premiere_lettre": "m",   // imposée dès le départ (comme au SUTOM)
  "essais_restants": 6
}
```

Les parties sont sauvegardées en mémoire côté serveur sous `id_partie`, et le
front garde cet id pour les essais suivants.

### `POST /essai`

```jsonc
// requête
{ "id_partie": "uuid", "mot": "maville" }

// réponse
{
  "resultat": ["correct", "present", "absent", "..."],  // un statut par lettre
  "essais_restants": 5,
  "termine": false,
  "gagne": false,
  "mot_secret": "maisons"   // présent UNIQUEMENT si la partie est perdue
}
```

Trois statuts possibles : `correct`, `present`, `absent`.

### Système d'erreurs

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
      │                                     crée Partie{ secret, 6 essais }
      │  ◀─── longueur, 1re lettre, uuid    la range dans AppState.parties
  préremplit la saisie avec la 1re lettre
  phase = "en-cours"

l'utilisateur tape (clavier physique)
  taper() / effacer() modifient `saisie`
  (une seule ligne "active" dans la grille)

Entrée → valider()
      │  POST /essai { uuid, mot } ───────▶ mot dans le dictionnaire ? (sinon 422)
      │                                     partie trouvée ? terminée ? (404 / 409)
      │                                     bonne longueur ? (sinon 422)
      │                                     comparer(essai, secret)
      │                                     décrémente essais, calcule gagne / termine
      │  ◀─── resultat[], essais_restants,  révèle mot_secret si perdu
      │        termine, gagne, mot_secret?
  ajoute la tentative à la grille (colorée)
  gagne   → phase "gagne"
  termine → phase "perdu" (affiche mot_secret)
  sinon   → réinitialise la saisie à la 1re lettre

partie finie → bouton "Rejouer" → demarrer()
```

## Règle de comparaison

Pour éviter que le jeu réponde `present` pour une lettre qui n'existe dans le
secret qu'à un emplacement **déjà trouvé**, `back/src/jeu.rs` compte le nombre
d'occurrences de chaque lettre et agit en conséquence (deux passes : lettres
bien placées d'abord, puis lettres présentes dans la limite des occurrences
restantes).

## Dictionnaire

Repris du modèle disponible sur GitLab. Stocké dans le back et **filtré pour ne
garder que les mots de 6 à 9 lettres**. Il sert à la fois à tirer le mot secret
et à valider chaque essai.

## Backend

Fait par **Emery** (j'ai forcé pour faire du Rust).

Serveur Axum découpé en modules :

| Fichier | Rôle |
|---|---|
| `src/main.rs` | démarrage du serveur |
| `src/config.rs` | configuration (variables d'environnement) |
| `src/dictionnaire.rs` | filtrage des mots, tirage aléatoire, validation |
| `src/etat.rs` | état partagé et modèle d'une partie |
| `src/jeu.rs` | comparaison essai / secret |
| `src/routes.rs` | routes HTTP, structures de requête / réponse |
| `src/erreur.rs` | erreurs → réponse HTTP JSON |

**Choix BDD : aucune.** Purge automatique toutes les 5 min des parties terminées
ou inactives depuis plus d'1 h.

## Frontend

Le frontend a été divisé en deux pour qu'on travaille tous les deux sur du
TypeScript :

1. **Jeu / état** — Emery
2. **Interface / interaction** — Hugo

### Partie jeu / état (Emery)

| Fichier | Rôle |
|---|---|
| `src/api/client.ts` | client HTTP, gestion des erreurs |
| `src/api/game.ts` | créer la partie, soumettre un essai |
| `src/types/game.ts` | types (statut, phase, réponses API) |
| `src/composables/useGame.ts` | état de la partie et actions (démarrer, valider, taper, effacer) |
| `src/components/game/GameBoard.vue` | branche le clavier, affiche la grille et le bouton « Rejouer » |
| `src/components/game/GameRow.vue` | une ligne de la grille |
| `src/components/game/GameTile.vue` | une case = une lettre, couleur selon le statut |

Choix assumé : **pas de clavier à l'écran** (saisie au clavier physique).

## Lancer le projet en local

Config dans `.envrc` (via [direnv](https://direnv.net/)) : `BACK_PORT=3000`,
`FRONT_PORT=5173`, `VITE_API_BASE_URL=http://localhost:3000`. Toolchains
épinglées : Rust `stable` (`rust-toolchain.toml`), Node `20` (`.nvmrc`).

```bash
# terminal 1 — backend
cd back && cargo run

# terminal 2 — frontend
cd front && npm install && npm run dev
```

## Documentation détaillée

- [docs/projet.md](docs/projet.md) — vue d'ensemble, contrat d'API, flux complet
- [back/docs/back-rust.md](back/docs/back-rust.md) — chaque fichier du backend
- [front/docs/jeu-front.md](front/docs/jeu-front.md) — chaque fichier du frontend (jeu / état)
- [front/docs/a-faire.md](front/docs/a-faire.md) — ce qu'il reste à faire côté front

## Note de fin (IA et disparités)

Pour le contexte : je (Emery) possède Claude Code, j'ai donc utilisé cet outil
pour la base de mon code, dans une démarche d'efficience. J'ai ensuite créé des
fichiers Markdown et décortiqué le code écrit, dans une démarche d'apprentissage
— ce qui m'a permis de produire cette documentation.

Je précise aussi que Hugo ne dispose pas d'une IA intégrée à son éditeur : il est
donc normal que sa partie soit *moins avancée*, sa tâche étant objectivement plus
complexe.
