<!--
SPDX-FileCopyrightText: 2026 2026-Present ninetailedtori <ninetailedtori@uwu.gal>
SPDX-FileContributor: WhailMail contributors

SPDX-License-Identifier: GPL-3.0-or-later
-->

<script lang="ts" setup>
import { useTheme } from "../stores/theme";

const theme = useTheme();

const emoji = {
  latte: "🌻",
  frappe: "🪴",
  macchiato: "🌺",
  mocha: "🌿",
} as const;

const handleThemeChange = async (e: Event) => {
  await theme.setFlavour((e.target as HTMLSelectElement).value as never);
};

const handleAccentChange = async (e: Event) => {
  await theme.setAccent((e.target as HTMLSelectElement).value as never);
};
</script>

<template>
  <div class="flex items-center gap-4">
    <div class="flex items-center gap-2">
      <label class="text-ctp-text text-sm font-medium" for="theme"
        >Flavor:</label
      >
      <select
        id="theme"
        :value="theme.currentFlavour"
        class="bg-ctp-surface0 text-ctp-text border-ctp-surface1 rounded border px-3 py-1 text-sm"
        @change="handleThemeChange"
      >
        <option
          v-for="flavour in theme.flavours"
          :key="flavour"
          :value="flavour"
        >
          {{ emoji[flavour] }}
          {{ flavour.charAt(0).toUpperCase() + flavour.slice(1) }}
        </option>
      </select>
    </div>

    <div class="flex items-center gap-2">
      <label class="text-ctp-text text-sm font-medium" for="accent"
        >Accent:</label
      >
      <select
        id="accent"
        :value="theme.currentAccent"
        class="bg-ctp-surface0 text-ctp-text border-ctp-surface1 rounded border px-3 py-1 text-sm"
        @change="handleAccentChange"
      >
        <option
          v-for="colour in theme.accentColours"
          :key="colour"
          :value="colour"
        >
          {{ colour.charAt(0).toUpperCase() + colour.slice(1) }}
        </option>
      </select>
    </div>
  </div>
</template>
