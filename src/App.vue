<script setup lang="ts">
import { ref } from "vue";
import Sites from "./components/Sites.vue";
import PHP from "./components/PHP.vue";
// import SideNav from "./components/SideNav.vue";

const selected_tab = ref(0);

const tabs = [Sites, PHP];
</script>

<template>
  <div class="flex h-screen bg-slate-900">
    <!-- <SideNav :selected_tab="selected_tab" @update:selected_tab="selected_tab = $event" /> -->

    <main class="flex-1">
      <component :is="tabs[selected_tab]"></component>
    </main>
  </div>
</template>
