import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('./views/MemeList.vue'),
    },
    {
      path: '/meme/:memeKey',
      name: 'meme',
      component: () => import('./views/MemeGenerator.vue'),
      props: true,
    },
  ],
})

export default router
