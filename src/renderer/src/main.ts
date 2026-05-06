import "./assets/main.scss";
import { createApp } from "vue";

// @ts-expect-error - It's OK
import App from "./App.vue";
import { router } from "./router";

createApp(App).use(router).mount("#app");
