// Below Tailwind's `lg` breakpoint — the same boundary the app shell uses to
// swap its desktop sidebar for the mobile bottom nav. Matching it exactly is
// what keeps a surface in sync with the chrome around it.
export function useIsMobile() {
  return useMediaQuery("(max-width: 1023px)");
}
