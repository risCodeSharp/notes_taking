// src/services/notebookService.ts
import api from './api'
import type { ApiResponse, NotebookResponse, NotebookPayload } from '@/types'

// Helper: unwrap { success, data: T } → T
function unwrap<T>(res: { data: ApiResponse<T> }): T {
  return res.data.data
}

export const notebookService = {

  // POST /api/notebooks  → NotebookResponse
  create: async (payload: NotebookPayload): Promise<NotebookResponse> => {
    const res = await api.post<ApiResponse<NotebookResponse>>('/api/notebooks', payload)
    return unwrap(res)
  },

  // GET /api/notebooks  → NotebookResponse[]
  list: async (): Promise<NotebookResponse[]> => {
    const res = await api.get<ApiResponse<NotebookResponse[]>>('/api/notebooks')
    return unwrap(res)
  },

  // GET /api/notebooks/:id  → NotebookResponse
  get: async (notebookId: number): Promise<NotebookResponse> => {
    const res = await api.get<ApiResponse<NotebookResponse>>(`/api/notebooks/${notebookId}`)
    return unwrap(res)
  },

  // PUT /api/notebooks/:id  → NotebookResponse
  update: async (notebookId: number, payload: NotebookPayload): Promise<NotebookResponse> => {
    const res = await api.put<ApiResponse<NotebookResponse>>(`/api/notebooks/${notebookId}`, payload)
    return unwrap(res)
  },

  // DELETE /api/notebooks/:id
  delete: (notebookId: number) =>
    api.delete(`/api/notebooks/${notebookId}`),
}