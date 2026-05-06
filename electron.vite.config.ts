import { resolve } from "path";

import ui from "@nuxt/ui/vite";
import vue from "@vitejs/plugin-vue";
import { defineConfig } from "electron-vite";
import AutoImport from "unplugin-auto-import/vite";
import IconsResolver from "unplugin-icons/resolver";
import Icons from "unplugin-icons/vite";
import Components from "unplugin-vue-components/vite";
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
      ui(),
      AutoImport({
        imports: ["vue", "vue-router", "pinia"],
        dirs: ["./src/renderer/src/composables", "./src/renderer/src/utils"],
        eslintrc: {
          enabled: true,
        },
      }),
      Components({
        resolvers: [IconsResolver()],
        dirs: ["./src/renderer/src/components"],
      }),
      Icons({
        autoInstall: true,
      }),
    ],
  },
});
