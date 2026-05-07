/*
 * SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
 * SPDX-FileContributor: WhailMail contributors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import ui from "@nuxt/ui/vue-plugin";
import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";

import "@assets/main.css";
import App from "./App.vue";
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';
import { Tooltip } from "primevue";
import '@primeuix/themes/aura';

const app = createApp(App);

const router = createRouter({
  routes: [],
  history: createWebHistory(),
});

app.use(router)
  .use(ui)
  .use(PrimeVue, {
    theme: {
      preset: Aura
    },
  })
  .directive('tooltip', Tooltip)
  .mount("#app");
