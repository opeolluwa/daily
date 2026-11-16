import { useAuthStore } from "~/stores/auth";

export default defineNuxtRouteMiddleware((to) => {
  if (to.path.startsWith("/auth")) return;

  const authStore = useAuthStore();
  if (authStore.isAuthenticated || authStore.isGuest) return;

  return navigateTo("/auth/login");
});
