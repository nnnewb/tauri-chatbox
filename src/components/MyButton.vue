<template>
  <button :class="['button', type]" @click="$emit('click')">
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
defineProps({
  type: {
    type: String,
    default: 'primary',
    validator: (value: string) => ['primary', 'info', 'danger'].includes(value)
  }
});
</script>

<style scoped>
.button {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.button:active {
  transform: translateY(2px);
}

.button.primary {
  background-color: #007bff;
  color: white;
}

.button.primary:hover {
  background-color: #0056b3;
}

.button.primary:active {
  background-color: #004085;
}

.button.info {
  background-color: #17a2b8;
  color: white;
}

.button.info:hover {
  background-color: #138496;
}

.button.info:active {
  background-color: #117a8b;
}

.button.danger {
  background-color: #dc3545;
  color: white;
}

.button.danger:hover {
  background-color: #c82333;
}

.button.danger:active {
  background-color: #bd2130;
}
</style>