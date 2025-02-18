<script setup lang="ts">
import { useRoute, onBeforeRouteUpdate } from 'vue-router'
import { ref } from 'vue'

const route = useRoute()
const chatId = ref(route.params.id)

onBeforeRouteUpdate((to) => {
  chatId.value = to.params.id
})
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