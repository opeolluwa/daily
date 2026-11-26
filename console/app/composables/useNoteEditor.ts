import { onBeforeRouteLeave } from "vue-router";
import { stripHtml } from "~/utils/text";

interface UseNoteEditorOptions {
  saveFn: () => Promise<void>;
  canSave: MaybeRefOrGetter<boolean>;
}

export function useNoteEditor({ saveFn, canSave }: UseNoteEditorOptions) {
  const router = useRouter();

  const content = ref("");
  const submitting = ref(false);
  const saved = ref(false);
  const error = ref<string | null>(null);
  const notesEditor = ref<InstanceType<typeof NotesEditor> | null>(null);

  const title = computed(() => {
    const text = stripHtml(content.value).trim();
    const firstLine = text.split("\n")[0]?.trim() ?? "";
    return firstLine.slice(0, 80) || "Untitled";
  });

  async function handleSave() {
    if (!toValue(canSave)) return;
    submitting.value = true;
    error.value = null;
    try {
      await saveFn();
      saved.value = true;
      router.push("/notes");
    } catch (e) {
      error.value = String(e);
      submitting.value = false;
    }
  }

  useEventListener("keydown", (e: KeyboardEvent) => {
    if ((e.metaKey || e.ctrlKey) && e.key === "s") {
      e.preventDefault();
      if (toValue(canSave) && !submitting.value) handleSave();
    }
  });

  onBeforeRouteLeave(async () => {
    if (submitting.value || saved.value) return;
    if (!toValue(canSave)) return;
    try {
      await saveFn();
    } catch (e) {
      console.error(e);
    }
  });

  return {
    content,
    submitting,
    saved,
    error,
    notesEditor,
    title,
    handleSave,
  };
}
