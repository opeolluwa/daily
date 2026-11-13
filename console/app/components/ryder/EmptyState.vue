<script setup lang="ts">
import BaseEmptyState from "@opeolluwa/ryder/components/feedback/EmptyState.vue";

interface Props {
  title: string;
  description: string;
  icon?: string;
  actionLabel?: string;
  compact?: boolean;
  ui?: {
    root?: string;
    title?: string;
    description?: string;
    iconWrapper?: string;
    icon?: string;
    actionLabel?: string;
  };
}

const props = withDefaults(defineProps<Props>(), {
  icon: "heroicons:users",
  compact: false,
  ui: () => ({}),
});

const emit = defineEmits<{ action: [] }>();

const mergedUi = computed(() => ({
  ...props.ui,
  iconWrapper: ["bg-primary-50", props.ui.iconWrapper]
    .filter(Boolean)
    .join(" "),
  actionLabel: ["dark:text-white", props.ui.actionLabel]
    .filter(Boolean)
    .join(" "),
}));
</script>

<template>
  <BaseEmptyState
    :title="title"
    :description="description"
    :icon="icon"
    :action-label="actionLabel"
    :compact="compact"
    :ui="mergedUi"
    @action="emit('action')"
  />
</template>
