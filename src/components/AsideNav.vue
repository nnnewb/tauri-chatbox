<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import NavButton from "./NavButton.vue";
import { SessionRepository } from "../lib/session";

const route = useRoute();
const router = useRouter();
const sessions = ref<Session[]>([]);

const sessionRepository = SessionRepository.getInstance();

onMounted(async () => {
  await fetchSessions();
});

async function fetchSessions() {
  sessions.value = await sessionRepository.get_all_sessions();
}

const addChat = async () => {
  const newSession = await sessionRepository.create_session("untitled");
  sessions.value.push(newSession);
  router.push(`/chat/${newSession.id}`);
};
</script>

<template>
  <aside>
    <div class="topdown">
      <nav-button @click="addChat">添加</nav-button>
      <nav-button
        v-for="session in sessions"
        :key="session.id"
        :to="`/chat/${session.id}`"
        :active="route.params.id === session.id.toString()"
      >
        {{ session.name }}
      </nav-button>
    </div>
    <div class="bottomup">
      <nav-button to="/settings" :active="route.path === '/settings'">设置</nav-button>
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