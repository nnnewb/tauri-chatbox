import { createRouter, createWebHistory } from 'vue-router'
import WelcomePage from './components/WelcomePage.vue'
import ChatPage from './components/ChatPage.vue'
import SettingPage from './components/SettingPage.vue'

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