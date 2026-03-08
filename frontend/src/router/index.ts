import CreateView from '@/views/CreateView.vue'
import EditorView from '@/views/EditorView.vue'
import LoginView from '@/views/LoginView.vue'
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "Editor",
      component: EditorView
    },
    {
      path: "/create",
      name: "Create Account",
      component: CreateView
    },
    {
      path: "/login",
      name: "Login Account",
      component: LoginView
    },
  ],
})

export default router

