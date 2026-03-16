import RegisterView from '@/views/RegisterView.vue'
import EditorView from '@/views/EditorView.vue'
import LoginView from '@/views/LoginView.vue'
import { createRouter, createWebHistory } from 'vue-router'
import AccountSettingsView from '@/views/accountSettingsView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "Editor",
      component: EditorView
    },
    {
      path: "/register",
      name: "Create Account",
      component: RegisterView
    },
    {
      path: "/login",
      name: "Login Account",
      component: LoginView
    },
    {
      path: "/profile",
      name: "Account Setting",
      component: AccountSettingsView
    }
  ],
})

export default router

