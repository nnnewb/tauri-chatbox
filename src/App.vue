<script setup lang="ts">
import { RouterView } from "vue-router";
import AsideNav from "./components/AsideNav.vue";
</script>

<template>
  <div class="app">
    <aside-nav />
    <router-view class="main" />
  </div>
</template>

<style scoped>
.app {
  display: flex;
  height: 100vh;
}

.main {
  flex: 1;
}
</style>
