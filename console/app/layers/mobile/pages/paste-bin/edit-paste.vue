<script setup lang="ts">
import PasteBinForm from "@mobile/components/paste-bin/form.vue";
import { usePasteBinStore } from "@shared/stores/paste-bin";

definePageMeta({ name: "Edit paste" });

const route = useRoute();
const router = useRouter();
const pasteBinStore = usePasteBinStore();
const { notify } = useAppNotification();

const id = computed(() =>
  typeof route.query.id === "string" ? route.query.id : undefined,
);

const paste = computed(
  () =>
    pasteBinStore.pasteBins.find((entry) => entry.identifier === id.value) ??
    null,
);

function handleSaved() {
  notify({ message: "Paste updated", type: "success" });
  router.replace("/paste-bin");
}

onMounted(async () => {
  if (pasteBinStore.pasteBins.length === 0) {
    await pasteBinStore.fetchPasteBins();
  }
});
</script>

<template>
  <div>
    <div v-if="pasteBinStore.loading && !paste" class="flex flex-col gap-4">
      <USkeleton class="h-16 rounded-lg" />
      <USkeleton class="h-56 rounded-lg" />
      <USkeleton class="h-10 rounded-lg" />
    </div>

    <div
      v-else-if="!paste"
      class="flex flex-col items-center justify-center py-20 text-center"
    >
      <div class="mb-4 p-3 rounded-full bg-gray-100 dark:bg-gray-800">
        <UIcon name="heroicons:clipboard" class="size-7 text-gray-400" />
      </div>
      <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Paste not found
      </h3>
      <button
        class="text-xs text-primary-500 hover:text-primary-600 font-medium mt-2"
        @click="router.push('/paste-bin')"
      >
        Back to paste bin
      </button>
    </div>

    <PasteBinForm v-else :paste="paste" @saved="handleSaved" />
  </div>
</template>
