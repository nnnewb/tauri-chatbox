<script setup lang="ts">
import { inject, ref } from "vue";
import NavButton from "./components/NavButton.vue";

const chat = inject("chat", ref<string[]>([]));

const addChat = () => {
    console.log(chat);
  const newChatId = `chat_${chat.value.length + 1}`;
  chat.value.push(newChatId);
};
</script>

<template>
  <aside>
    <div class="topdown">
      <nav-button @click="addChat">添加</nav-button>
      <nav-button v-for="(chatId, index) in chat" :key="index" :to="`/chat/${chatId}`">
        会话 {{ index + 1 }}
      </nav-button>
    </div>
    <div class="bottomup">
      <nav-button to="/settings">设置</nav-button>
    </div>
  </aside>
</template>

<style scoped>
aside {
  width: 250px;
  background-color: #f5f5f5;
  display: flex;
  flex-direction: column;
}

.topdown {
  flex: 1;
  overflow-y: auto;
}

.setting-btn {
  width: 100%;
  height: 40px;
  background-color: #f5f5f5;
}
</style>