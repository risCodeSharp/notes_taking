import RegisterView from '@/views/RegisterView.vue'
import EditorView from '@/views/EditorView.vue'
import LoginView from '@/views/LoginView.vue'
// 1. Change createWebHistory to createWebHashHistory
import { createRouter, createWebHashHistory } from 'vue-router' 
import AccountSettingsView from '@/views/accountSettingsView.vue'

const router = createRouter({
  // 2. Update this line to use the hash history
  history: createWebHashHistory(import.meta.env.BASE_URL),
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