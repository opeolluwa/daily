const KEYBOARD_THRESHOLD = 50;

export function useVirtualKeyboard() {
  const isMobile = useIsMobile();
  const keyboardHeight = ref(0);

  let cleanup: (() => void) | null = null;

  function update() {
    const vv = window.visualViewport;
    if (!isMobile.value || !vv) {
      keyboardHeight.value = 0;
      return;
    }
    const estimated = window.innerHeight - vv.height - vv.offsetTop;
    keyboardHeight.value =
      estimated > KEYBOARD_THRESHOLD ? Math.round(estimated) : 0;
  }

  onMounted(() => {
    const vv = window.visualViewport;
    if (!vv) return;

    vv.addEventListener("resize", update);
    vv.addEventListener("scroll", update);

    cleanup = () => {
      vv.removeEventListener("resize", update);
      vv.removeEventListener("scroll", update);
    };
  });

  onUnmounted(() => {
    cleanup?.();
  });

  const isKeyboardOpen = computed(() => keyboardHeight.value > 0);

  return { keyboardHeight, isKeyboardOpen };
}
