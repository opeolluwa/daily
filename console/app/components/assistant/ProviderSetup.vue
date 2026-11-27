<script setup lang="ts">
import { useIntelligenceStore } from "~/stores/intelligence";

const store = useIntelligenceStore();
const loading = ref(false);
const endpoint = ref("http://localhost:11434");
const model = ref("");

async function loadModels() {
  loading.value = true;
  try {
    await store.configure({
      kind: "ollama",
      model: "temp",
      endpoint: endpoint.value || undefined,
    });
    await store.fetchModels();
    if (store.models.length > 0) {
      model.value = store.models[0];
    }
  } finally {
    loading.value = false;
  }
}

async function connect() {
  loading.value = true;
  try {
    await store.configure({
      kind: "ollama",
      model: model.value,
      endpoint: endpoint.value || undefined,
    });
  } finally {
    loading.value = false;
  }
}

onMounted(loadModels);
</script>

<template>
  <div class="flex flex-col items-center justify-center h-full px-4">
    <div class="w-full max-w-sm space-y-6">
      <div class="text-center space-y-2">
        <UIcon name="ri:sparkling-fill" class="size-10 text-primary-500" />
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Connect AI Provider
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Set up a local LLM to power your assistant.
        </p>
      </div>

      <div class="space-y-4">
        <UFormField label="Provider">
          <UInput model-value="Ollama" class="w-full" disabled />
        </UFormField>

        <UFormField label="Endpoint">
          <UInput
            v-model="endpoint"
            placeholder="http://localhost:11434"
            class="w-full"
          />
        </UFormField>

        <UFormField label="Model">
          <USelect
            v-if="store.models.length > 0"
            v-model="model"
            :items="store.models"
            class="w-full"
          />
          <UInput
            v-else
            v-model="model"
            placeholder="qwen2.5:7b"
            class="w-full"
          />
        </UFormField>

        <UButton block :loading="loading" :disabled="!model" @click="connect">
          Connect
        </UButton>

        <p class="text-xs text-gray-500 dark:text-gray-400 text-center">
          OpenAI and Anthropic providers are not available yet.
        </p>

        <p
          v-if="store.error"
          class="text-sm text-red-500 dark:text-red-400 text-center"
        >
          {{ store.error }}
        </p>
      </div>
    </div>
  </div>
</template>
