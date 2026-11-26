<script setup lang="ts">
import { useNoteStore } from "~/stores/notes";
import { useUserPreferenceStore } from "~/stores/workspace-profile";
import { tourConfig } from "~/data/tour-steps";

definePageMeta({ layout: false });

const noteStore = useNoteStore();
const userPreferenceStore = useUserPreferenceStore();

const query = ref("");

const { setSearch, clearSearch } = useAppSearch();

const { start: startTour } = useDriver("onboarding", { autoSkip: true });

onMounted(async () => {
  await Promise.all([
    noteStore.fetchNotes(),
    userPreferenceStore.fetchPreference(),
  ]);
  setSearch({ placeholder: "Search everything…" });

  await nextTick();
  setTimeout(() => startTour(tourConfig), 500);
});

onUnmounted(() => {
  clearSearch();
});

const userName = computed(
  () => userPreferenceStore.preference?.firstName || "there",
);

const now = ref(new Date());
let timer: ReturnType<typeof setInterval>;

onMounted(() => {
  timer = setInterval(() => {
    now.value = new Date();
  }, 60_000);
});
onUnmounted(() => clearInterval(timer));

const greeting = computed(() => {
  const h = now.value.getHours();
  if (h < 12) return "Good morning";
  if (h < 17) return "Good afternoon";
  return "Good evening";
});

const today = computed(() =>
  now.value.toLocaleDateString("en-US", {
    weekday: "long",
    month: "long",
    day: "numeric",
  }),
);
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div
        class="-mx-6 -mt-5 -mb-6 flex min-h-0 w-full flex-col items-center justify-center flex-1 gap-6 px-5 relative"
      >
        <div
          class="flex w-full flex-col items-center justify-center gap-4 text-center"
        >
          <div>
            <p
              class="text-xs font-semibold tracking-widest uppercase text-gray-400 dark:text-gray-500 mb-1.5"
            >
              {{ today }}
            </p>
            <h1
              class="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl dark:text-white"
            >
              {{ greeting }}, <span class="capitalize">{{ userName }}</span>
            </h1>
          </div>
        </div>

        <UForm class="w-full max-w-xl">
          <UInput
            v-model="query"
            class="w-full max-w-xl"
            icon="heroicons:plus"
            size="lg"
            placeholder="Search notes, bookmarks, tasks…"
            :ui="{
              root: 'pl-3',
              base: 'py-4 shadow-sm bg-transparent placeholder:text-sm text-sm rounded-full',
              leadingIcon: 'text-primary size-5',
            }"
          >
            <template #leading>
              <UTooltip text="attach files">
                <UButton
                  color="neutral"
                  icon="heroicons:plus-solid"
                  variant="link"
                  class="pl-2 cursor-pointer font-bold"
                  size="lg"
                />
              </UTooltip>
            </template>

            <template #trailing>
              <UButton
                type="submit"
                color="neutral"
                icon="heroicons:arrow-long-right"
                variant="link"
                class="bg-primary-300 rounded-full text-white"
                size="lg"
              />
            </template>
          </UInput>
        </UForm>
      </div>
    </template>
  </NuxtLayout>
</template>
