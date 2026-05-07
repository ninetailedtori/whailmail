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
        <Toolbar class="border-0! border-b!">
          <template #start>
            <div class="__no_drag inline-flex gap-2 ml-4">
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

          <template #center>
            <div class="__no_drag my-2">
              <input type="search" placeholder="Search..."
                     class="border border-ctp-surface1 bg-ctp-surface0 text-ctp-text rounded-lg px-3 py-1 w-96
focus:outline-none focus:ring-2 focus:ring-ctp-blue" />
            </div>
          </template>


          <template #end>
            <div class="__no_drag">
              <div class="inline-flex gap-2 mr-4">
                <button type="button" class="window-control-button minimize"
                        @click="_minimizeWindow">
                </button>
                <button type="button" class="window-control-button maximize"
                        @click="_maximizeWindow">
                </button>
                <button type="button" class="window-control-button quit"
                        @click="_closeApplication">
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
  @apply w-full;
}

.toolbar button:not(.window-control-button) {
  @apply bg-transparent p-1.5 text-ctp-subtext0 rounded-lg cursor-pointer;
  @apply hover:text-ctp-text hover:bg-ctp-surface1;
  @apply transition duration-150 ease-in;
}

.toolbar-icons {
  @apply size-5;
}

.window-control-button {
  @apply bg-ctp-surface0;
  @apply p-2 my-2 rounded-full cursor-pointer text-ctp-text;
  @apply hover:bg-ctp-surface1;

  &.quit {
    @apply bg-ctp-red-700 hover:bg-ctp-red-950;
  }

  &.minimize {
    @apply bg-ctp-yellow-700 hover:bg-ctp-yellow-950;
  }

  &.maximize {
    @apply bg-ctp-green-700 hover:bg-ctp-green-950;
  }
}

main {
 /* min-height: calc(100vh - 42px);*/
  overflow: auto;
}
</style>
