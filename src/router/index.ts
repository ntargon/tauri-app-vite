// import Canvas from './components/Canvas.vue'
import Canvas from '../components/Canvas.vue'
import Monitor from '../components/Monitor.vue'

import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'canvas',
    component: Canvas
  },
  {
    path: '/test',
    name: 'monitor',
    component: Monitor
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router;