<script setup lang="ts">
import GameTile from './GameTile.vue'
import type { Statut } from '@/types/game'

defineProps<{
  lettres: string[]
  statuts: Statut[] | null
  active?: boolean
}>()
</script>

<template>
  <div class="flex gap-1.5">
    <GameTile
      v-for="(lettre, i) in lettres"
      :key="i"
      :lettre="lettre"
      :statut="statuts?.[i] ?? null"
      :active="active"
    />
  </div>
</template>
