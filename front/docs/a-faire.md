# Front — ce qu'il reste à faire

Guideline pour reprendre le front. La logique de jeu est écrite
([useGame.ts](../src/composables/useGame.ts), [api/](../src/api/),
[components/game/](../src/components/game/)) et documentée dans
[jeu-front.md](./jeu-front.md), mais **rien n'est branché** : l'app ne monte même
pas Vue aujourd'hui. Cette page liste les étapes, de la plus bloquante à la plus
accessoire.

Prérequis : le back doit tourner (`cd back && cargo run`, port 3000) et
`VITE_API_BASE_URL` être défini (déjà fait via `.envrc`).

---

## P0 — Rendre l'app fonctionnelle (bloquant)

Sans ça, `npm run dev` affiche une page blanche.

### 1. Monter l'application Vue dans `main.ts`

[src/main.ts](../src/main.ts) ne fait qu'exporter une fonction `estUnMotValide` —
il n'appelle jamais `createApp`. À remplacer entièrement par :

```ts
import { createApp } from 'vue'
import App from './App.vue'
import './style.css'

createApp(App).mount('#app')
```

> Au passage ça supprime l'ancien dico client (`noun.csv` + `estUnMotValide`),
> devenu inutile puisque le back valide les mots. Voir P1.4.

### 2. Brancher `GameBoard` dans `App.vue`

[src/App.vue](../src/App.vue) affiche encore `HelloWorld`. Le remplacer par le
jeu, avec un minimum de mise en page (titre, centrage, largeur max) :

```vue
<script setup lang="ts">
import GameBoard from '@/components/game/GameBoard.vue'
</script>

<template>
  <main class="mx-auto flex min-h-screen max-w-md flex-col items-center gap-8 px-4 py-10">
    <h1 class="text-3xl font-bold tracking-wide text-slate-800">SUDOM</h1>
    <GameBoard />
  </main>
</template>
```

### 3. Vérifier que Tailwind se charge

`style.css` contient juste `@import "tailwindcss"`. Il faut qu'il soit importé
par `main.ts` (fait en P0.1). Lancer `npm run dev` et confirmer que les classes
(`bg-red-600`, `flex`, …) s'appliquent.

### 4. Test de bout en bout manuel

- La grille s'affiche, une partie démarre seule.
- Taper un mot au clavier, `Entrée` → la ligne se colore.
- Mot trop court : `Entrée` ne fait rien (`peutValider` false).
- Mot inconnu : message d'erreur rouge, la saisie reste.
- Gagner / perdre → message + bouton « Rejouer ».
- Couper le back → message « Serveur injoignable ».

---

## P1 — Intégration propre & nettoyage

### 1. Supprimer le template de démarrage Vite

- [src/components/HelloWorld.vue](../src/components/HelloWorld.vue)
- `src/assets/hero.png`, `src/assets/vue.svg`, `src/assets/vite.svg` (si plus
  référencés)
- vérifier qu'aucun `import` ne pointe encore dessus

### 2. `index.html`

- `<title>front</title>` → `SUDOM` (ou le nom retenu)
- `<html lang="en">` → `lang="fr"`
- vérifier le favicon (`/favicon.svg` doit exister dans `public/`)

### 3. Cohérence de l'alias `@`

L'alias `@ → src/` est défini dans `vite.config.ts`, `tsconfig.json` et
`tsconfig.app.json`. OK, mais s'assurer que `vue-tsc` (via `npm run build`) ne
râle pas.

### 4. Retirer le dictionnaire côté client

`src/data/noun.csv` et toute référence à `estUnMotValide` : le back est seul
juge des mots valides. Ne pas réintroduire de validation de dictionnaire dans le
front (au mieux dupliqué, au pire faux).

### 5. `npm run build` doit passer

`vue-tsc -b && vite build` sans erreur de type. C'est le garde-fou CI.

---

## P2 — UX & accessibilité

### 1. Indiquer l'envoi en cours

`useGame` expose `envoiEnCours` mais `GameBoard` ne l'utilise pas. Pendant la
requête `/essai`, désactiver visuellement la grille active / afficher un état
d'attente pour éviter le double `Entrée`.

### 2. Clavier à l'écran (mobile)

Aujourd'hui le jeu n'écoute que le **clavier physique** (`keydown` sur `window`).
Injouable sur téléphone. Décider :
- soit un composant clavier AZERTY cliquable qui appelle `taper` / `effacer` /
  `valider` (les 3 actions sont déjà exposées par `useGame`) ;
- soit un `<input>` masqué qui capte la saisie tactile.
Idéalement colorer les touches déjà jouées (correct / présent / absent) —
demande d'agréger les statuts par lettre dans `useGame`.

### 3. Lettres accentuées au clavier physique

Les touches mortes (`^`, `¨`) produisent `e.key === 'Dead'` et n'insèrent rien —
gênant pour les mots avec accent circonflexe. Le back garde les accents dans le
dictionnaire. À tester et, si besoin, gérer `compositionend` ou normaliser.

### 4. Accessibilité

- annoncer le résultat d'un essai et la fin de partie via `aria-live` ;
- `role`/`aria-label` sur les tuiles (« lettre M, bien placée ») ;
- vérifier le contraste des couleurs (ambre sur texte foncé surtout) ;
- focus visible / point d'entrée clavier clair.

### 5. États limites d'affichage

- première lettre imposée : bien visible qu'elle est « offerte » et non
  effaçable (elle l'est déjà via `effacer`, mais rien ne le signale) ;
- longueur variable (6 à 9) : vérifier que la grille reste centrée et lisible
  sur mobile pour 9 lettres.

---

## P3 — Confort (plus tard)

- petites animations (retournement des tuiles, secousse sur mot invalide) ;
- écran de fin plus travaillé + bouton « Partager » (résultat en carrés) ;
- compteur de parties / stats en `localStorage` ;
- thème sombre ;
- gérer le rechargement de page en cours de partie : l'`id_partie` est perdu
  (état seulement en mémoire front). Soit l'accepter, soit persister
  `id_partie` en `localStorage` et prévoir une route back de reprise (n'existe
  pas aujourd'hui → discussion avec l'équipe back).

---

## Points d'attention transverses

- **Ne pas remettre de logique de jeu dans le front.** Validité du mot,
  longueur, comptage des essais, victoire : c'est le back. Le front collecte et
  affiche.
- **Le mot secret** n'arrive que dans `mot_secret` en cas de défaite. Ne jamais
  le logguer ni l'exposer autrement.
- **Erreurs** : `ApiError.statut` vaut `0` si le serveur est injoignable, sinon
  le code HTTP (422 = mot inconnu / mauvaise longueur, 404 = partie inconnue,
  409 = partie finie). Les messages sont déjà lisibles côté API.
- **Build de prod** : `VITE_API_BASE_URL` doit pointer vers l'URL publique du
  back, et cette origine doit être autorisée côté back (`FRONT_ORIGIN`).
- **Branche** : tout ça vit sur `feat/front-jeu-etat`. Rebaser sur `main` à jour
  avant d'ouvrir la PR.
