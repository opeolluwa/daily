<script setup lang="ts">
withDefaults(
  defineProps<{
    title: string;
    message?: string;
    icon?: string;
    confirmLabel?: string;
    cancelLabel?: string;
    confirmColor?:
      | "primary"
      | "secondary"
      | "success"
      | "info"
      | "warning"
      | "error"
      | "neutral";
    loading?: boolean;
  }>(),
  {
    message: "",
    icon: "heroicons:exclamation-triangle",
    confirmLabel: "Confirm",
    cancelLabel: "Cancel",
    confirmColor: "warning",
    loading: false,
  },
);

const open = defineModel<boolean>("open", { required: true });

const emit = defineEmits<{ confirm: []; cancel: [] }>();

function handleCancel() {
  open.value = false;
  emit("cancel");
}
</script>

<template>
  <UModal v-model:open="open">
    <template #content>
      <div class="px-6 py-6 flex flex-col gap-5">
        <div class="flex items-start gap-4">
          <div
            class="shrink-0 size-10 rounded-full bg-orange-100 dark:bg-orange-950 flex items-center justify-center"
          >
            <UIcon
              :name="icon"
              class="size-5 text-orange-300 dark:text-orange-400"
            />
          </div>
          <div class="flex flex-col gap-1">
            <h2 class="text-base font-semibold text-gray-900 dark:text-white">
              {{ title }}
            </h2>
            <slot>
              <p class="text-sm text-gray-500 dark:text-gray-400">
                {{ message }}
              </p>
            </slot>
          </div>
        </div>

        <div class="flex justify-end gap-2">
          <RyderButton
            size="sm"
            variant="ghost"
            :disabled="loading"
            @click="handleCancel"
          >
            {{ cancelLabel }}
          </RyderButton>
          <RyderButton
            size="sm"
            :color="confirmColor"
            :loading="loading"
            @click="emit('confirm')"
          >
            {{ confirmLabel }}
          </RyderButton>
        </div>
      </div>
    </template>
  </UModal>
</template>
