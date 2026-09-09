<script setup lang="ts">

</script>

<template>
    <section>
        <div class="flex justify-center p-2">
            <div class="grid grid-cols-8 border-2 rounded-2xl border-black">
                <div v-for="i in 40" :key="i"
                    class="w-12 h-12 border rounded-xl border-black flex items-center justify-center text-lg font-bold">
                </div>
            </div>
        </div>
    </section>
</template>