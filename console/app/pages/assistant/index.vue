<script setup lang="ts">
import { useIntelligenceStore } from "~/stores/intelligence";
import ProviderSetup from "~/components/assistant/ProviderSetup.vue";

definePageMeta({ layout: false });

const store = useIntelligenceStore();
const input = ref("");
const includeContext = ref(false);
const chatContainer = ref<HTMLElement>();

async function onSubmit() {
  const text = input.value.trim();
  if (!text) return;

  input.value = "";
  await store.sendMessage(text, includeContext.value);

  nextTick(() => {
    chatContainer.value?.scrollTo({
      top: chatContainer.value.scrollHeight,
      behavior: "smooth",
    });
  });
}

watch(
  () => store.messages.length,
  () => {
    nextTick(() => {
      chatContainer.value?.scrollTo({
        top: chatContainer.value.scrollHeight,
        behavior: "smooth",
      });
    });
  },
);

onMounted(async () => {
  await store.fetchConfig();
  if (store.configured) {
    await store.checkHealth();
  }
});
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <ProviderSetup v-if="!store.configured || !store.healthy" />

      <template v-else>
        <div class="flex flex-col h-full min-h-0">
          <!-- Header -->
          <div
            class="shrink-0 flex items-center justify-between px-4 py-3 border-b border-gray-200 dark:border-gray-800"
          >
            <div class="flex items-center gap-2">
              <UIcon
                name="ri:sparkling-fill"
                class="size-5 text-primary-500"
              />
              <h1 class="text-base font-semibold text-gray-900 dark:text-white">
                Assistant
              </h1>
            </div>
            <div class="flex items-center gap-2">
              <UTooltip text="Include app context (notes, tasks, bookmarks)">
                  <USwitch v-model="includeContext" size="sm" />
              </UTooltip>
              <span
                v-if="!includeContext"
                class="text-xs text-gray-500 dark:text-gray-400"
              >
                Context off
              </span>
              <span v-else class="text-xs text-primary-500"> Context on </span>
              <UButton
                v-if="store.messages.length > 0"
                variant="ghost"
                color="neutral"
                size="sm"
                icon="i-lucide-trash-2"
                @click="store.clearConversation()"
              />
            </div>
          </div>

          <!-- Messages -->
          <div
            ref="chatContainer"
            class="flex-1 overflow-y-auto px-4 py-6 space-y-4"
          >
            <div
              v-if="store.messages.length === 0"
              class="flex flex-col items-center justify-center h-full text-center"
            >
              <UIcon
                name="ri:sparkling-line"
                class="size-12 text-gray-300 dark:text-gray-600 mb-3"
              />
              <p class="text-sm text-gray-500 dark:text-gray-400">
                Ask anything. Toggle context to let the assistant see your
                notes, tasks, and bookmarks.
              </p>
            </div>

            <UChatMessage
              v-for="msg in store.messages"
              :id="msg.id"
              :key="msg.id"
              :role="msg.role"
              :parts="msg.parts"
              :side="msg.role === 'user' ? 'right' : 'left'"
              :variant="msg.role === 'user' ? 'soft' : 'naked'"
              :avatar="
                msg.role === 'assistant'
                  ? { icon: 'i-lucide-bot' }
                  : undefined
              "
            />
          </div>

          <!-- Input -->
          <div class="shrink-0 border-t border-gray-200 dark:border-gray-800 p-4">
            <UChatPrompt
              v-model="input"
              :status="store.status"
              placeholder="Message the assistant..."
              @submit="onSubmit"
            >
              <template #footer>
                <div class="flex justify-end">
                  <UChatPromptSubmit :status="store.status" />
                </div>
              </template>
            </UChatPrompt>
          </div>
        </div>
      </template>
    </template>
  </NuxtLayout>
</template>
