// src/services/api.ts
import axios from 'axios'

const api = axios.create({
  baseURL: 'https://notes-y0ae.onrender.com',
  timeout: 10000,
  headers: { 'Content-Type': 'application/json' },
})

// ── Attach token to every protected request ───────────────────
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// ── Global response error handling ────────────────────────────
api.interceptors.response.use(
  (res) => res,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token')
      if (!window.location.pathname.includes('/login')) {
        window.location.href = '/login'
      }
    }
    return Promise.reject(error)
  }
)

export default api