# Front — logique et état du jeu

Branche `feat/front-jeu-etat`. Ajoute toute la partie « jeu » (type SUTOM) côté
front : appels API, état de la partie, et composants d'affichage de la grille.

Le **backend fait autorité** (mot secret, validité du mot, longueur, comptage des
essais, victoire/défaite). Le front ne fait que gérer la saisie clavier et
refléter les réponses de l'API.

## Vue d'ensemble

```
GameBoard.vue        (écoute le clavier, orchestre l'affichage)
  └── useGame()       (état de la partie + actions)
        └── api/game.ts     (creerPartie / soumettreEssai)
              └── api/client.ts   (POST JSON + gestion d'erreurs)
  └── GameRow.vue     (une ligne = liste de tuiles)
        └── GameTile.vue  (une case : lettre + couleur selon le statut)

types/game.ts          (types partagés par tous les fichiers ci-dessus)
```

## Fichiers

### `src/api/client.ts`
Client HTTP minimal pour parler au backend.

- `BASE_URL` : lue depuis `VITE_API_BASE_URL`, sinon `http://localhost:3000`.
- `ApiError` : erreur personnalisée avec un champ `statut` (code HTTP, ou `0` si
  le serveur est injoignable).
- `postJson<T>(chemin, corps)` : envoie un `POST` JSON, parse la réponse.
  - Si succès → renvoie le corps typé `T`.
  - Si échec → lève une `ApiError` avec le message renvoyé par l'API
    (`{ "erreur": "..." }`) ou un message générique.

### `src/api/game.ts`
Les deux appels métier du jeu, construits sur `postJson`.

- `creerPartie()` → `POST /nouvelle-partie` : démarre une partie, renvoie
  `NouvellePartie`.
- `soumettreEssai(idPartie, mot)` → `POST /essai` avec `{ id_partie, mot }` :
  renvoie `ResultatEssai`.

### `src/types/game.ts`
Types TypeScript partagés (aucune logique).

- `Statut` : `'correct' | 'present' | 'absent'` (statut d'une lettre).
- `NouvellePartie` : réponse de `/nouvelle-partie` (`id_partie`, `longueur`,
  `premiere_lettre`, `essais_restants`).
- `ResultatEssai` : réponse de `/essai` (`resultat`, `essais_restants`,
  `termine`, `gagne`, `mot_secret?` présent seulement en cas de défaite).
- `Phase` : état courant côté front —
  `'chargement' | 'en-cours' | 'gagne' | 'perdu' | 'erreur'`.
- `Tentative` : un essai déjà joué (`lettres[]` + `statuts[]`).
- `LigneGrille` : une ligne prête à afficher (`lettres[]`, `statuts` ou `null`,
  `active`).

### `src/composables/useGame.ts`
Cœur de la logique : un composable Vue qui tient l'état d'**une** partie.

État réactif (`ref`) : `phase`, `longueur`, `premiereLettre`, `essaisMax`,
`essaisRestants`, `tentatives`, `saisie`, `motSecret`, `erreur`,
`envoiEnCours`. `idPartie` est gardé en variable privée.

Valeurs dérivées (`computed`) :
- `enCours`, `terminee` : raccourcis sur `phase`.
- `peutValider` : `true` si la partie est en cours, aucun envoi en cours, et la
  saisie a la bonne longueur.
- `grille` : construit les lignes à afficher = tentatives passées + ligne active
  (saisie en cours, complétée de cases vides) + lignes vides jusqu'à `essaisMax`.

Actions :
- `demarrer()` : appelle `creerPartie()`, initialise l'état, préremplit la
  `saisie` avec la première lettre imposée, passe en `'en-cours'`. En cas
  d'échec → `phase = 'erreur'`.
- `taper(caractere)` : ajoute une lettre à la saisie (une seule lettre Unicode,
  accents compris ; ignoré si partie non en cours ou saisie déjà pleine).
- `effacer()` : retire la dernière lettre, sans jamais toucher à la première
  lettre imposée.
- `valider()` : envoie `soumettreEssai()`, ajoute la tentative, met à jour
  `essaisRestants`, puis selon la réponse passe en `'gagne'` / `'perdu'`
  (stocke `motSecret`) ou réinitialise la saisie à la première lettre. Une
  erreur (ex. 422 mot inconnu) est affichée sans effacer la saisie.

`onMounted(demarrer)` : une partie démarre automatiquement au montage.

Le composable renvoie l'état (en lecture seule côté consommateur), les dérivés
et les actions.

### `src/components/game/GameBoard.vue`
Composant racine du jeu. Appelle `useGame()` et câble le clavier.

- `onKeydown` : `Enter` → `valider()`, `Backspace` → `effacer()`, toute touche
  d'un seul caractère → `taper()`. Ignore les raccourcis avec `Cmd/Ctrl/Alt`.
- Écouteur `keydown` ajouté sur `window` au montage, retiré au démontage.
- Template : « Chargement… » pendant `phase === 'chargement'` ; sinon la grille
  (`GameRow` × lignes), une ligne de message (erreur, « Gagné en N essai(s) »,
  « Perdu — le mot était … », ou nombre d'essais restants), et un bouton
  « Rejouer » quand la partie est terminée ou en erreur.

### `src/components/game/GameRow.vue`
Présentation pure : une ligne de la grille.

Props : `lettres: string[]`, `statuts: Statut[] | null`, `active?: boolean`.
Rend un `GameTile` par lettre, en passant `statuts?.[i] ?? null`.

### `src/components/game/GameTile.vue`
Présentation pure : une case.

Props : `lettre`, `statut: Statut | null`, `active?`.
`classes` (computed) choisit les couleurs Tailwind selon le statut :
- `correct` → rouge, `present` → ambre, `absent` → gris,
- pas de statut → fond blanc (bordure plus marquée si une lettre est présente).
`active` ajoute un anneau (`ring`) autour de la case.

### `tsconfig.app.json` (modifié)
Ajout de l'alias de chemins `"@/*": ["./src/*"]` pour que les imports
`@/api/...`, `@/composables/...`, `@/types/...` résolvent côté TypeScript
(l'équivalent Vite existe déjà dans `vite.config.ts`).
