<script setup lang="ts">
import { marked } from "marked";
import DOMPurify from "dompurify";
import { Domternal } from "@domternal/vue";
import type { Editor } from "@domternal/core";

import { extensions } from "./extension";
import { getDmVars } from "./stylerules";
import { isHtml } from "./utils";

const colorMode = useColorMode();
const isDark = computed(() => colorMode.value === "dark");
const dmVars = computed(() => getDmVars(isDark.value));

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function handleUpdate({ editor }: { editor: any }) {
  model.value = DOMPurify.sanitize(editor.getHTML());
}

const model = defineModel<string>();

const editor = shallowRef<Editor | null>(null);

function handleCreate(ed: Editor) {
  editor.value = ed;
}

function handleDestroy() {
  editor.value = null;
}

defineExpose({
  editor,
});

const initialContent = computed(() => {
  const raw = model.value ?? "";
  if (!raw || isHtml(raw)) return raw;
  return marked.parse(raw) as string;
});
</script>

<template>
  <div
    :class="{ 'dm-theme-dark': isDark }"
    :style="dmVars"
    class="notes-editor"
  >
    <Domternal
      :extensions="extensions"
      :content="initialContent"
      :on-update="handleUpdate"
      :on-create="handleCreate"
      :on-destroy="handleDestroy"
    >
      <Domternal.Toolbar />
      <Domternal.Content class="mt-4" />
    </Domternal>
  </div>
</template>

<style>
.dm-editor {
  --dm-editor-bg: transparent;
  --dm-bg: transparent;
  --dm-editor-border: none;
  --dm-border-color: transparent;
  --dm-primary: var(--color-primary-500);
  --dm-active: var(--color-primary-500);
}

.notes-editor .dm-editor .ProseMirror {
  --dm-editor-padding: 0;
  --dm-editor-padding-top-extra: 0.25rem;
}
</style>
