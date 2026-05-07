<script lang="ts" setup>
import { onMounted, ref } from "vue";

import ThemeDropdown from "./components/ThemeDropdown.vue";

const platform = ref("unknown");

onMounted(async () => {
  // @ts-expect-error (define in dts)
  platform.value = await window.electron.ipcRenderer.invoke("get-platform");
});
</script>

<template>
  <div class="bg-ctp-base min-h-screen">
    <div class="mx-auto max-w-4xl px-4 py-10">
      <div class="mb-12 flex items-center justify-between">
        <div>
          <h1 class="text-ctp-lavender text-5xl font-bold">🐋 WhailMail</h1>
          <p class="text-ctp-subtext1">
            The app that makes handling your mail, a whale of a time!
          </p>
        </div>
        <ThemeDropdown />
      </div>

      <div
        class="mx-auto mb-12 grid max-w-2xl grid-cols-1 gap-6 md:grid-cols-2"
      >
        <div class="bg-ctp-surface0 border-ctp-surface1 rounded-lg border p-6">
          <h2 class="text-ctp-text mb-4 text-lg font-semibold">Status</h2>
          <div class="text-ctp-subtext0 space-y-2 text-sm">
            <p>
              Build: <span class="text-ctp-blue font-mono">development</span>
            </p>
            <p>
              Platform:
              <span class="text-ctp-blue font-mono">{{ platform }}</span>
            </p>
          </div>
        </div>

        <div class="bg-ctp-surface0 border-ctp-surface1 rounded-lg border p-6">
          <h2 class="text-ctp-text mb-4 text-lg font-semibold">Quick Start</h2>
          <div class="flex flex-col gap-2">
            <button
              class="bg-ctp-blue hover:bg-ctp-sapphire text-ctp-crust rounded px-4 py-2 text-sm font-medium transition"
            >
              Connect Mail
            </button>
            <button
              class="bg-ctp-surface1 hover:bg-ctp-surface2 text-ctp-text rounded px-4 py-2 text-sm font-medium transition"
            >
              Settings
            </button>
          </div>
        </div>
      </div>

      <div class="bg-ctp-surface0 border-ctp-surface1 rounded-lg border p-6">
        <h3 class="text-ctp-subtext1 mb-3 text-sm font-semibold">Roadmap</h3>
        <ul
          class="text-ctp-subtext0 grid grid-cols-1 gap-2 text-xs md:grid-cols-2"
        >
          <li>✓ Cross-platform native UI</li>
          <li>✓ Multi-protocol support</li>
          <li>✓ Background sync & indexing</li>
          <li>✓ System notifications</li>
          <li>✓ Mail filtering</li>
          <li>✓ Self-hosted support</li>
        </ul>
      </div>
    </div>
  </div>
</template>
