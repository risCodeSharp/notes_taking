import axios from 'axios'
import router from '@/router'

const api = axios.create({
  baseURL: 'https://notes-y0ae.onrender.com',
  timeout: 10000,
  headers: { 'Content-Type': 'application/json' },
})

api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

api.interceptors.response.use(
  (res) => res,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token')

      if (router.currentRoute.value.name !== 'login') {
        router.push({ name: 'login' })
      }
    }

    return Promise.reject(error)
  }
)

export default api