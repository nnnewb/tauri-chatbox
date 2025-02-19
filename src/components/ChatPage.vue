<script setup lang="ts">
import { useRoute, onBeforeRouteUpdate } from "vue-router";
import { ref, onMounted } from "vue";
import { Session, SessionRepository } from "../lib/session";
import MyButton from "./MyButton.vue";

const route = useRoute();
const chatId = ref(route.params.id);
const session = ref<Session | null>(null);
const messages = ref<Message[]>([]);
const messageInput = ref(""); // 用于存储输入框的内容

const sessionRepository = SessionRepository.getInstance();

onMounted(async () => {
  await fetchSession();
});

onBeforeRouteUpdate(async (to) => {
  chatId.value = to.params.id;
  await fetchSession();
});

async function fetchSession() {
  if (chatId.value) {
    const sessionId = parseInt(chatId.value as string, 10);
    if (isNaN(sessionId)) {
      console.log(`Invalid session ID: ${chatId.value}`);
      return;
    }
    session.value = await sessionRepository.get_session(sessionId);
    if (session.value) {
      console.log(session.value);
      messages.value = await session.value.list_messages();
    }
  }
}

async function sendMessage() {
  if (session.value && messageInput.value.trim()) {
    const newMessage = await session.value.add_message("self", messageInput.value.trim());
    messages.value.push(newMessage);
    messageInput.value = ""; // 清空输入框
  }
}

async function clearMessages() {
  if (session.value) {
    await session.value.clear_messages();
    messages.value = [];
  }
}

function handleKeyPress(event: KeyboardEvent) {
  if (event.key === "Enter" && !event.shiftKey) {
    event.preventDefault(); // 阻止默认的换行行为
    sendMessage();
  }
}
</script>

<template>
  <div class="chat">
    <h1>{{ session?.name }}</h1>
    <div class="chat-content">
      <div class="message-container">
        <div
          v-for="message in messages"
          :key="message.id"
          :class="['message', message.role === 'self' ? 'self' : 'other']"
        >
          {{ message.text }}
        </div>
      </div>
    </div>
    <div class="chat-input-container">
      <textarea
        class="chat-input"
        placeholder="Type your message here..."
        v-model="messageInput"
        @keypress="handleKeyPress"
      ></textarea>
    </div>
    <div class="chat-action">
      <my-button type="danger" @click="clearMessages">Clear Messages</my-button>
      <my-button type="primary" @click="sendMessage">Send</my-button>
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
  scrollbar-width: thin; /* 设置滚动条宽度 */
  scrollbar-color: #ccc transparent; /* 设置滚动条颜色，背景透明 */
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
  gap: 12px;
}

/* 消息气泡样式 */
.message-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.message {
  max-width: 70%;
  padding: 10px;
  border-radius: 10px;
  word-wrap: break-word;
}

.message.other {
  background-color: #f1f1f1;
  align-self: flex-start;
}

.message.self {
  background-color: #007bff;
  color: white;
  align-self: flex-end;
}
</style>