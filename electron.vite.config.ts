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

export default defineConfig({
  main: {},
  preload: {},
  renderer: {
    resolve: {
      alias: {
        "@renderer": resolve("src/renderer/src"),
        "@stores": resolve("src/renderer/stores"),
      },
    },
    plugins: [
      tailwindcss(),
      VueRouter({
        dts: resolve("src/renderer/typed-router.d.ts"),
      }),
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
