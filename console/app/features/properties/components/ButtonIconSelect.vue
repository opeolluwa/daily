<script setup lang="ts" generic="T extends string | number">

const { options, modelValue } = defineProps<{
  options: { label: string; value: T; icon?: string; iconName?: string }[];
  modelValue: T | "mixed" | null;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: T];
}>();

function isActive(optionValue: T): boolean {
  if (modelValue === "mixed" || modelValue === null) return false;
  return modelValue === optionValue;
}
</script>

<template>
  <div class="flex gap-0.5 rounded-md bg-base/40 p-0.5 border border-edge/10">
    <UButton
      v-for="option in options"
      :key="option.value"
      variant="ghost"
      color="neutral"
      size="xs"
      square
      :aria-label="option.label"
      :aria-pressed="isActive(option.value)"
      :class="
        isActive(option.value)
          ? 'bg-accent/20 text-accent hover:bg-accent/30'
          : 'text-foreground/70 hover:bg-subdued/20 hover:text-foreground'
      "
      @click="emit('update:modelValue', option.value)"
    >
      <UIcon
        v-if="option.iconName"
        :name="option.iconName"
        class="h-4 w-4"
      />
      <!-- eslint-disable vue/no-v-html -->
      <svg
        v-else-if="option.icon"
        aria-hidden="true"
        class="h-4 w-4"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        v-html="option.icon!"
      />
      <!-- eslint-enable vue/no-v-html -->
      <span v-else class="leading-none">{{ option.label }}</span>
    </UButton>
  </div>
</template>
