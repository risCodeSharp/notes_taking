// src/services/noteService.ts
import api from "./api";
import type {
  ApiResponse,
  Note,
  NoteResponse,
  CreateNotePayload,
  UpdateNotePayload,
} from "@/types";

function unwrap<T>(res: { data: ApiResponse<T> }): T {
  return res.data.data;
}

export const noteService = {
  // POST /api/notebooks/:notebookId/notes
 // noteService.ts
create: async (notebookId: number, payload: CreateNotePayload): Promise<Note> => {
  const res = await api.post<ApiResponse<Note>>(
    `/api/notebooks/${notebookId}/notes`, payload
  )
  return unwrap(res)
},

  // GET /api/notebooks/:notebookId/notes
  list: async (notebookId: number): Promise<Note[]> => {
    const res = await api.get<ApiResponse<Note[]>>(
      `/api/notebooks/${notebookId}/notes`
    );
    return unwrap(res);
  },

  // GET /api/notebooks/:notebookId/notes/:noteId
  get: async (notebookId: number, noteId: number): Promise<Note> => {
    const res = await api.get<ApiResponse<Note>>(
      `/api/notebooks/${notebookId}/notes/${noteId}`
    );
    return unwrap(res);
  },

  // PUT /api/notebooks/:notebookId/notes/:noteId
  update: async (
    notebookId: number,
    noteId: number,
    payload: UpdateNotePayload
  ): Promise<Note> => {
    const res = await api.put<ApiResponse<Note>>(
      `/api/notebooks/${notebookId}/notes/${noteId}`,
      payload
    );
    return unwrap(res);
  },

  // DELETE /api/notebooks/:notebookId/notes/:noteId
  delete: (notebookId: number, noteId: number) =>
    api.delete(`/api/notebooks/${notebookId}/notes/${noteId}`),
};
