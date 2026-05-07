/*
 * SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
 * SPDX-FileContributor: WhailMail contributors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import {
  defineConfigWithVueTs,
  vueTsConfigs,
} from "@vue/eslint-config-typescript";
import skipFormatting from "eslint-config-prettier/flat";
import pluginOxlint from "eslint-plugin-oxlint";
import pluginVue from "eslint-plugin-vue";
import { globalIgnores } from "eslint/config";

import eslintrcImport from "./.eslintrc-auto-import.json";

// To allow more languages other than `ts` in `.vue` files, uncomment the following lines:
// import { configureVueProject } from '@vue/eslint-config-typescript'
// configureVueProject({ scriptLangs: ['ts', 'tsx'] })
// More info at https://github.com/vuejs/eslint-config-typescript/#advanced-setup

export default defineConfigWithVueTs(
  {
    name: "app/files-to-lint",
    languageOptions: { ...eslintrcImport },
    files: ["**/*.{vue,ts,mts,tsx}"],
  },

  globalIgnores(["**/dist/**", "**/out/**", "**/coverage/**"]),

  ...pluginVue.configs["flat/essential"],
  vueTsConfigs.recommended,

  {
    name: "app/vue-rule-overrides",
    rules: {
      "@typescript-eslint/no-unused-vars": "warn",
      "@typescript-eslint/no-empty-object-type": "warn",
      "vue/multi-word-component-names": "off",
      "vue/require-default-prop": "off",
      "vue/block-lang": [
        "error",
        {
          script: {
            lang: "ts",
          },
        },
      ],
    },
  },

  ...pluginOxlint.buildFromOxlintConfigFile(".oxlintrc.json"),

  skipFormatting
);
