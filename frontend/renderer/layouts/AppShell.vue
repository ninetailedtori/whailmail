<script setup lang="ts">
import {
  CloudDownload,
  MailPlus,
  BookUser as Contacts,
  CalendarDays as Calendar,
  Minus,
  Square,
  X,
} from '@lucide/vue';
import Toolbar from 'primevue/toolbar';


const _minimizeWindow = () => {
  window.windowControls.minimize();
}

const _maximizeWindow = () => {
  window.windowControls.maximize();
}

const _closeApplication = () => {
  window.windowControls.close();
}

</script>

<template>
  <div class="dragger">
    <nav class="navbar">
      <div class="toolbar">
        <Toolbar class="rounded-none border-0! border-b! bg-zinc-800 pl-2!">
          <template #start>
            <div class="__no_drag inline-flex gap-2">
              <button type="button" v-tooltip.bottom="'Fetch new messages'">
                <cloud-download class="toolbar-icons" />
              </button>
              <button type="button" v-tooltip.bottom="'Create new message'">
                <mail-plus class="toolbar-icons" />
              </button>
              <button type="button" v-tooltip.bottom="'View contacts'">
                <contacts class="toolbar-icons" />
              </button>
              <button type="button" v-tooltip.bottom="'View calendar'">
                <calendar class="toolbar-icons" />
              </button>
            </div>
          </template>


          <template #end>
            <div class="__no_drag">
              <div class="inline-flex gap-2">
                <button type="button" class="window-control-button" @click="_minimizeWindow">
                  <minus class="window-control-icon" />
                </button>
                <button type="button" class="window-control-button" @click="_maximizeWindow">
                  <square class="window-control-icon" />
                </button>
                <button type="button" class="window-control-button quit" @click="_closeApplication">
                  <x class="window-control-icon size-4!" />
                </button>
              </div>
            </div>
          </template>


        </Toolbar>

      </div>
    </nav>
  </div>

  <main>
    <slot />
  </main>

  <footer></footer>
</template>

<style scoped>
@reference './../assets/main.css';

.toolbar {
  @apply w-full leading-0!;
}

.toolbar button:not(.window-control-button) {
  @apply bg-transparent p-1.5 text-ctp-subtext0 rounded-lg cursor-pointer;
  @apply hover:text-ctp-text hover:bg-ctp-surface1;
  @apply transition duration-150 ease-in;
}

.toolbar-icons {
  @apply size-4;
}

.window-control-button {
  @apply bg-transparent p-2.5 cursor-pointer text-ctp-text;
  @apply hover:bg-ctp-surface1;

  &.quit {
    @apply hover:bg-ctp-red-900;
  }
}

.window-control-icon {
  @apply size-3 rounded-none;
}

main {
  min-height: calc(100vh - 37px);
  overflow: auto;
}
</style>
