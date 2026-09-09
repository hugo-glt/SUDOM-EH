<script setup lang="ts">
import GameGrid from '../components/game/GameGrid.vue'
import Button from '../ui/Button.vue'

</script>

<template>
    <section class="flex flex-col items-center gap-2">
        <div>
            <GameGrid />
        </div>
    </section>

    <section class="flex items-center justify-center text-4xl font-bold mt-5">
        <Button />
    </section>
</template>