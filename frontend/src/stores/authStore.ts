// src/stores/authStore.ts
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { authService } from '@/services/authService'
import api from '@/services/api'
import type { LoginPayload, CreateUserPayload } from '@/types'

// Matches Rust UserPublic { id, email } — no name field
interface AuthUser {
  id: number
  email: string
}

const USER_KEY = 'auth_user'

export const useAuthStore = defineStore('auth', () => {
  const _stored   = localStorage.getItem(USER_KEY)
  const user      = ref<AuthUser | null>(_stored ? JSON.parse(_stored) : null)
  const meLoading = ref(false)

  // ── Getters ──────────────────────────────────────────────────
  const isLoggedIn  = computed(() => !!localStorage.getItem('token'))
  // UserPublic has no name — use email local-part as display name
  const displayName = computed(() => {
    if (!user.value?.email) return ''
    return user.value.email.split('@')[0]
  })
  const email    = computed(() => user.value?.email ?? '')
  const initials = computed(() => {
    const src = user.value?.email ?? ''
    return src.charAt(0).toUpperCase() || '?'
  })

  // ── Helpers ──────────────────────────────────────────────────
  function _persist(u: AuthUser) {
    user.value = u
    localStorage.setItem(USER_KEY, JSON.stringify(u))
  }

  function _clear() {
    user.value = null
    localStorage.removeItem(USER_KEY)
    localStorage.removeItem('token')
  }

  // ── Actions ──────────────────────────────────────────────────

  // GET /api/users/me
  // Rust returns: ApiResponse<UserPublic>
  // Shape: { success, message, data: { id, email } }
  async function fetchMe(): Promise<void> {
    if (!localStorage.getItem('token')) return
    meLoading.value = true
    try {
      const res = await api.get<any>('/api/users/me')
      // data is UserPublic: { id, email }
      const u: AuthUser = res.data?.data
      if (u?.id && u?.email) _persist(u)
    } catch (e) {
      console.error('[authStore] fetchMe failed:', e)
    } finally {
      meLoading.value = false
    }
  }

  async function login(payload: LoginPayload) {
    const authData = await authService.login(payload)
    if (authData.user?.id) _persist(authData.user)
    await fetchMe()
    return authData
  }

  async function register(payload: CreateUserPayload) {
    const res  = await authService.register(payload)
    const body = res.data as any
    const u: AuthUser = body?.data?.user ?? body?.user ?? null
    if (u) _persist(u)
    return res
  }

  function logout() {
    authService.logout()
    _clear()
  }

  return { user, meLoading, isLoggedIn, displayName, email, initials, fetchMe, login, register, logout }
})