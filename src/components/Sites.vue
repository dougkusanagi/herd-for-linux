<script lang="ts" setup>
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const sites = ref([]);
const php_global_version = ref("");

const site_urls = computed(() => {
  return sites.value.map((site: string) => {
    const tokens = site.split("/");
    return `http://${tokens[tokens.length - 1]}.test`;
  });
});

async function fetch_php_global_version() {
  php_global_version.value = (await invoke("get_php_global_versions"))[0].version;
}

async function main() {
  const paths: string = await invoke("get_links");
  const path_list = JSON.parse(paths);
  sites.value = path_list;

  await fetch_php_global_version();
}

main();
</script>

<template>
  <h2 class="px-8 py-6 text-2xl text-white border-b border-white/10">
    Sites
  </h2>

  <div class="px-8 mt-8">
    <button
      class="inline-flex items-center px-4 py-2 text-sm bg-indigo-700 rounded transition-all duration-200 text-white/80 hover:bg-indigo-600 hover:text-white"
      @click="main">
      <svg class="mr-2 size-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
        stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round"
          d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99" />
      </svg>

      Reload Sites
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
          <td class="px-4 py-3 border-b border-white/10">
            <a :href="site_urls[index]" target="_blank">
              {{ site_urls[index] }}
            </a>

            <br />

            <span class="text-xs font-normal text-white/60">
              {{ site }}
            </span>
          </td>

          <td class="px-4 py-3 border-b border-white/10">
            <span class="text-sm font-normal">
              {{ php_global_version }}
            </span>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
