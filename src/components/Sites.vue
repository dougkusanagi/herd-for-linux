<script lang="ts" setup>
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const sites = ref([]);

const site_urls = computed(() => {
  return sites.value.map((site: string) => {
    const tokens = site.split("/");
    return `http://${tokens[tokens.length - 1]}.test`;
  });
});

async function main() {
  const paths: string = await invoke("get_links");
  const path_list = JSON.parse(paths);
  sites.value = path_list;
}

main();
</script>

<template>
  <h2 class="px-8 py-6 text-2xl text-white border-b border-white/10">
    Sites
  </h2>

  <div class="px-8 mt-8">
    <button
      class="px-6 py-2 text-sm bg-indigo-700 rounded transition-all duration-200 text-white/80 hover:bg-indigo-600 hover:text-white"
      @click="main">
      Load Sites
    </button>

    <table class="mt-6 w-full text-white">
      <thead>
        <tr>
          <th class="px-4 py-2 text-left border-r border-b border-white/10 bg-white/5">
            Site
          </th>
          <th class="px-4 py-2 text-left border-b border-white/10 bg-white/5">
            PHP Version
          </th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="(site, index) in sites" :key="site">
          <td class="px-4 py-4 border-b border-white/10">
            <a :href="site_urls[index]" target="_blank">
              {{ site_urls[index] }}
            </a>

            <br />

            <span class="text-xs font-normal text-white/60">
              {{ site }}
            </span>
          </td>

          <td class="px-4 py-4 border-b border-white/10">
            <span class="text-sm font-normal">PHP</span>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
