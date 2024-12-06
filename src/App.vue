<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const sites = ref([]);

async function greet() {
  const paths: string = await invoke("get_links");
  const path_list = JSON.parse(paths);
  sites.value = path_list;
}
</script>

<template>
  <div class="flex h-screen bg-slate-900">
    <nav class="w-80 border-r bg-black/10 border-white/60">
      <h1 class="px-4 py-8 text-2xl text-white border-b border-white/60">Herd For Linux</h1>

      <div class="flex flex-col gap-2 mt-8 text-white/60">
        <a class="px-6 py-2 mx-2 text-sm text-white rounded border bg-white/5 border-white/10" href="#">Sites</a>
        <a class="px-6 py-2 mx-2 text-sm rounded transition-all duration-200 hover:bg-white/5 hover:text-white"
          href="#">PHP</a>
      </div>
    </nav>

    <main class="flex-1">
      <ul class="text-white">
        <li v-for="site in sites" :key="site">{{ site }}</li>
      </ul>
    </main>
  </div>
</template>
