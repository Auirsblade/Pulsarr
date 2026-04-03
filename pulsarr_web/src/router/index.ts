import { createRouter, createWebHistory } from 'vue-router'
import ActivityFeedView from '../views/ActivityFeedView.vue'
import { useContextStore } from '@/stores/context'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: ActivityFeedView,
    },
    {
      path: '/groups',
      name: 'groups',
      component: () => import('../views/GroupsView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/AboutView.vue'),
    },
    {
      path: '/group/:groupId',
      name: 'group',
      component: () => import('../views/GroupView.vue'),
    },
    {
      path: '/join/:groupId',
      name: 'join-group',
      component: () => import('../views/JoinGroupView.vue'),
    },
    {
      path: '/reset-password',
      name: 'reset-password',
      component: () => import('../views/ResetPasswordView.vue'),
    },
    {
      path: '/profile',
      name: 'profile',
      component: () => import('../views/ProfileView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/history',
      name: 'rating-history',
      component: () => import('../views/RatingHistoryView.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/user/:userId',
      name: 'user-profile',
      component: () => import('../views/UserProfileView.vue'),
      meta: { requiresAuth: true },
    },
  ],
})

let sessionRestored = false;

router.beforeEach(async (to, _from, next) => {
  const contextStore = useContextStore();

  if (!sessionRestored) {
    await contextStore.getSession();
    sessionRestored = true;
  }

  if (to.meta.requiresAuth && !contextStore.isLoggedIn) {
    contextStore.showSignInDialog = true;
    next('/');
    return;
  }

  next();
});

export default router
