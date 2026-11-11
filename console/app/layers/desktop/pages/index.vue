<script setup lang="ts">
import { useNoteStore } from "@shared/stores/notes";
import {
  useBookmarkStore,
  type BookmarkTag,
  type CreateBookmarkPayload,
} from "@shared/stores/bookmarks";
import { useTaskStore } from "@shared/stores/tasks";
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";
import TaskDialog from "@desktop/components/task/Dialog.vue";
import ReminderDialog from "@desktop/components/reminders/ReminderDialog.vue";
import BookmarkAddModal from "@desktop/components/bookmark/add-modal.vue";

definePageMeta({ layout: false });

const noteStore = useNoteStore();
const bookmarkStore = useBookmarkStore();
const taskStore = useTaskStore();
const userPreferenceStore = useUserPreferenceStore();

const router = useRouter();
const query = ref("");

const taskDialogOpen = ref(false);
const reminderDialogOpen = ref(false);
const bookmarkDialogOpen = ref(false);

const bookmarkTags: { label: string; value: BookmarkTag }[] = [
  { label: "Development", value: "development" },
  { label: "Design", value: "design" },
  { label: "Research", value: "research" },
  { label: "Inspiration", value: "inspiration" },
];

async function handleCreateBookmark(payload: CreateBookmarkPayload) {
  await bookmarkStore.createBookmark(payload);
}

const { setSearch, clearSearch } = useAppSearch();

onMounted(async () => {
  await Promise.all([
    noteStore.fetchNotes(),
    bookmarkStore.fetchBookmarks(),
    taskStore.fetchTasks(),
    userPreferenceStore.fetchPreference(),
  ]);
  setSearch({ placeholder: "Search everything…" });
});

onUnmounted(() => clearSearch());

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
          <!-- tint -->
          
        <div
          class="pointer-events-none absolute -top-20 -left-24 size-96 rounded-full bg-primary-300/20 dark:bg-primary-500/10 blur-3xl  md:block "
        />

        <!-- tint -->
        <div
          class="pointer-events-none absolute -bottom-24 -right-24 rounded-full bg-violet-300/20 dark:bg-violet-500/10 blur-3xl hidden md:block"
        />

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
            <p class="mt-3 text-sm text-gray-500 dark:text-gray-400">
              Your personal assistant — ready when you are
            </p>
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
              <UButton
                color="neutral"
                icon="heroicons:sparkles-solid"
                variant="link"
                class="text-primary pl-2 cursor-pointer font-medium"
                size="lg"
              />
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

        <div class="flex flex-wrap items-center justify-center gap-6 sm:gap-10">
          <button
            type="button"
            class="flex flex-col items-center justify-center gap-2 rounded-2xl px-4 py-3 text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800"
            @click="router.push('/notes/create-notes')"
          >
            <span
              class="flex size-14 items-center justify-center rounded-2xl border border-gray-50 bg-primary-50/40 dark:border-gray-800 dark:bg-gray-800/30"
            >
              <UIcon name="heroicons:document-plus" class="size-8" />
            </span>
            <span class="text-xs font-medium">Create note</span>
          </button>

          <button
            type="button"
            class="flex flex-col items-center justify-center gap-2 rounded-2xl px-4 py-3 text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800"
            @click="taskDialogOpen = true"
          >
            <span
              class="flex size-14 items-center justify-center rounded-2xl border border-gray-50 bg-primary-50/40 dark:border-gray-800 dark:bg-gray-800/30"
            >
              <UIcon name="heroicons:check-circle" class="size-8" />
            </span>
            <span class="text-xs font-medium">Add a task</span>
          </button>

          <button
            type="button"
            class="flex flex-col items-center justify-center gap-2 rounded-2xl px-4 py-3 text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800"
            @click="reminderDialogOpen = true"
          >
            <span
              class="flex size-14 items-center justify-center rounded-2xl border border-gray-50 bg-primary-50/40 dark:border-gray-800 dark:bg-gray-800/30"
            >
              <UIcon name="heroicons:bell" class="size-8" />
            </span>
            <span class="text-xs font-medium">Set a reminder</span>
          </button>

          <button
            type="button"
            class="flex flex-col items-center justify-center gap-2 rounded-2xl px-4 py-3 text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800"
            @click="bookmarkDialogOpen = true"
          >
            <span
              class="flex size-14 items-center justify-center rounded-2xl border border-gray-50 bg-primary-50/40 dark:border-gray-800 dark:bg-gray-800/30"
            >
              <UIcon name="heroicons:bookmark" class="size-8" />
            </span>
            <span class="text-xs font-medium">Save a link</span>
          </button>
        </div>
      </div>
    </template>

    <template #modal_content>
      <TaskDialog v-model:open="taskDialogOpen" />
      <ReminderDialog v-model:open="reminderDialogOpen" />
      <BookmarkAddModal
        v-model:open="bookmarkDialogOpen"
        :tags="bookmarkTags"
        @create="handleCreateBookmark"
      />
    </template>
  </NuxtLayout>
</template>
