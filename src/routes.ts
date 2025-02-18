import { createRouter, createWebHistory } from 'vue-router'
import WelcomePage from './WelcomePage.vue'
import ChatPage from './ChatPage.vue'
import SettingPage from './SettingPage.vue'

const routes = [
  {
    path: '/',
    component: WelcomePage
  },
  {
    path: '/chat/:id',
    component: ChatPage
  },
  {
    path: '/settings',
    component: SettingPage
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router