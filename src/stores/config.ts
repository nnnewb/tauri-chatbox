import { defineStore } from "pinia";
interface BackendConfig {
  host: string;
  model: string;
}

export const useConfigStore = defineStore("config", {
  state() {
    return {
      backend: {
        host: "192.168.1.5:11434",
        model: "deepseek-r1:8b",
      } as BackendConfig,
    };
  },
  getters: {
    host: (state) => state.backend.host,
    base_url: (state) => `http://${state.backend.host}`,
    model_name: (state) => state.backend.model,
  },
});
