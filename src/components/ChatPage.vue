<script setup lang="ts">
import { useRoute, onBeforeRouteUpdate } from "vue-router";
import { ref } from "vue";

const route = useRoute();
const chatId = ref(route.params.id);

onBeforeRouteUpdate((to) => {
  chatId.value = to.params.id;
});
</script>

<template>
  <div class="chat">
    <div class="chat-content">
      <h1>Chat {{ chatId }}</h1>
      <p>This is the chat page</p>
    </div>
    <div class="chat-input-container">
      <textarea class="chat-input" placeholder="Type your message here..."></textarea>
    </div>
    <div class="chat-action">
      <button class="send-button">Send</button>
    </div>
  </div>
</template>

<style scoped>
.chat {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.chat-content {
  flex: 1;
  overflow-y: auto;
  margin-bottom: 20px;
}

.chat-input-container {
  width: 100%;
}

.chat-input {
  width: 100%;
  height: 100px; /* 固定高度 */
  padding: 0;
  border: 1px solid #ccc;
  border-radius: 4px;
  resize: none;
}

.chat-action {
  display: flex;
  justify-content: flex-end;
  align-items: center;
}

.send-button {
  margin-top: 10px;
  padding: 10px 20px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.send-button:hover {
  background-color: #0056b3;
}
</style>
