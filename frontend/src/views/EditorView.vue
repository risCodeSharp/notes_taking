<script lang="ts" setup>
import { ref, computed, onMounted, watch } from "vue";
import { storeToRefs } from "pinia";
import { useNotesStore } from "@/stores/notesStore";
import type { EditorMode} from "@/types";

import Splitter      from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";

import DesktopSidebar from "@/components/DesktopSidebar.vue";
import MobileDrawer   from "@/components/MobileDrawer.vue";
import ModeSwitcher   from "@/components/ModeSwitcher.vue";
import EditorPanel    from "@/components/EditorPanel.vue";
import EditorNavbar   from "@/components/EditorNavbar.vue";
import TabButton      from "@/components/TabButton.vue";

import "md-editor-v3/lib/style.css";
import "md-editor-v3/lib/preview.css";

// ── Store ──────────────────────────────────────────────────────
const store = useNotesStore();
const { tabs, activeNoteId, activeNote, loading, error } = storeToRefs(store);

// ── Local UI state ─────────────────────────────────────────────
const currentMode     = ref<EditorMode>("edit");
const visibleOnMobile = ref(false);

// activeTab drives PrimeVue Tabs — must be the string tab index
const activeTab = computed({
  get: () => {
    const idx = tabs.value.findIndex(t => t.noteId === activeNoteId.value);
    return idx >= 0 ? String(idx) : "0";
  },
  set: (val: string) => {
    const tab = tabs.value[Number(val)];
    if (tab) store.setActiveTab(tab.noteId);
  },
});

// ── Lifecycle ──────────────────────────────────────────────────
onMounted(async () => {
  await store.fetchNotebooks();
});

// ── Handlers ──────────────────────────────────────────────────

function showSidebar() {
  visibleOnMobile.value = true;
}

// Called by TabButton close (×) button
function handleCloseTab(noteId: number) {
  store.closeTab(noteId);
}

// Called by TabButton click (switch tab)
function handleTabClick(tabIndex: number) {
  const tab = tabs.value[tabIndex];
  if (tab) store.setActiveTab(tab.noteId);
}

// Called by EditorPanel on every keystroke
function handleContentUpdate(noteId: number, content: string) {
  store.updateContent(content);
}

// Called by ModeSwitcher Save button
async function handleSave() {
  if (!activeNote.value || !activeNoteId.value) return;
  const tab = tabs.value.find(t => t.noteId === activeNoteId.value);
  if (!tab) return;
  await store.saveNote(tab.notebookId, activeNoteId.value, {
    title:   activeNote.value.title,
    content: activeNote.value.content,
  });
}

// Called by EditorNavbar search (optional: filter sidebar)
function handleSearch(query: string) {
  store.setSearchQuery(query);
}

// Keyboard shortcut: Ctrl+S / Cmd+S
function onKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === "s") {
    e.preventDefault();
    handleSave();
  }
}
</script>

<template>
  <div class="flex flex-col h-screen overflow-hidden" @keydown="onKeydown" tabindex="-1">

    <!-- Navbar -->
    <EditorNavbar
      @open-sidebar="showSidebar"
      @search="handleSearch"
      class="border-b border-b-gray-200 shrink-0"
    />

    <!-- Mobile Drawer -->
    <MobileDrawer v-model:visible="visibleOnMobile" />

    <!-- Error banner -->
    <div v-if="error" class="px-4 py-2 text-sm text-red-600 bg-red-50 border-b border-red-200 shrink-0">
      {{ error }}
    </div>

    <!-- Main Layout -->
    <Splitter
      class="flex-1 min-h-0 overflow-hidden"
      :pt="{
        gutter: 'w-2 bg-gray-200 hover:bg-blue-400 transition-colors duration-150 cursor-col-resize',
      }"
    >
      <!-- Sidebar Panel -->
      <SplitterPanel
        :size="35"
        :minSize="12"
        class="hidden md:flex flex-col overflow-hidden"
      >
        <DesktopSidebar />
      </SplitterPanel>

      <!-- Editor Panel -->
      <SplitterPanel :size="82" :minSize="50" class="overflow-auto bg-gray-100">
        <main class="h-full">

          <!-- Empty state — no tabs open -->
          <div
            v-if="tabs.length === 0"
            class="h-full flex flex-col items-center justify-center text-gray-400 gap-3"
          >
            <i class="pi pi-file text-4xl opacity-30"></i>
            <p class="text-sm">Select a note from the sidebar to start editing</p>
          </div>

          <!-- Tabs -->
          <div v-else class="ml-2 mx-auto shadow-xl rounded-lg">

            <!-- Tab Headers (plain div, no PrimeVue Tabs wrapper needed) -->
            <div class="flex border-b bg-gray-50 border-gray-300 overflow-x-auto pt-2">
              <TabButton
                v-for="(tab, index) in tabs"
                :key="tab.noteId"
                :tab-number="index"
                :text="tab.name"
                :active-tab="Number(activeTab)"
                :is-dirty="tab.isDirty"
                @update-tab="handleTabClick"
                @close-tab="handleCloseTab(tab.noteId)"
              />
            </div>

            <!-- Mode switcher + Save -->
            <ModeSwitcher
              :current-mode="currentMode"
              :is-saving="loading.saving"
              @update-mode="(mode: EditorMode) => currentMode = mode"
              @save="handleSave"
            />

            <!-- Active note content — renders immediately on open, no TabPanel gating -->
            <div class="p-5 border border-t-0 border-gray-300 rounded-b-md bg-white">
              <div v-if="loading.note" class="flex items-center justify-center py-12">
                <i class="pi pi-spin pi-spinner text-2xl text-blue-400"></i>
              </div>
              <EditorPanel
                v-else-if="activeNote && activeNoteId"
                :key="activeNoteId"
                :current-mode="currentMode"
                :note-name="activeNote.title"
                :markdown="activeNote.content"
                @update:markdown="(val: string) => handleContentUpdate(activeNoteId!, val)"
              />
            </div>

          </div>

        </main>
      </SplitterPanel>
    </Splitter>

  </div>
</template>