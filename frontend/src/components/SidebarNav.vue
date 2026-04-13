<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { ref, nextTick, watch } from 'vue'
import { useNotesStore } from '@/stores/notesStore'
import NotebookContainer from '@/components/NotebookContainer.vue'
import NoteItem from './NoteItem.vue'
import { BookCheck, SquarePlus } from '@lucide/vue'

const store = useNotesStore()
const { filteredNotebooks, loading } = storeToRefs(store)

// ── Load notes eagerly when notebooks are available ──────────────
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

// ── Actions ────────────────────────────────────────────────────
async function renameNotebook(id: number, newTitle: string) {
  await store.renameNotebook(id, newTitle)
}

async function deleteNotebook(id: number) {
  await store.deleteNotebook(id)
}

async function toggleNotebook(id: number) {
  await store.toggleNotebook(id)
}

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
  <div class="flex flex-col h-full w-full bg-white border-r border-slate-200">

    <div class="flex mt-4 mb-2 ml-4 mr-2 items-center text-slate-500">
      <span class="text-[0.7rem] font-bold uppercase tracking-widest">Library</span>

      <div class="flex items-center gap-1">
        <i v-if="loading.notebooks" class="pi pi-spin pi-spinner text-slate-400 text-xs"></i>
        <button v-else
          @click="beginCreateNotebook"
          class="p-1.5 rounded-md text-slate-400 hover:text-blue-600 hover:bg-blue-50 transition-all duration-150"
          title="New notebook"
        >
          <SquarePlus class="size-4"/>
        </button>
      </div>
    </div>

    <transition
      enter-active-class="transition-all duration-200 ease-out"
      enter-from-class="opacity-0 -translate-y-1"
      leave-active-class="transition-all duration-150 ease-in"
      leave-to-class="opacity-0 -translate-y-1"
    >
      <div v-if="isCreatingNotebook" class="mx-3 mb-3 flex items-center gap-2 px-2.5 py-2 rounded-lg bg-blue-50 border border-blue-100 shrink-0">
        <BookCheck class="size-4 text-blue-500"/>
        <input
          ref="notebookInputRef"
          v-model="newNotebookName"
          @keydown.enter="confirmCreateNotebook"
          @keydown.esc="cancelCreateNotebook"
          placeholder="Notebook Name"
          class="flex-1 min-w-0 text-[0.75rem] text-slate-700 font-semibold bg-transparent outline-none placeholder-blue-300 tracking-wider"
        />
        <div class="flex items-center gap-1">
          <button v-if="newNotebookName.trim()" @click="confirmCreateNotebook" class="p-1 rounded text-emerald-600 hover:bg-white transition-colors">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
          </button>
          <button @click="cancelCreateNotebook" class="p-1 rounded text-slate-400 hover:bg-white transition-colors">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
          </button>
        </div>
      </div>
    </transition>

    <div v-if="!loading.notebooks && filteredNotebooks.length === 0" class="flex flex-col items-center justify-center flex-1 gap-3 text-slate-300 px-6 text-center">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-10 h-10 opacity-20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
      </svg>
      <p class="text-[12px] font-medium italic">Create your first notebook to get started</p>
    </div>

    <div class="flex-1 min-h-0 overflow-y-auto overflow-x-hidden px-1 custom-scrollbar">
      <div class="flex flex-col pb-6">
        <NotebookContainer
          v-for="nb in filteredNotebooks"
          :key="nb.id"
          :title="nb.name"
          @rename="(t: string) => renameNotebook(nb.id, t)"
          @delete="deleteNotebook(nb.id)"
          @toggle="toggleNotebook(nb.id)"
        >
          <template v-if="nb.notesLoaded">
            <NoteItem
              v-for="note in nb.notes"
              :key="note.id"
              :name="note.title"
              class="ml-6"
              @click="openNote(nb.id, note.id)"
              @rename="(n: string) => renameNote(nb.id, note.id, n)"
              @delete="deleteNote(nb.id, note.id)"
            />
            <p v-if="nb.notes.length === 0" class="px-10 py-2 text-[11px] text-slate-400 italic">
              Empty notebook
            </p>
          </template>

          <div v-else class="px-10 py-2 flex items-center gap-2 text-slate-400">
            <i class="pi pi-spin pi-spinner text-[10px]"></i>
            <span class="text-[11px]">Syncing...</span>
          </div>

          <transition 
            enter-active-class="transition-all duration-200" 
            enter-from-class="opacity-0 -translate-x-2"
          >
            <div v-if="creatingNoteForNotebook === nb.id" class="flex items-center gap-2 px-3 py-1.5 mx-3 mt-1 rounded-md bg-slate-50 border border-slate-200">
              <span class="w-1.5 h-1.5 rounded-full bg-blue-400 shrink-0" />
              <input 
                ref="noteInputRef" 
                v-model="newNoteName" 
                @keydown.enter="confirmCreateNote(nb.id)" 
                @keydown.esc="cancelCreateNote" 
                placeholder="Note title..." 
                class="flex-1 min-w-0 text-[13px] bg-transparent outline-none text-slate-700 placeholder-slate-300" 
              />
              <button @click="confirmCreateNote(nb.id)" class="text-emerald-600 p-0.5">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
              </button>
            </div>
          </transition>

          <button 
            @click="beginCreateNote(nb.id)" 
            class="group flex items-center gap-2 w-fit px-3 py-1.5 mt-1 ml-8 rounded-md text-[12px] text-slate-400 hover:text-blue-600 hover:bg-blue-50/50 transition-all duration-150"
          >
            <span class="flex items-center justify-center w-3.5 h-3.5 rounded-full border border-dashed border-slate-300 group-hover:border-blue-400 shrink-0">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-2 h-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
              </svg>
            </span>
            <span>Add page</span>
          </button>
        </NotebookContainer>
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #e2e8f0;
  border-radius: 10px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: #cbd5e1;
}
</style>