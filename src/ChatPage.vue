<script setup lang="ts">
import { useRoute } from 'vue-router'
const route = useRoute()
const chatId = route.params.id
</script>

<template>
  <div class="chat">
    <h1>Chat {{ chatId }}</h1>
    <p>This is the chat page</p>
  </div>
</template>

<style scoped>
.chat {
  padding: 20px;
}
</style>