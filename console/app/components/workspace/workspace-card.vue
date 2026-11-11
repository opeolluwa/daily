<script setup lang="ts">
import type { Workspace } from "~/stores/workspaces";
import type { DropdownMenuItem } from "@nuxt/ui";

const props = defineProps<{ workspace: Workspace }>();

const emit = defineEmits<{
  delete: [identifier: string];
  edit: [identifier: string];
  toggleHidden: [identifier: string];
  setDefault: [identifier: string];
  toggleSecured: [identifier: string];
}>();

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

const extraItems = computed<DropdownMenuItem[]>(() => [
  {
    label: props.workspace.isHidden ? "Show workspace" : "Hide workspace",
    icon: props.workspace.isHidden ? "heroicons:eye" : "heroicons:eye-slash",
    class: "text-gray-700 dark:text-gray-300",
    onSelect: () => emit("toggleHidden", props.workspace.identifier),
  },
  ...(!props.workspace.isDefault
    ? [
        {
          label: "Set as default",
          icon: "heroicons:star",
          class: "text-gray-700 dark:text-gray-300",
          onSelect: () => emit("setDefault", props.workspace.identifier),
        },
      ]
    : []),
]);
</script>

<template>
  <RyderCard
    :ui="{
      root: 'group flex items-center gap-4 hover:shadow-sm transition-shadow',
    }"
  >
    <UIcon
      name="heroicons:briefcase"
      class="size-5 text-primary-500 shrink-0"
    />
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2">
        <h3
          class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate"
        >
          {{ workspace.name }}
        </h3>
        <span
          v-if="workspace.isDefault"
          class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-primary-100 dark:bg-primary-900 text-primary-600 dark:text-primary-300 shrink-0"
        >
          default
        </span>
        <span
          v-if="workspace.isHidden"
          class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 shrink-0"
        >
          hidden
        </span>
        <span
          v-if="workspace.isSecured"
          class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-amber-100 dark:bg-amber-900 text-amber-600 dark:text-amber-300 shrink-0"
        >
          secured
        </span>
      </div>
      <div class="text-xs text-gray-400 truncate block">
        {{ workspace.description }}
      </div>
    </div>
    <p class="text-xs text-gray-400 shrink-0 hidden sm:block">
      {{ formatDate(workspace.createdAt) }}
    </p>
    <div
      class="opacity-100 md:opacity-0 md:group-hover:opacity-100 transition-opacity flex items-center gap-1"
    >
      <UTooltip
        :text="workspace.isSecured ? 'Remove password' : 'Set password'"
      >
        <button
          class="text-gray-400 hover:text-amber-500 transition-colors"
          @click="emit('toggleSecured', workspace.identifier)"
        >
          <UIcon
            :name="
              workspace.isSecured
                ? 'heroicons:lock-closed'
                : 'heroicons:lock-open'
            "
            class="size-4"
          />
        </button>
      </UTooltip>
      <MetaControls
        item-name="workspace"
        :show-edit="true"
        :show-duplicate="false"
        :show-transfer="false"
        :extra-items="extraItems"
        @edit-record="emit('edit', workspace.identifier)"
        @delete-record="emit('delete', workspace.identifier)"
      />
    </div>
  </RyderCard>
</template>
