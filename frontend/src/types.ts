
export type EditorMode = 'edit' | 'preview' | 'split'
// src/types/index.ts

// ── Generic API wrapper ───────────────────────────────────────
// Your Rust backend wraps responses like:
// { success: true, message: "...", data: { ... } }
export interface ApiResponse<T> {
  success: boolean
  message: string
  data: T
}

// ── Auth ──────────────────────────────────────────────────────
export interface CreateUserPayload {
  name: string
  email: string
  password: string
}

export interface LoginPayload {
  email: string
  password: string
}

export interface UserPublic {
  id: number
  email: string
}

// What comes back inside data on login/register
export interface AuthData {
  token: string
  user: UserPublic
}

// ── Notebook ──────────────────────────────────────────────────
export interface NotebookResponse {
  id: number
  name: string
}

export interface NotebookPayload {
  name: string
}

// ── Note ──────────────────────────────────────────────────────
export interface Note {
  id: number
  owner_id: number
  title: string
  content: string
  visibility: string
  last_editor_id: number
  updated_at: string
}

export interface NoteResponse {
  id: number
  title: string
  notebook_id: number
}

export interface CreateNotePayload {
  title: string
  content: string
}

export interface UpdateNotePayload {
  title?: string
  content?: string
  visibilty?: string   // matches Rust field name (typo intentional)
  notebook_id?: number
}