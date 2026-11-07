<script lang="ts" setup>
import {
  mobileBottomNavRoutes,
  primaryRoutes,
  secondaryRoutes,
} from "@shared/data/routes";
import { kNavbar, kPage } from "konsta/vue";
import _ from "lodash";
const route = useRoute();
const router = useRouter();
const { toggleMobileNav } = useMobileNav();

const topLevelPaths = [...primaryRoutes, ...secondaryRoutes].map(
  (item) => item.path,
);
const isTopLevel = computed(() => topLevelPaths.includes(route.path));
const bottomNavItems = mobileBottomNavRoutes.map((item) => ({
  label: item.name,
  to: item.path,
  icon: item.icon,
  activeIcon: item.activeIcon,
}));
const pageTitle = computed(() => {
  const raw = route.name?.toString().replaceAll("-", " ") ?? "";
  return raw
    .split(" ")
    .map((w) => _.capitalize(w))
    .join(" ");
});
</script>

<template>
  <kPage class="h-dvh overflow-hidden flex flex-col">
    <kNavbar
      :title="isTopLevel ? undefined : pageTitle"
      :center-title="false"
      title-class="truncate text-md pl-4 font-medium"
      bg-class="bg-white dark:bg-app-dark-800"
      class="shrink-0 px-2"
    >
      <template #left>
        <RyderButton
          v-if="isTopLevel"
          size="md"
          color="neutral"
          variant="ghost"
          aria-label="Open menu"
          @click="toggleMobileNav"
        >
          <UIcon name="heroicons:bars-3" class="size-5" />
        </RyderButton>

        <button v-else class="inline-flex items-center" @click="router.back()">
          <UIcon name="lucide:arrow-left" class="size-5" />
        </button>
      </template>

      <template #right>
        <AppHeaderRightPartial />
      </template>
    </kNavbar>

    <main class="flex min-h-0 flex-1 flex-col overflow-hidden">
      <AppViewport class="bg-white/90 dark:bg-app-dark-800">
        <slot />
      </AppViewport>
      <RyderBottomNav
        :items="bottomNavItems"
        bg-class="bg-white dark:bg-app-dark-800"
      />
      <AppSideNav />
    </main>
  </kPage>
</template>
