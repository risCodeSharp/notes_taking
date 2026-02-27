<script lang="ts" setup>
import NotebookContainer from '@/components/NotebookContainer.vue'
import NoteItem from './NoteItem.vue'
import { ref, nextTick } from 'vue'

interface Note {
  id: number
  name: string
}

interface Notebook {
  id: number
  title: string
  notes: Note[]
}

let nextId = 0

const notebooks = ref<Notebook[]>([
  {
    id: nextId++,
    title: 'hello',
    notes: [
      { id: nextId++, name: 'third' },
      { id: nextId++, name: 'fourth' },
      { id: nextId++, name: 'fiveth' },
    ],
  },
])

// — New notebook inline creation —
const isCreatingNotebook = ref(false)
const newNotebookName = ref('')
const notebookInputRef = ref<HTMLInputElement | null>(null)

function beginCreateNotebook() {
  isCreatingNotebook.value = true
  newNotebookName.value = ''
  nextTick(() => notebookInputRef.value?.focus())
}

function confirmCreateNotebook() {
  const name = newNotebookName.value.trim()
  if (!name) return  // do nothing if empty
  notebooks.value.push({ id: nextId++, title: name, notes: [] })
  isCreatingNotebook.value = false
}

function cancelCreateNotebook() {
  isCreatingNotebook.value = false
}

// — New note inline creation per notebook —
const creatingNoteForNotebook = ref<number | null>(null)
const newNoteName = ref('')
const noteInputRef = ref<HTMLInputElement | null>(null)

function beginCreateNote(notebookId: number) {
  creatingNoteForNotebook.value = notebookId
  newNoteName.value = ''
  nextTick(() => noteInputRef.value?.focus())
}

function confirmCreateNote(notebookId: number) {
  const name = newNoteName.value.trim()
  if (!name) return  // do nothing if empty
  const nb = notebooks.value.find(n => n.id === notebookId)
  nb?.notes.push({ id: nextId++, name })
  creatingNoteForNotebook.value = null
}

function cancelCreateNote() {
  creatingNoteForNotebook.value = null
}

// — Notebook actions —
function renameNotebook(id: number, newTitle: string) {
  const nb = notebooks.value.find(n => n.id === id)
  if (nb) nb.title = newTitle
}

function deleteNotebook(id: number) {
  notebooks.value = notebooks.value.filter(n => n.id !== id)
}

// — Note actions —
function renameNote(notebookId: number, noteId: number, newName: string) {
  const nb = notebooks.value.find(n => n.id === notebookId)
  const note = nb?.notes.find(n => n.id === noteId)
  if (note) note.name = newName
}

function deleteNote(notebookId: number, noteId: number) {
  const nb = notebooks.value.find(n => n.id === notebookId)
  if (nb) nb.notes = nb.notes.filter(n => n.id !== noteId)
}
</script>

<template>
  <div class="w-60 flex flex-col">

    <!-- Top toolbar -->
    <div class="flex items-center justify-between px-2 py-2 mb-1">
      <span class="text-[11px] font-semibold text-slate-400 uppercase tracking-wider">Library</span>
      <div class="flex items-center gap-0.5">
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
      <div v-if="isCreatingNotebook" class="mx-2 mb-2 flex items-center gap-1.5 px-2 py-1.5 rounded-lg bg-blue-50 border border-blue-200">
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
        <!-- Tick only shown when input has value -->
        <button
          v-if="newNotebookName.trim()"
          @click="confirmCreateNotebook"
          class="p-1 rounded text-emerald-500 hover:bg-emerald-50 transition-colors shrink-0"
          title="Create"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
        </button>
        <button
          @click="cancelCreateNotebook"
          class="p-1 rounded text-slate-400 hover:bg-slate-100 transition-colors shrink-0"
          title="Cancel"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>
    </transition>

    <!-- Notebooks list -->
    <div class="flex flex-col">
      <NotebookContainer
        v-for="nb in notebooks"
        :key="nb.id"
        :title="nb.title"
        @rename="(t) => renameNotebook(nb.id, t)"
        @delete="deleteNotebook(nb.id)"
      >
        <!-- Notes -->
        <NoteItem
          v-for="note in nb.notes"
          :key="note.id"
          :name="note.name"
          @rename="(n) => renameNote(nb.id, note.id, n)"
          @delete="deleteNote(nb.id, note.id)"
        />

        <!-- Inline new note input -->
        <transition
          enter-active-class="transition-all duration-200 ease-out"
          enter-from-class="opacity-0 -translate-y-1"
          leave-active-class="transition-all duration-150 ease-in"
          leave-to-class="opacity-0 -translate-y-1"
        >
          <div
            v-if="creatingNoteForNotebook === nb.id"
            class="flex items-center gap-2 px-3 py-1.5 mx-1 rounded-lg bg-blue-50 border border-blue-200"
          >
            <span class="w-1.5 h-1.5 rounded-full bg-blue-400 shrink-0" />
            <input
              ref="noteInputRef"
              v-model="newNoteName"
              @keydown.enter="confirmCreateNote(nb.id)"
              @keydown.esc="cancelCreateNote"
              placeholder="Note name…"
              class="flex-1 min-w-0 text-[13px] bg-transparent outline-none text-slate-700 placeholder-blue-300"
            />
            <!-- Tick only shown when input has value -->
            <button
              v-if="newNoteName.trim()"
              @click="confirmCreateNote(nb.id)"
              class="p-1 rounded text-emerald-500 hover:bg-emerald-50 transition-colors shrink-0"
              title="Create"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12"/>
              </svg>
            </button>
            <button
              @click="cancelCreateNote"
              class="p-1 rounded text-slate-400 hover:bg-slate-100 transition-colors shrink-0"
              title="Cancel"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>
        </transition>

        <!-- Add note button (per notebook) -->
        <button
          @click="beginCreateNote(nb.id)"
          class="group flex items-center gap-2 w-full px-3 py-1.5 mx-1 rounded-lg text-[13px] text-slate-400 hover:text-blue-500 hover:bg-blue-50 transition-all duration-150"
        >
          <span class="flex items-center justify-center w-3.5 h-3.5 rounded-full border border-dashed border-slate-300 group-hover:border-blue-400 transition-colors duration-150 shrink-0">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-2 h-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
              <line x1="12" y1="5" x2="12" y2="19"/>
              <line x1="5" y1="12" x2="19" y2="12"/>
            </svg>
          </span>
          <span>Add note</span>
        </button>
      </NotebookContainer>
    </div>

  </div>
</template>