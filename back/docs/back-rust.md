# Back — backend Rust (Motus / SUTOM)

Branche `back`. Commit : *refactor(back): modularise le backend et solidifie la
logique de jeu*.

Serveur HTTP en **Axum** qui héberge une partie type SUTOM. Il tire un mot
secret, tient l'état de chaque partie en mémoire, évalue les essais lettre par
lettre et fait autorité sur toutes les règles.

Avant ce commit tout tenait dans un seul `main.rs`. Il est maintenant découpé en
modules, avec gestion d'erreurs propre, config par variables d'environnement,
purge des parties, logs `tracing` et arrêt gracieux.

## Vue d'ensemble

```
main.rs         démarrage : config, dictionnaire, état, CORS, serveur, tâche de purge
  ├── config.rs         lecture des variables d'environnement
  ├── dictionnaire.rs   chargement + filtrage de la liste de mots, tirage aléatoire
  ├── etat.rs           AppState (partagé) + Partie (une partie en cours)
  │     └── jeu.rs      comparer() : règle de comparaison essai / secret
  ├── routes.rs         routes HTTP + formats JSON requête/réponse
  └── erreur.rs         ApiError → réponse HTTP JSON { "erreur": "..." }
data/mots.csv           ~29 700 lignes brutes, filtrées au chargement (embarquées via include_str!)
```

Flux d'une partie :
1. `POST /nouvelle-partie` → `routes::nouvelle_partie` tire un mot
   (`dictionnaire.mot_aleatoire`), crée une `Partie`, la range dans
   `AppState.parties` sous un `Uuid`, renvoie longueur + première lettre.
2. `POST /essai` → `routes::essai` valide le mot (dictionnaire, longueur, partie
   non terminée), appelle `Partie::jouer` qui délègue à `jeu::comparer`, met à
   jour essais/victoire, renvoie le statut de chaque lettre.
3. Une tâche de fond (`boucle_purge`) supprime toutes les 5 min les parties
   terminées ou expirées (TTL 1 h).

## Fichiers

### `src/main.rs`
Point d'entrée et câblage.

- Déclare les modules (`config`, `dictionnaire`, `erreur`, `etat`, `jeu`,
  `routes`).
- Initialise `tracing_subscriber` (niveau via `RUST_LOG`, défaut `info`).
- Charge la config (`Config::depuis_env`) puis le dictionnaire ; `assert!` si le
  dictionnaire est vide.
- Construit l'`AppState` dans un `Arc`, lance `boucle_purge` en tâche Tokio.
- Configure le CORS (origine = `config.origine_front`, méthodes `GET`/`POST`,
  header `Content-Type`) et les couches `TraceLayer` (logs HTTP).
- Écoute sur `0.0.0.0:<port>`, sert l'app avec **arrêt gracieux** sur `Ctrl-C`.
- `boucle_purge` : `interval` de 5 min (`INTERVALLE_PURGE`) qui appelle
  `state.purger()`.

### `src/config.rs`
Configuration lue depuis l'environnement.

- `struct Config { port: u16, origine_front: String }`.
- `Config::depuis_env()` : `BACK_PORT` (défaut `3000`), `FRONT_ORIGIN` (défaut
  `http://localhost:5173`). Retombe sur les valeurs de dev si absentes ou
  invalides.

### `src/dictionnaire.rs`
Chargement et validation de la liste de mots jouables.

- `struct Dictionnaire { mots: Vec<String>, index: HashSet<String> }` — le `Vec`
  trié pour le tirage, le `HashSet` pour valider un essai en O(1).
- `charger()` : embarque `back/data/mots.csv` dans le binaire via
  `include_str!`, puis `depuis_texte`.
- `depuis_texte()` : passe chaque ligne dans `normaliser`, déduplique, trie.
- `contient(mot)` : le mot (déjà en minuscules) est-il jouable ?
- `mot_aleatoire()` : tire un index au hasard (`rand::rng()`).
- `len` / `est_vide`.
- `normaliser(ligne)` (privé) : retire les guillemets, rejette ce qui n'est pas
  jouable — vide, présence d'une majuscule (acronymes, noms propres), caractères
  non alphabétiques (chiffres, espaces, ponctuation), longueur hors
  `6..=9` (`LONGUEUR_MIN`/`LONGUEUR_MAX`) ; conserve les accents ; renvoie le mot
  en minuscules.
