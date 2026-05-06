import { resolve } from "path";

import ui from "@nuxt/ui/vite";
import vue from "@vitejs/plugin-vue";
import { defineConfig } from "electron-vite";
import Fonts from "unplugin-fonts/vite";
import IconsResolver from "unplugin-icons/resolver";
import Icons from "unplugin-icons/vite";
import vueDevTools from "vite-plugin-vue-devtools";
import VueRouter from "vue-router/vite";

export default defineConfig({
  main: {},
  preload: {},
  renderer: {
    resolve: {
      alias: {
        "@renderer": resolve("src/renderer/src"),
      },
    },
    plugins: [
      VueRouter(),
      vue(),
      vueDevTools(),
      ui({
        components: {
          resolvers: [IconsResolver()],
          dirs: ["./src/renderer/src/components"],
        },
        autoImport: {
          imports: ["vue", "vue-router", "pinia"],
          dirs: ["./src/renderer/src/composables", "./src/renderer/src/utils"],
          eslintrc: {
            enabled: true,
          },
        },
      }),
      Fonts({
        custom: {
          families: [
            {
              name: "JetBrainsMono Nerd Font",
              local: "JetBrainsMono Nerd Font",
              src: "./public/fonts/JetBrainsMono-NF-*.woff2",
            },
            {
              name: "Inria Sans",
              local: "Inria Sans",
              src: "https://fonts.googleapis.com/css2?family=Inria+Sans:ital,wght@0,300;0,400;0,700;1,300;1,400;1,700&display=swap",
            },
            {
              name: "Afacad Flux",
              local: "Afacad Flux",
              src: "https://fonts.googleapis.com/css2?family=Afacad+Flux:wght@100..1000&display=swap",
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
