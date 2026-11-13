<script setup lang="ts">
import { useMoodboardStore } from "~/stores/moodboard";

definePageMeta({ layout: false });

const moodboardStore = useMoodboardStore();
const fileInputRef = ref<HTMLInputElement | null>(null);

onMounted(() => {
  moodboardStore.fetchImages();
});

function triggerUpload() {
  fileInputRef.value?.click();
}

async function handleFileChange(event: Event) {
  const files = (event.target as HTMLInputElement).files;
  if (!files || files.length === 0) return;

  for (const file of Array.from(files)) {
    const buffer = await file.arrayBuffer();
    const bytes = Array.from(new Uint8Array(buffer));
    await moodboardStore.saveImage(file.name, bytes);
  }

  if (fileInputRef.value) fileInputRef.value.value = "";
}

async function handleDelete(filename: string) {
  await moodboardStore.deleteImage(filename);
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <RyderPageHeader
        v-if="moodboardStore.images.length > 0"
        title="Moodboard"
        cta-text="Add Image"
        cta-icon="heroicons:plus"
        @cta="triggerUpload"
      />
      <RyderFab
        aria-label="Add Image"
        @click="triggerUpload"
      />
      <input
        ref="fileInputRef"
        type="file"
        accept="image/*"
        multiple
        class="hidden"
        @change="handleFileChange"
      >
      <!-- Loading skeleton -->
      <div v-if="moodboardStore.loading" class="columns-2 gap-3 space-y-3">
        <div
          v-for="i in 4"
          :key="i"
          class="break-inside-avoid rounded-lg bg-gray-100 dark:bg-gray-800 animate-pulse"
          :style="{ height: `${120 + (i % 3) * 60}px` }"
        />
      </div>

      <RyderEmptyState
        v-else-if="moodboardStore.images.length === 0"
        title="No images yet"
        description="Add images to start building your moodboard."
        icon="heroicons:photo"
        action-label="Add Image"
        @action="triggerUpload"
      />

      <!-- Masonry grid -->
      <div v-else class="columns-2 gap-3 space-y-3">
        <RyderCard
          v-for="image in moodboardStore.images"
          :key="image.filename"
          :ui="{
            root: '!p-0 break-inside-avoid overflow-hidden hover:shadow-sm transition-shadow cursor-pointer group',
          }"
        >
          <div class="relative">
            <img
              :src="image.src"
              :alt="image.title"
              class="w-full object-cover"
            >
            <div
              class="absolute inset-0 bg-black/0 group-hover:bg-black/30 transition-colors flex items-end justify-between"
            >
              <p
                class="text-white text-xs font-medium p-3 opacity-0 group-hover:opacity-100 transition-opacity truncate"
              >
                {{ image.title }}
              </p>
              <button
                class="opacity-0 group-hover:opacity-100 transition-opacity m-2 p-1.5 rounded-lg bg-black/40 hover:bg-red-500/80 text-white"
                title="Delete image"
                @click.stop="handleDelete(image.filename)"
              >
                <UIcon name="heroicons:trash" class="size-3.5" />
              </button>
            </div>
          </div>
        </RyderCard>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Boards
      </h2>
      <div
        class="flex flex-col items-center justify-center py-6 text-center mb-6"
      >
        <UIcon
          name="heroicons:squares-2x2"
          class="size-6 text-gray-300 dark:text-gray-600 mb-2"
        />
        <p class="text-xs text-gray-400 dark:text-gray-500">No boards yet</p>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Tags
      </h2>
      <p class="text-xs text-gray-400 dark:text-gray-500">No tags yet</p>
    </template>
  </NuxtLayout>
</template>
