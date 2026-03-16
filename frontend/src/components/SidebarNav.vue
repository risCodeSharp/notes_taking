<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { ref, nextTick, onMounted, watch } from 'vue'
import { useNotesStore } from '@/stores/notesStore'
import NotebookContainer from '@/components/NotebookContainer.vue'
import NoteItem from './NoteItem.vue'

const store = useNotesStore()
const { filteredNotebooks, loading } = storeToRefs(store)

// ── Load notes as soon as notebooks are available ──────────────
// NotebookContainer defaults isOpen=true, but store has notesLoaded=false.
// Watch for notebooks to populate (fetchNotebooks runs in EditorView onMounted)
// then eagerly fetch notes for all of them.
watch(
  () => store.notebooks.length,
  async (len) => {
    if (len === 0) return
    for (const nb of store.notebooks) {
      if (!nb.notesLoaded) {
        await store.fetchNotes(nb.id)
      }
    }
  },
  { immediate: true }
)

// ── Notebook creation ──────────────────────────────────────────
const isCreatingNotebook = ref(false)
const newNotebookName    = ref('')
const notebookInputRef   = ref<HTMLInputElement | null>(null)

function beginCreateNotebook() {
  isCreatingNotebook.value = true
  newNotebookName.value    = ''
  nextTick(() => notebookInputRef.value?.focus())
}

async function confirmCreateNotebook() {
  const name = newNotebookName.value.trim()
  if (!name) return
  await store.createNotebook(name)
  isCreatingNotebook.value = false
}

function cancelCreateNotebook() {
  isCreatingNotebook.value = false
}

// ── Note creation ──────────────────────────────────────────────
const creatingNoteForNotebook = ref<number | null>(null)
const newNoteName             = ref('')
const noteInputRef            = ref<HTMLInputElement[]>([])

function beginCreateNote(notebookId: number) {
  creatingNoteForNotebook.value = notebookId
  newNoteName.value             = ''
  nextTick(() => noteInputRef.value[0]?.focus())
}

async function confirmCreateNote(notebookId: number) {
  const name = newNoteName.value.trim()
  if (!name) return
  await store.createNote(notebookId, { title: name, content: '' })
  creatingNoteForNotebook.value = null
}

function cancelCreateNote() {
  creatingNoteForNotebook.value = null
}

// ── Notebook actions ───────────────────────────────────────────
async function renameNotebook(id: number, newTitle: string) {
  await store.renameNotebook(id, newTitle)
}

async function deleteNotebook(id: number) {
  await store.deleteNotebook(id)
}

// toggleNotebook handles lazy-load for notebooks not yet fetched
async function toggleNotebook(id: number) {
  await store.toggleNotebook(id)
}

// ── Note actions ───────────────────────────────────────────────
async function openNote(notebookId: number, noteId: number) {
  await store.openNote(notebookId, noteId)
}

async function renameNote(notebookId: number, noteId: number, newName: string) {
  await store.saveNote(notebookId, noteId, { title: newName })
}

async function deleteNote(notebookId: number, noteId: number) {
  await store.deleteNote(notebookId, noteId)
}
</script>

