// src/services/authService.ts
import api from './api'
import type { ApiResponse, AuthData, LoginPayload, CreateUserPayload } from '@/types'

export const authService = {

  // POST /api/users/register
  register: (payload: CreateUserPayload) =>
    api.post<ApiResponse<AuthData>>('/api/users/register', payload),

  // POST /api/users/login
  // Rust returns: ApiResponse<AuthResponse>
  // Shape: { success, message, data: { token: string, user: { id, email } } }
  login: async (payload: LoginPayload): Promise<AuthData> => {
    const res  = await api.post<any>('/api/users/login', payload)
    const data = res.data?.data  // unwrap ApiResponse → AuthResponse

    const token: string        = data?.token
    const user: { id: number, email: string } = data?.user ?? null

    if (!token) throw new Error('No token in login response')

    localStorage.setItem('token', token)
    if (user) localStorage.setItem('auth_user', JSON.stringify(user))

    return { token, user } as AuthData
  },

  logout: () => {
    localStorage.removeItem('token')
    localStorage.removeItem('auth_user')
  },
}