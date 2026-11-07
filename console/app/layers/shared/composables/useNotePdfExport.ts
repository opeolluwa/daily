import { useNoteStore } from "@shared/stores/notes";

export function useNotePdfExport() {
  const noteStore = useNoteStore();

  const showOverwriteConfirm = ref(false);
  const duplicateFileName = ref("");
  const exporting = ref(false);

  let pending: {
    recordIdentifier: string;
    previousWorkspaceIdentifier: string;
  } | null = null;

  const overwriteMessage = computed(() =>
    duplicateFileName.value
      ? `A PDF named "${duplicateFileName.value}" already exists. Do you want to overwrite it?`
      : "A PDF with this name already exists. Do you want to overwrite it?",
  );

  watch(showOverwriteConfirm, (isOpen) => {
    if (isOpen) return;

    pending = null;
    duplicateFileName.value = "";
  });

  async function exportAsPdf(
    recordIdentifier: string,
    previousWorkspaceIdentifier: string,
  ) {
    try {
      const result = await noteStore.exportAsPdf(
        recordIdentifier,
        previousWorkspaceIdentifier,
        false,
      );

      if (!result || result.status !== "duplicate") return;

      pending = { recordIdentifier, previousWorkspaceIdentifier };
      duplicateFileName.value = result.path.split(/[\\/]/).pop() ?? "file";
      showOverwriteConfirm.value = true;
    } catch (err) {
      console.log((err as unknown as Error).message);
    }
  }

  async function confirmOverwrite() {
    const request = pending;

    if (!request) {
      showOverwriteConfirm.value = false;
      return;
    }

    exporting.value = true;

    try {
      await noteStore.exportAsPdf(
        request.recordIdentifier,
        request.previousWorkspaceIdentifier,
        true,
      );
    } catch (err) {
      console.log((err as unknown as Error).message);
    } finally {
      exporting.value = false;
      showOverwriteConfirm.value = false;
      pending = null;
      duplicateFileName.value = "";
    }
  }

  function cancelOverwrite() {
    showOverwriteConfirm.value = false;
    pending = null;
    duplicateFileName.value = "";
  }

  return {
    showOverwriteConfirm,
    duplicateFileName,
    overwriteMessage,
    exporting,
    exportAsPdf,
    confirmOverwrite,
    cancelOverwrite,
  };
}