- Tests unitaires : normalisation, accents, rejets, et sanity check du
  dictionnaire embarqué.

### `src/etat.rs`
État partagé du serveur et modèle d'une partie.

- `ESSAIS_MAX = 6`, `TTL_PARTIE = 1 h`.
- `struct AppState { dictionnaire, parties: Mutex<HashMap<Uuid, Partie>> }` —
  partagé entre handlers via `Arc`.
  - `AppState::new(dictionnaire)`.
  - `purger()` : retire les parties `termine` ou dont `creee_a.elapsed() >= TTL`,
    renvoie le nombre supprimé.
- `struct Partie { secret: Vec<char>, essais_restants, termine, gagne, creee_a }`.
  - `nouvelle(secret)` : 6 essais, horodatée `Instant::now()`.
  - `longueur()`, `premiere_lettre()`, `secret_texte()`.
  - `jouer(essai)` : appelle `jeu::comparer`, décrémente `essais_restants`
    (`saturating_sub`), positionne `gagne` (tout `Correct`) et `termine`
    (`gagne` ou plus d'essais), renvoie le `Vec<Statut>`.

### `src/jeu.rs`
La règle du jeu, isolée et testée.

- `enum Statut { Correct, Present, Absent }` — sérialisé en minuscules
  (`"correct"`, `"present"`, `"absent"`).
- `comparer(essai, secret) -> Vec<Statut>` : algorithme Wordle/SUTOM en deux
  passes.
  1. Marque les lettres bien placées (`Correct`) ; compte les occurrences
     restantes des autres lettres du secret.
  2. Pour les positions non résolues, attribue `Present` tant qu'il reste des
     occurrences disponibles de cette lettre, sinon `Absent`.
  - Gère correctement les lettres en double.
- Tests : mot exact, aucune lettre commune, anagramme, lettres en double,
  priorité aux lettres bien placées.

### `src/routes.rs`
Routes HTTP et contrats JSON.

- `router(state)` : `GET /` (ping texte), `POST /nouvelle-partie`,
  `POST /essai`.
- `nouvelle_partie` (handler) : tire un mot, crée la `Partie`, l'insère sous un
  `Uuid::new_v4()`, renvoie `NouvellePartieResponse { id_partie, longueur,
  premiere_lettre, essais_restants }`. Log `debug` avec le mot secret.
- `essai` (handler) :
  - `EssaiRequest { id_partie: Uuid, mot: String }` — `mot` est `trim` +
    minuscules.
  - Valide **hors du verrou** que le mot est dans le dictionnaire
    (`ApiError::MotInconnu`).
  - Prend le `Mutex`, récupère la partie (`ApiError::PartieIntrouvable`),
    vérifie qu'elle n'est pas terminée (`ApiError::PartieTerminee`) et que la
    longueur correspond (`ApiError::MauvaiseLongueur`).
  - Appelle `partie.jouer`, renseigne `mot_secret` **uniquement si la partie est
    perdue**.
  - `EssaiResponse { resultat, essais_restants, termine, gagne, mot_secret? }`
    (`mot_secret` omis du JSON quand `None`).

### `src/erreur.rs`
Erreurs de l'API.

- `enum ApiError { PartieIntrouvable, PartieTerminee, MotInconnu,
  MauvaiseLongueur { attendue } }`.
- `impl IntoResponse` : mappe chaque variante vers un code HTTP + message —
  `404` introuvable, `409` terminée, `422` mot inconnu / mauvaise longueur — et
  renvoie `{ "erreur": "<message>" }`.

### `data/mots.csv`
Liste brute (~29 700 lignes, une par mot, entre guillemets). Non nettoyée :
`dictionnaire::normaliser` fait le tri au chargement. Embarquée dans le binaire,
donc pas de fichier à déployer.

## Fichiers de config modifiés

- **`back/Cargo.toml`** : ajout de `rand` (tirage aléatoire), `tracing` +
  `tracing-subscriber` (logs) ; features `tokio` explicites (`macros`,
  `rt-multi-thread`, `net`, `time`, `signal`) au lieu de `full` ; feature
  `trace` sur `tower-http` ; versions assouplies (`"1"` au lieu de `"1.x.y"`) ;
  suppression de la dépendance `http` directe.
- **`back/Cargo.lock`** : désormais versionné (le crate produit un binaire).
- **`.gitignore`** : `back/Cargo.lock` n'est plus ignoré.
