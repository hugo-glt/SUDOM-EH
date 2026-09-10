<script setup lang="ts">
import { computed } from 'vue'
import type { LigneGrille } from '@/types/game'

const props = defineProps<{
    longueurMot: number
    grille: LigneGrille[]
}>()

const styleGrille = computed(() => ({
    gridTemplateColumns: `repeat(${props.longueurMot}, minmax(0, 1fr))`,
}))

const classesStatut: Record<string, string> = {
    correct: 'bg-green-500 border-green-500 text-white',
    present: 'bg-yellow-500 border-yellow-500 text-white',
    absent: 'bg-neutral-600 border-neutral-600 text-white',
    vide: 'bg-transparent border-neutral-400 text-neutral-100',
}

function classePourCase(ligne: LigneGrille, index: number): string {
    const statut = ligne.statuts?.[index] ?? 'vide'
    return classesStatut[statut]
}
</script>

<template>
    <div class="flex flex-col gap-1.5 mx-auto w-fit" role="grid" aria-label="Grille du jeu">
        <div
            v-for="(ligne, i) in grille"
            :key="i"
            class="grid gap-1.5"
            :style="styleGrille"
            role="row">
        <div
        v-for="(lettre, j) in ligne.lettres"
        :key="j"
        class="aspect-square w-10 sm:w-12 flex items-center justify-center border-2 rounded font-bold uppercase text-lg sm:text-xl transition-colors duration-200"
        :class="classePourCase(ligne, j)"
        role="gridcell">
        {{ lettre }}
        </div>
    </div>
    </div>
</template>