<script setup lang="ts">
import { ref } from 'vue';

const apiServiceType = ref('ollama');
const apiServiceUrl = ref('');
const modelName = ref('');
</script>

<template>
  <div class="settings">
    <h1>Settings</h1>
    <p>This is the settings page</p>
    <section class="llm-api-section">
      <h2>LLM API 服务</h2>
      <form class="llm-api-form">
        <div class="form-group">
          <label for="api-service-type">API 服务类型</label>
          <select id="api-service-type" v-model="apiServiceType">
            <option value="ollama">ollama</option>
          </select>
        </div>
        <div class="form-group">
          <label for="api-service-url">API 服务地址</label>
          <input id="api-service-url" type="text" v-model="apiServiceUrl" placeholder="请输入API服务地址" />
        </div>
        <div class="form-group">
          <label for="model-name">模型名称</label>
          <input id="model-name" type="text" v-model="modelName" placeholder="请输入模型名称" />
        </div>
      </form>
    </section>
  </div>
</template>

<style scoped>
.settings {
  padding: 20px;
}

.llm-api-section {
  margin-top: 20px;
}

.llm-api-form {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.form-group {
  display: flex;
  align-items: center;
}

.form-group label {
  width: 120px;
  margin-right: 10px;
}

.form-group input,
.form-group select {
  flex: 1;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
}
</style>