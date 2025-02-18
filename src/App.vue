<script setup lang="ts">
import { ref, provide } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { RouterView } from "vue-router";
import AsideNav from "./AsideNav.vue";

const greetMsg = ref("");
const name = ref("");
const chat = ref([]);

provide("chat", chat);

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}

</script>

<template>
  <div class="app">
    <aside-nav />
    <main class="main">
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.app {
  display: flex;
  height: 100vh;
}

.main {
  flex: 1;
  padding: 20px;
}
</style>
