<script setup lang="ts">
import tabs, { settingsTabForPath } from "~/data/settings-tabs";

const route = useRoute();

const visibleTabs = computed(() => tabs);

const activeKey = computed<string | null>(() => {
  const current = settingsTabForPath(visibleTabs.value, route.path)?.key;
  if (current) return current;
  if (route.path === "/settings") return visibleTabs.value[0]?.key ?? null;
  return null;
});

const currentTabLabel = computed(
  () => settingsTabForPath(visibleTabs.value, route.path)?.label ?? "Settings",
);
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="flex items-start gap-6">
        <aside class="hidden w-60 shrink-0 lg:block">
          <div
            class="sticky top-0 rounded-2xl bg-white dark:bg-gray-800 border border-gray-100 dark:border-gray-700 p-2"
          >
            <p
              class="px-3 py-2 text-[10px] font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500"
            >
              Settings
            </p>

            <nav class="space-y-0.5">
              <NuxtLink
                v-for="tab in visibleTabs"
                :key="tab.key"
                :to="tab.to"
                class="flex w-full cursor-pointer items-center gap-3 rounded-xl px-3 py-2.5 text-left text-sm transition-all"
                :class="
                  activeKey === tab.key
                    ? 'bg-primary-50 dark:bg-primary-950 text-primary-600 dark:text-primary-400 font-medium'
                    : 'text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700/50 hover:text-gray-800 dark:hover:text-white'
                "
              >
                <UIcon :name="tab.icon" class="size-4 shrink-0" />

                <div class="min-w-0">
                  <p class="leading-tight">{{ tab.label }}</p>

                  <p
                    class="mt-0.5 truncate text-[10px] leading-tight"
                    :class="
                      activeKey === tab.key
                        ? 'text-primary-500/60 dark:text-primary-400/60'
                        : 'text-gray-400 dark:text-gray-500'
                    "
                  >
                    {{ tab.desc }}
                  </p>
                </div>
              </NuxtLink>
            </nav>
          </div>
        </aside>

        <RyderCard
          class="min-w-0 max-w-xl flex-1 lg:rounded-xl lg:bg-white lg:p-4 lg:dark:bg-gray-800"
        >
          <!-- Mobile back bar -->
          <div
            v-if="route.path !== '/settings'"
            class="lg:hidden mb-3 flex items-center gap-2"
          >
            <UButton
              icon="heroicons:arrow-left"
              size="sm"
              color="neutral"
              variant="ghost"
              @click="navigateTo('/settings')"
            />
            <span
              class="text-sm font-medium text-gray-700 dark:text-gray-200"
            >
              {{ currentTabLabel }}
            </span>
          </div>

          <slot />
        </RyderCard>
      </div>
    </template>
  </NuxtLayout>
</template>
