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
                <button type="button" class="window-control-button">
                  <minus class="window-control-icon" />
                </button>
                <button type="button" class="window-control-button">
                  <square class="window-control-icon" />
                </button>
                <button type="button" class="window-control-button quit">
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

<style scoped lang="scss">
@reference './../assets/main.scss';

.toolbar {
  @apply w-full leading-0!;
}

.toolbar button:not(.window-control-button) {
  @apply bg-transparent p-1.5 text-zinc-300 rounded-lg cursor-pointer;
  @apply hover:text-white hover:bg-zinc-700;
  @apply transition duration-150 ease-in;
}

.toolbar-icons {
  @apply size-4;
}

.window-control-button {
  @apply bg-transparent p-2.5;
  @apply hover:bg-zinc-700;

  &.quit {
    @apply hover:bg-red-500;
  }
}

.window-control-icon {
  @apply size-3 rounded-none;
}
</style>
