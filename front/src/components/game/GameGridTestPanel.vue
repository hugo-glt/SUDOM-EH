<script setup lang="ts">
import { ref, computed } from 'vue'
import GameGrid from './GameGrid.vue'
import type { LigneGrille, Statut } from '@/types/game'

const longueurTest = ref(5)
const maxTentativesTest = ref(6)

const statutsCycle: Statut[] = ['absent', 'present', 'correct']

// Génère une grille factice pour prévisualiser le rendu
const grilleTest = ref<LigneGrille[]>([])

function genererLigneAleatoire(): LigneGrille {
    return {
        lettres: Array.from({ length: longueurTest.value }, () =>
            String.fromCharCode(65 + Math.floor(Math.random() * 26)),
        ),
        statuts: Array.from({ length: longueurTest.value }, () =>
            statutsCycle[Math.floor(Math.random() * statutsCycle.length)],
        ),
        active: false,
    }
}

function ajouterLigneTest() {
    if (grilleTest.value.length < maxTentativesTest.value) {
    grilleTest.value.push(genererLigneAleatoire())
    }
}

function reinitialiser() {
    grilleTest.value = []
}

const peutAjouter = computed(() => grilleTest.value.length < maxTentativesTest.value)
</script>

<template>
    <div class="border border-dashed border-neutral-400 rounded-lg p-4 my-6 space-y-4">
    <p class="text-sm font-semibold text-neutral-500 uppercase tracking-wide">
        Mode test — prévisualisation front (sans backend)
    </p>

    <div class="flex flex-wrap gap-4 items-end">
        <label class="flex flex-col text-sm gap-1">
        Longueur du mot
        <input
            type="number"
            min="3"
            max="12"
            v-model.number="longueurTest"
            class="border rounded px-2 py-1 w-20"
        />
        </label>

        <label class="flex flex-col text-sm gap-1">
        Nb de tentatives max
        <input
            type="number"
            min="1"
            max="10"
            v-model.number="maxTentativesTest"
            class="border rounded px-2 py-1 w-20"
        />
        </label>

        <button
        type="button"
        class="px-3 py-1.5 rounded bg-blue-600 text-white disabled:opacity-40"
        :disabled="!peutAjouter"
        @click="ajouterLigneTest">
        Ajouter une ligne aléatoire
        </button>

        <button
        type="button"
        class="px-3 py-1.5 rounded border"
        @click="reinitialiser">
        Réinitialiser
        </button>
    </div>

    <GameGrid
        :longueur-mot="longueurTest"
        :max-tentatives="maxTentativesTest"
        :grille="grilleTest"
        tentative-actuelle=""
    />
    </div>
</template>