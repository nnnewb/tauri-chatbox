import { createApp } from "vue";
import App from "./App.vue";
import "./normalize.css";
import router from "./routes";
import { createPinia } from "pinia";

const pinia = createPinia();
const app = createApp(App);
app.use(pinia);
app.use(router);
app.mount("#app");