<template>
  <div class="flex flex-col h-full w-full">

    <!-- Top toolbar -->
    <div class="flex items-center justify-between px-2 py-2 mb-1 shrink-0">
      <span class="text-[11px] font-semibold text-slate-400 uppercase tracking-wider">Library</span>
      <div class="flex items-center gap-0.5">
        <i v-if="loading.notebooks" class="pi pi-spin pi-spinner text-slate-400 text-xs mr-1"></i>
        <button
          @click="beginCreateNotebook"
          class="p-1.5 rounded-md text-slate-400 hover:text-blue-500 hover:bg-blue-50 transition-all duration-150"
          title="New notebook"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M2 6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6z"/>
            <path d="M8 2v4M8 18v4M16 2v4M16 18v4"/>
            <line x1="12" y1="9" x2="12" y2="15"/>
            <line x1="9" y1="12" x2="15" y2="12"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- Inline new notebook input -->
    <transition
      enter-active-class="transition-all duration-200 ease-out"
      enter-from-class="opacity-0 -translate-y-1"
      leave-active-class="transition-all duration-150 ease-in"
      leave-to-class="opacity-0 -translate-y-1"
    >
      <div v-if="isCreatingNotebook" class="mx-2 mb-2 flex items-center gap-1.5 px-2 py-1.5 rounded-lg bg-blue-50 border border-blue-200 shrink-0">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-blue-400 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M2 6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6z"/>
          <path d="M8 2v4M8 18v4M16 2v4M16 18v4"/>
        </svg>
        <input
          ref="notebookInputRef"
          v-model="newNotebookName"
          @keydown.enter="confirmCreateNotebook"
          @keydown.esc="cancelCreateNotebook"
          placeholder="Notebook name…"
          class="flex-1 min-w-0 text-[13px] font-semibold bg-transparent outline-none text-slate-700 placeholder-blue-300 uppercase tracking-wide"
        />
        <button v-if="newNotebookName.trim()" @click="confirmCreateNotebook" class="p-1 rounded text-emerald-500 hover:bg-emerald-50 transition-colors shrink-0" title="Create">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
        </button>
        <button @click="cancelCreateNotebook" class="p-1 rounded text-slate-400 hover:bg-slate-100 transition-colors shrink-0" title="Cancel">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>
    </transition>

    <!-- Empty state -->
    <div v-if="!loading.notebooks && filteredNotebooks.length === 0" class="flex flex-col items-center justify-center flex-1 gap-2 text-slate-400">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8 opacity-30" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
      </svg>
      <p class="text-[12px]">No notebooks yet</p>
    </div>

    <!-- Scrollable notebooks list -->
    <div class="flex-1 min-h-0 overflow-y-auto overflow-x-hidden">
      <div class="flex flex-col pb-4">
        <NotebookContainer
          v-for="nb in filteredNotebooks"
          :key="nb.id"
          :title="nb.name"
          @rename="(t: string) => renameNotebook(nb.id, t)"
          @delete="deleteNotebook(nb.id)"
          @toggle="toggleNotebook(nb.id)"
        >
          <!-- Notes -->
          <template v-if="nb.notesLoaded">
            <NoteItem
              v-for="note in nb.notes"
              :key="note.id"
              :name="note.title"
              @click="openNote(nb.id, note.id)"
              @rename="(n: string) => renameNote(nb.id, note.id, n)"
              @delete="deleteNote(nb.id, note.id)"
            />
            <p v-if="nb.notes.length === 0" class="px-4 py-2 text-[12px] text-slate-400 italic">
              No notes yet
            </p>
          </template>

          <!-- Loading spinner while notes fetch -->
          <div v-else class="px-4 py-2 flex items-center gap-2 text-slate-400">
            <i class="pi pi-spin pi-spinner text-xs"></i>
            <span class="text-[12px]">Loading notes…</span>
          </div>

          <!-- Inline new note input -->
          <transition enter-active-class="transition-all duration-200 ease-out" enter-from-class="opacity-0 -translate-y-1" leave-active-class="transition-all duration-150 ease-in" leave-to-class="opacity-0 -translate-y-1">
            <div v-if="creatingNoteForNotebook === nb.id" class="flex items-center gap-2 px-3 py-1.5 mx-1 rounded-lg bg-blue-50 border border-blue-200">
              <span class="w-1.5 h-1.5 rounded-full bg-blue-400 shrink-0" />
              <input ref="noteInputRef" v-model="newNoteName" @keydown.enter="confirmCreateNote(nb.id)" @keydown.esc="cancelCreateNote" placeholder="Note name…" class="flex-1 min-w-0 text-[13px] bg-transparent outline-none text-slate-700 placeholder-blue-300" />
              <button v-if="newNoteName.trim()" @click="confirmCreateNote(nb.id)" class="p-1 rounded text-emerald-500 hover:bg-emerald-50 transition-colors shrink-0" title="Create">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
              </button>
              <button @click="cancelCreateNote" class="p-1 rounded text-slate-400 hover:bg-slate-100 transition-colors shrink-0" title="Cancel">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
              </button>
            </div>
          </transition>

          <!-- Add note button -->
          <button @click="beginCreateNote(nb.id)" class="group flex items-center gap-2 w-full px-3 py-1.5 mx-1 rounded-lg text-[13px] text-slate-400 hover:text-blue-500 hover:bg-blue-50 transition-all duration-150">
            <span class="flex items-center justify-center w-3.5 h-3.5 rounded-full border border-dashed border-slate-300 group-hover:border-blue-400 transition-colors duration-150 shrink-0">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-2 h-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
              </svg>
            </span>
            <span>Add note</span>
          </button>
        </NotebookContainer>
      </div>
    </div>

  </div>
</template>