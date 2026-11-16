<script setup lang="ts">
import { deriveTitle } from "~/utils/markdown";

definePageMeta({ layout: false, name: "Edit Paste" });

const route = useRoute();
const router = useRouter();
const pasteBinStore = usePasteBinStore();
const toast = useToast();

const id = computed(() => route.query.id as string | undefined);
const paste = computed(() => pasteBinStore.pasteBins.find((p) => p.identifier === id.value) ?? null);

const form = reactive({ title: "", content: "", isPinned: false });
const titleDirty = ref(false);
const submitting = ref(false);

function hydrate() {
  if (!paste.value) return;
  const derived = deriveTitle(paste.value.content);
  titleDirty.value = !!paste.value.title && paste.value.title !== derived;
  form.title = paste.value.title ?? derived;
  form.content = paste.value.content;
  form.isPinned = paste.value.isPinned;
}

watch(paste, hydrate, { immediate: true });
watch(() => form.content, (value) => { if (!titleDirty.value) form.title = deriveTitle(value); });

onMounted(async () => {
  if (pasteBinStore.pasteBins.length === 0) await pasteBinStore.fetchPasteBins();
  hydrate();
});

function handleTitleInput() { titleDirty.value = true; }

async function handleSubmit() {
  if (!paste.value || !form.content.trim()) return;
  submitting.value = true;
  try {
    await pasteBinStore.updatePasteBin(paste.value.identifier, {
      title: form.title.trim() || null,
      content: form.content,
      isPinned: form.isPinned,
    });
    toast.add({ color: "success", icon: "heroicons:check-circle", title: "Paste Updated", description: "Your changes have been saved" });
    await router.push("/paste-bin");
  } catch {
    toast.add({ color: "error", icon: "heroicons:x-circle", title: "Something Went Wrong", description: "Failed to update paste" });
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-3xl">
        <div class="mb-6 flex items-center gap-3">
          <h1 class="text-lg font-semibold text-gray-800 dark:text-gray-100">Edit Paste</h1>
        </div>

        <div v-if="!paste" class="flex flex-col items-center justify-center py-20 text-center">
          <UIcon name="ri:clipboard-line" class="size-7 text-gray-400 mb-3" />
          <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">Paste not found</p>
          <UButton size="sm" variant="outline" @click="router.push('/paste-bin')">Back to paste bin</UButton>
        </div>

        <form v-else class="flex flex-col gap-4" @submit.prevent="handleSubmit">
          <RyderInput v-model="form.title" label="Title" type="text" name="paste-title" placeholder="Untitled paste" :disabled="submitting" @input="handleTitleInput" />
          <div class="rounded-md border border-gray-200 dark:border-gray-700">
            <UEditor v-model="form.content" content-type="markdown" :placeholder="{ placeholder: 'Paste or type anything...', mode: 'firstLine' }" :editable="!submitting" :ui="{ base: 'px-4 py-3 sm:px-4', content: 'min-h-64 max-h-[50vh] overflow-y-auto' }" />
          </div>
          <div class="flex items-center justify-between">
            <label for="paste-pin" class="text-sm font-medium text-gray-700 dark:text-gray-300">Pin this paste</label>
            <USwitch id="paste-pin" v-model="form.isPinned" :disabled="submitting" />
          </div>
          <div class="flex justify-end gap-2 pt-2">
            <UButton type="button" color="neutral" variant="ghost" :disabled="submitting" @click="router.back()">Cancel</UButton>
            <UButton type="submit" :loading="submitting" :disabled="!form.content.trim()">Save</UButton>
          </div>
        </form>
      </div>
    </template>
  </NuxtLayout>
</template>
