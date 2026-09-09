<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'

import GameRow from './GameRow.vue'
import { useGame } from '@/composables/useGame'

const {
    phase,
    grille,
    erreur,
    essaisRestants,
    motSecret,
    tentatives,
    terminee,
    demarrer,
    taper,
    effacer,
    valider,
} = useGame()

function onKeydown(e: KeyboardEvent) {
    if (e.metaKey || e.ctrlKey || e.altKey) return

    if (e.key === 'Enter') {
    e.preventDefault()
    void valider()
    } else if (e.key === 'Backspace') {
    e.preventDefault()
    effacer()
    } else if (e.key.length === 1) {
    taper(e.key)
    }
}

onMounted(() => window.addEventListener('keydown', onKeydown))
onUnmounted(() => window.removeEventListener('keydown', onKeydown))

defineExpose({ valider, demarrer })
</script>

<template>
    <div class="flex flex-col items-center gap-2">
    <div class="flex flex-col gap-1.5">
        <GameRow
        v-for="(ligne, i) in grille"
        :key="i"
        :lettres="ligne.lettres"
        :statuts="ligne.statuts"
        :active="ligne.active"
        />
    </div>

    <p class="h-6 text-sm text-center">
        <span v-if="erreur" class="font-medium text-red-600">{{ erreur }}</span>
            <span v-else-if="phase === 'gagne'" class="font-semibold text-green-600">
                Gagné en {{ tentatives.length }} essai(s) !
            </span>
            <span v-else-if="phase === 'perdu'" class="font-medium text-slate-700">
                Perdu — le mot était « {{ motSecret }} ».
            </span>
        <span v-else class="text-slate-500">{{ essaisRestants }} essai(s) restant(s)</span>
    </p>

    <button
        v-if="terminee || phase === 'erreur'"
        type="button"
        class="rounded bg-slate-800 px-4 py-2 text-sm font-medium text-white hover:bg-slate-700"
        @click="demarrer()">Nouvelle partie
    </button>
    </div>
</template>