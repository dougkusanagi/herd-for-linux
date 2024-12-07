<script setup lang="ts">
const props = defineProps({
  selected_tab: Number
})
</script>

<template>
  <nav class="w-80 border-r bg-black/10 border-white/10">
    <h1 class="px-4 py-6 text-2xl text-white border-b border-white/10">
      Herd For Linux
    </h1>

    <div class="flex flex-col gap-2 mt-8 text-white/60">
      <button
        class="px-6 py-2 mx-2 text-sm text-left rounded transition-all duration-200 hover:bg-white/5 hover:text-white">
        General
      </button>

      <button class="px-6 py-2 mx-2 text-sm text-left text-white rounded border bg-white/5 border-white/10"
        @click="$emit('update:selected_tab', 0)">
        Sites
      </button>

      <button
        class="px-6 py-2 mx-2 text-sm text-left rounded transition-all duration-200 hover:bg-white/5 hover:text-white"
        @click="$emit('update:selected_tab', 1)">
        PHP
      </button>
    </div>
  </nav>
</template>
