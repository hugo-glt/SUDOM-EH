<script setup lang="ts">
import { ref } from 'vue'
import GameGrid from '../components/game/GameGrid.vue'
import Button from '../ui/Button.vue'

const gameGrid = ref<InstanceType<typeof GameGrid> | null>(null)
</script>

<template>
    <section class="flex flex-col items-center gap-5">
        <GameGrid ref="gameGrid" />
        <Button @click="gameGrid?.valider()" />
    </section>
</template>