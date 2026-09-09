<script setup lang="ts">
import { computed } from 'vue'

import type { Statut } from '@/types/game'

const props = defineProps<{
  lettre: string
  statut: Statut | null
  active?: boolean
}>()

const classes = computed(() => {
  switch (props.statut) {
    case 'correct':
      return 'bg-red-600 border-red-600 text-white'
    case 'present':
      return 'bg-amber-400 border-amber-400 text-neutral-900'
    case 'absent':
      return 'bg-slate-600 border-slate-600 text-white'
    default:
      return props.lettre
        ? 'bg-white border-slate-400 text-slate-900'
        : 'bg-white border-slate-200 text-slate-900'
  }
})
</script>

<template>
  <div
    class="flex h-12 w-12 items-center justify-center rounded border-2 text-2xl font-bold uppercase transition-colors sm:h-14 sm:w-14"
    :class="[classes, active ? 'ring-2 ring-slate-400' : '']"
  >
    {{ lettre }}
  </div>
</template>
