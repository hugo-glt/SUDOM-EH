<script setup lang="ts">
import GameGrid from '../components/game/GameGrid.vue'
import Button from '../ui/Button.vue'
import { useGame } from '@/composables/useGame'

const { longueur, grille, peutValider, envoiEnCours, erreur, valider } = useGame()
</script>

<template>
    <section class="flex flex-col items-center gap-5">
    <GameGrid :longueur-mot="longueur" :grille="grille" />

    <p v-if="erreur" class="text-red-500 text-sm" role="alert">
        {{ erreur }}
    </p>

    <Button :disabled="!peutValider || envoiEnCours" @click="valider" />
    </section>
</template>