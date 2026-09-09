<script setup lang="ts">
const emit = defineEmits<{ (e: 'click'): void }>()
</script>

<template>
    <button
        class="bg-blue-500 hover:bg-blue-700 text-white w-24 h-10 text-base font-bold py-2 px-2 rounded"
        @click="emit('click')">submit
    </button>
</template>