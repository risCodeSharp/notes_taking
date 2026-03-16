// src/stores/useNotesStore.ts
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authService }     from '@/services/authService'
import { notebookService } from '@/services/notebookService'
import { noteService }     from '@/services/noteService'
import type {
  NotebookResponse,
  NoteResponse,
  Note,
  CreateUserPayload,
  LoginPayload,
  CreateNotePayload,
  UpdateNotePayload,
} from '@/types'

export type EditorMode = 'edit' | 'preview' | 'split'

export interface Tab {
  noteId:     number
  notebookId: number
  name:       string
  isDirty:    boolean
}

export interface NotebookLocal extends NotebookResponse {
  notes:       Note[]
  isExpanded:  boolean
  notesLoaded: boolean
}

export const useNotesStore = defineStore('notes', () => {

  // ── State ─────────────────────────────────────────────────────
  const notebooks    = ref<NotebookLocal[]>([])
  const activeNote   = ref<Note | null>(null)
  const tabs         = ref<Tab[]>([])
  const activeNoteId = ref<number | null>(null)
  const editorMode   = ref<EditorMode>('edit')
  const searchQuery  = ref('')
  const loading      = ref({ notebooks: false, note: false, saving: false })
  const error        = ref<string | null>(null)

  // ── Auth ──────────────────────────────────────────────────────
  async function register(payload: CreateUserPayload) {
    return authService.register(payload)
  }

  async function login(payload: LoginPayload) {
    return authService.login(payload)
  }

  function logout() {
    authService.logout()
    notebooks.value    = []
    tabs.value         = []
    activeNote.value   = null
    activeNoteId.value = null
    error.value        = null
  }

  // ── Notebooks ─────────────────────────────────────────────────
  async function fetchNotebooks() {
    loading.value.notebooks = true
    error.value = null
    try {
      // notebookService.list() now returns NotebookResponse[] directly (already unwrapped)
      const list = await notebookService.list()
      notebooks.value = list.map(nb => ({
        ...nb,
        notes:       [],
        isExpanded:  false,
        notesLoaded: false,
      }))
    } catch (e: any) {
      error.value = e.response?.data?.message ?? 'Failed to load notebooks'
    } finally {
      loading.value.notebooks = false
    }
  }

  async function createNotebook(name: string) {
    // returns NotebookResponse directly
    const nb = await notebookService.create({ name })
    notebooks.value.push({ ...nb, notes: [], isExpanded: true, notesLoaded: true })
    return nb
  }

  async function renameNotebook(notebookId: number, name: string) {
    const updated = await notebookService.update(notebookId, { name })
    const nb = notebooks.value.find(n => n.id === notebookId)
    if (nb) nb.name = updated.name
  }

  async function deleteNotebook(notebookId: number) {
    await notebookService.delete(notebookId)
    const nb = notebooks.value.find(n => n.id === notebookId)
    nb?.notes.forEach(note => closeTab(note.id))
    notebooks.value = notebooks.value.filter(n => n.id !== notebookId)
  }

  async function toggleNotebook(notebookId: number) {
    const nb = notebooks.value.find(n => n.id === notebookId)
    if (!nb) return
    nb.isExpanded = !nb.isExpanded
    if (nb.isExpanded && !nb.notesLoaded) {
      await fetchNotes(notebookId)
    }
  }

  // ── Notes ─────────────────────────────────────────────────────
  async function fetchNotes(notebookId: number) {
    const list = await noteService.list(notebookId)
    const nb   = notebooks.value.find(n => n.id === notebookId)
    if (nb) {
      nb.notes       = list
      nb.notesLoaded = true
    }
    return list
  }

  async function openNote(notebookId: number, noteId: number) {
    const existing = tabs.value.find(t => t.noteId === noteId)
    if (existing) activeNoteId.value = noteId

    loading.value.note = true
    error.value = null
    try {
      const note = await noteService.get(notebookId, noteId)
      activeNote.value   = note
      activeNoteId.value = noteId

      if (!existing) {
        tabs.value.push({
          noteId:     note.id,
          notebookId,
          name:       note.title,
          isDirty:    false,
        })
      }
    } catch (e: any) {
      error.value = e.response?.data?.message ?? 'Failed to load note'
    } finally {
      loading.value.note = false
    }
  }

  async function createNote(notebookId: number, payload: CreateNotePayload) {
    const note = await noteService.create(notebookId, payload)
    const nb   = notebooks.value.find(n => n.id === notebookId)
    nb?.notes.push(note)
    await openNote(notebookId, note.id)
    return note
  }

  async function saveNote(notebookId: number, noteId: number, payload: UpdateNotePayload) {
    loading.value.saving = true
    try {
      const updated = await noteService.update(notebookId, noteId, payload)
      if (activeNote.value?.id === noteId) activeNote.value = updated
      if (payload.title) {
        const tab  = tabs.value.find(t => t.noteId === noteId)
        const nb   = notebooks.value.find(n => n.id === notebookId)
        const item = nb?.notes.find(n => n.id === noteId)
        if (tab)  tab.name    = payload.title
        if (item) item.title  = payload.title
      }
      const tab = tabs.value.find(t => t.noteId === noteId)
      if (tab) tab.isDirty = false
      return updated
    } finally {
      loading.value.saving = false
    }
  }

  async function deleteNote(notebookId: number, noteId: number) {
    await noteService.delete(notebookId, noteId)
    closeTab(noteId)
    const nb = notebooks.value.find(n => n.id === notebookId)
    if (nb) nb.notes = nb.notes.filter(n => n.id !== noteId)
    if (activeNote.value?.id === noteId) activeNote.value = null
  }

  function updateContent(content: string) {
    if (!activeNote.value) return
    activeNote.value.content = content
    const tab = tabs.value.find(t => t.noteId === activeNote.value!.id)
    if (tab) tab.isDirty = true
  }

  // ── Tabs ──────────────────────────────────────────────────────
  function closeTab(noteId: number) {
    const idx = tabs.value.findIndex(t => t.noteId === noteId)
    if (idx === -1) return
    tabs.value.splice(idx, 1)
    if (activeNoteId.value === noteId) {
      const next = tabs.value[idx - 1] ?? tabs.value[idx] ?? null
      if (next) {
        openNote(next.notebookId, next.noteId)
      } else {
        activeNoteId.value = null
        activeNote.value   = null
      }
    }
  }

  function setActiveTab(noteId: number) {
    const tab = tabs.value.find(t => t.noteId === noteId)
    if (tab) openNote(tab.notebookId, noteId)
  }

  function setEditorMode(mode: EditorMode) { editorMode.value = mode }
  function setSearchQuery(q: string)        { searchQuery.value = q  }

  // ── Getters ───────────────────────────────────────────────────
  const characterCount = computed(() => {
    if (!activeNote.value) return 0
    return activeNote.value.content.replace(/<[^>]*>/g, '').length
  })

  const filteredNotebooks = computed(() => {
    const q = searchQuery.value.trim().toLowerCase()
    if (!q) return notebooks.value
    return notebooks.value
      .map(nb => ({
        ...nb,
        notes: nb.notes.filter(n => n.title.toLowerCase().includes(q)),
      }))
      .filter(nb => nb.notes.length > 0 || nb.name.toLowerCase().includes(q))
  })

  return {
    notebooks, activeNote, tabs, activeNoteId,
    editorMode, searchQuery, loading, error,
    characterCount, filteredNotebooks,
    register, login, logout,
    fetchNotebooks, createNotebook, renameNotebook, deleteNotebook, toggleNotebook,
    fetchNotes, openNote, createNote, saveNote, deleteNote, updateContent,
    closeTab, setActiveTab,
    setEditorMode, setSearchQuery,
  }
})