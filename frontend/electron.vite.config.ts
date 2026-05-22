// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

import { resolve } from "path";

import ui from "@nuxt/ui/vite";
import tailwindcss from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";
import { defineConfig } from "electron-vite";
import Fonts from "unplugin-fonts/vite";
import IconsResolver from "unplugin-icons/resolver";
import Icons from "unplugin-icons/vite";
import vueDevTools from "vite-plugin-vue-devtools";
import VueRouter from "vue-router/vite";

const sharedAliases = {
  "@resources": resolve("./resources"),
  "@shared": resolve("./shared"),
};

export default defineConfig({
  main: {
    build: {
      lib: {
        entry: "main/index.ts",
      },
    },
    resolve: {
      alias: {
        ...sharedAliases,
      },
    },
  },
  preload: {
    build: {
      lib: {
        entry: "preload/index.ts",
      },
    },
    resolve: {
      alias: {
        ...sharedAliases,
      },
    },
  },
  renderer: {
    root: "renderer",
    build: {
      rollupOptions: {
        input: "renderer/index.html",
      },
    },
    resolve: {
      alias: {
        ...sharedAliases,
        "@": resolve("./renderer"),
        "@assets": resolve("./renderer/assets"),
        "@components": resolve("./renderer/components"),
        "@layouts": resolve("./renderer/layouts"),
        "@pages": resolve("./renderer/pages"),
        "@router": resolve("./renderer/router"),
        "@stores": resolve("./renderer/stores"),
      },
    },
    plugins: [
      tailwindcss(),
      VueRouter({
        dts: resolve("renderer/typed-router.d.ts"),
      }),
      vue(),
      vueDevTools(),
      ui({
        router: true,
        components: {
          resolvers: [IconsResolver()],
          dirs: ["./renderer/components"],
        },
        ui: {
          colors: {
            primary: "ctp-rosewater",
            secondary: "ctp-mauve",
            success: "ctp-green",
            info: "ctp-sky",
            warning: "ctp-peach",
            error: "ctp-red",
            neutral: "ctp-surface0",
          },
        },
        autoImport: {
          imports: ["vue", "vue-router", "pinia"],
          dirs: ["./renderer/composables", "./renderer/utils"],
          eslintrc: {
            enabled: true,
          },
        },
      }),
      Fonts({
        google: {
          families: ["Inria Sans:300,400,700", "Afacad Flux:100..1000"],
        },
        custom: {
          families: [
            {
              name: "JetBrainsMono Nerd Font",
              local: "JetBrainsMono Nerd Font",
              src: "./public/fonts/JetBrainsMono-NF-*.ttf",
            },
          ],
        },
      }),
      Icons({
        autoInstall: true,
      }),
    ],
  },
});
