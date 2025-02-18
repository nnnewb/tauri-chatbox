<script setup lang="ts">
import { defineProps } from "vue";

// 接受 active prop 和 to prop
defineProps({
  active: {
    type: Boolean,
    default: false,
  },
  to: {
    type: String,
    default: null,
  },
});
</script>

<template>
  <router-link v-if="to" :to="to" class="nav-button" :class="{ active: active }">
    <slot>Default Button</slot>
  </router-link>
  <button v-else class="nav-button" :class="{ active: active }">
    <slot>Default Button</slot>
  </button>
</template>

<style scoped>
.nav-button {
  width: 100%;
  height: 40px; /* 固定高度 */
  background-color: #f0f0f0; /* 未激活状态背景色 */
  border: none;
  cursor: pointer;
  transition: background-color 0.3s ease; /* 添加过渡效果 */
  display: flex;
  align-items: center;
  justify-content: center;
  text-decoration: none;
  color: inherit;
}

.nav-button:hover {
  background-color: #d0d0d0; /* hover状态背景色 */
}

.nav-button.active {
  background-color: #a0a0a0; /* 激活状态背景色 */
}
</style>