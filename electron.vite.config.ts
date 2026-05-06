import { resolve } from "path";

import ui from "@nuxt/ui/vite";
import vue from "@vitejs/plugin-vue";
import { defineConfig } from "electron-vite";
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
      Icons({
        autoInstall: true,
      }),
    ],
  },
});
