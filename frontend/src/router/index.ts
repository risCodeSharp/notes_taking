import RegisterView from '@/views/RegisterView.vue'
import EditorView from '@/views/EditorView.vue'
import LoginView from '@/views/LoginView.vue'
import AccountSettingsView from '@/views/AccountSettingsView.vue'

import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      name: 'Editor',
      component: EditorView
    },
    {
      path: '/register',
      name: 'Register',
      component: RegisterView
    },
    {
      path: '/login',
      name: 'Login',
      component: LoginView
    },
    {
      path: '/profile',
      name: 'Profile',
      component: AccountSettingsView
    }
  ]
})

export default router