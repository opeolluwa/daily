<script setup lang="ts">
const props = defineProps<{
  label: string;
  icon?: string;
  to?: string;
}>();

const emit = defineEmits<{
  click: [];
}>();

function handleFabClick() {
  if (props.to) {
    navigateTo(props.to);
    return;
  }

  emit("click");
}
</script>

<template>
  <!-- Desktop: labeled link or action button -->
  <NuxtLink
    v-if="to"
    :to="to"
    class="hidden md:flex items-center gap-2 py-2 px-4 bg-primary-500 text-white rounded-lg text-sm font-medium hover:bg-primary-600 transition-colors"
  >
    <UIcon v-if="icon" :name="icon" class="size-4" />
    {{ label }}
  </NuxtLink>

  <button
    v-else
    type="button"
    class="hidden md:flex items-center gap-2 py-2 px-4 bg-primary-500 text-white rounded-lg text-sm font-medium hover:bg-primary-600 transition-colors"
    @click="emit('click')"
  >
    <UIcon v-if="icon" :name="icon" class="size-4" />
    {{ label }}
  </button>

  <!-- Mobile: icon-only round FAB -->
  <RyderFab :icon="icon" :aria-label="label" @click="handleFabClick" />
</template>
