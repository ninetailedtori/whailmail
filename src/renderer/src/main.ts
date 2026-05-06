import "./assets/main.css";
import { createApp } from "vue";

// @ts-expect-error: it's OK
import App from "./App.vue";
import { router } from "./router";

createApp(App).use(router).mount("#app");
