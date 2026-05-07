import "./assets/main.css";
import { createApp } from "vue";
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';
import Tooltip from 'primevue/tooltip';

import App from "./App.vue";
import { router } from "./router";


createApp(App)
  .use(router)
  .use(PrimeVue, {
    theme: {
      preset: Aura
    },
  })
  .directive('tooltip', Tooltip)
  .mount("#app");
