<script setup>
import { defineProps } from "vue";

// 接受 active prop 和 to prop
defineProps({
  active: {
    type: Boolean,
    default: false,
  },
  to: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <router-link :to="to" class="nav-button" :class="{ active: active }">
    <slot>Default Button</slot>
    <!-- 使用默认插槽 -->
  </router-link>
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