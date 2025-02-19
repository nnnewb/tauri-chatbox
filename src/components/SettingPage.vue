<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import MyButton from "./MyButton.vue";

const apiServiceType = ref('ollama');
const apiServiceUrl = ref('');
const modelName = ref('');

// 新增：加载配置
const loadConfig = async () => {
  try {
    const apiServiceTypeValue = await invoke('get_config', { key: 'apiServiceType' });
    const apiServiceUrlValue = await invoke('get_config', { key: 'apiServiceUrl' });
    const modelNameValue = await invoke('get_config', { key: 'modelName' });

    if (apiServiceTypeValue) apiServiceType.value = apiServiceTypeValue as string;
    if (apiServiceUrlValue) apiServiceUrl.value = apiServiceUrlValue as string;
    if (modelNameValue) modelName.value = modelNameValue as string;
  } catch (error) {
    console.error('Failed to load config:', error);
  }
};

// 新增：保存配置
const saveConfig = async () => {
  try {
    await invoke('set_config', { key: 'apiServiceType', value: apiServiceType.value });
    await invoke('set_config', { key: 'apiServiceUrl', value: apiServiceUrl.value });
    await invoke('set_config', { key: 'modelName', value: modelName.value });
    console.log('Config saved successfully');
  } catch (error) {
    console.error('Failed to save config:', error);
  }
};

// 新增：删除配置
const deleteConfig = async (key: string) => {
  try {
    await invoke('delete_config', { key });
    console.log(`Config with key ${key} deleted successfully`);
  } catch (error) {
    console.error(`Failed to delete config with key ${key}:`, error);
  }
};

onMounted(() => {
  loadConfig();
});
</script>

<template>
  <div class="settings">
    <h1>Settings</h1>
    <p>This is the settings page</p>
    <section class="llm-api-section">
      <h2>LLM API 服务</h2>
      <form class="llm-api-form" @submit.prevent="saveConfig">
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
        <div class="form-group">
          <MyButton type="primary" @click="saveConfig">保存配置</MyButton>
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

.form-group button {
  padding: 8px 16px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.form-group button:hover {
  background-color: #0056b3;
}
</style>