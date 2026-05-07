// SPDX-FileCopyrightText: 2026 2026-Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

import { createRouter, createWebHistory } from "vue-router";
import { routes, handleHotUpdate } from "vue-router/auto-routes";

export const router = createRouter({
  history: createWebHistory(),
  routes,
});

// This will update routes at runtime without reloading the page!!
if (import.meta.hot) handleHotUpdate(router);
