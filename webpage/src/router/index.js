import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Submit from '../views/Submit.vue';
import Result from '../views/Result.vue';
import NotFound from '../views/NotFound.vue';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/submit',
    name: 'Submit',
    component: Submit,
  },
  {
    path: '/tasks/:taskId',
    name: 'Result',
    component: Result,
    props: true, // 将路由参数作为 props 传递给组件
  },
    {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    component: NotFound,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;