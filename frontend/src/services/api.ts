// src/services/api.ts
import axios from 'axios'
import router from '@/router'

const api = axios.create({
  baseURL: 'https://notes-y0ae.onrender.com',
  timeout: 10000,
  headers: { 'Content-Type': 'application/json' },
})

// ── Attach token to every protected request ───────────────────
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')

  if (token && config.headers) {
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

      // Prevent redirect loop
      const current = window.location.hash

      if (current !== '#/login') {
        // ✅ GitHub Pages safe redirect
        window.location.hash = '#/login'
      }
    }

    return Promise.reject(error)
  }
)

export default api