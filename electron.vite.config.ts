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
  "@shared": resolve("./frontend/shared"),
};

export default defineConfig({
  main: {
    resolve: {
      alias: {
        ...sharedAliases,
      },
    },
  },
  preload: {
    resolve: {
      alias: {
        ...sharedAliases,
      },
    },
  },
  renderer: {
    resolve: {
      alias: {
        ...sharedAliases,
        "@": resolve("./frontend/renderer"),
        "@stores": resolve("./frontend/renderer/stores"),
        "@components": resolve("./frontend/renderer/components"),
        "@pages": resolve("./frontend/renderer/pages"),
        "@layouts": resolve("./frontend/renderer/layouts"),
        "@router": resolve("./frontend/renderer/router"),
        "@styles": resolve("./frontend/renderer/styles"),
      },
    },
    plugins: [
      tailwindcss(),
      VueRouter({
        dts: resolve("frontend/renderer/typed-router.d.ts"),
      }),
      vue(),
      vueDevTools(),
      ui({
        router: true,
        components: {
          resolvers: [IconsResolver()],
          dirs: ["./frontend/renderer/components"],
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
          dirs: [
            "./frontend/renderer/composables",
            "./frontend/renderer/utils",
          ],
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
