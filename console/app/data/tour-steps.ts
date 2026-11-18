export const tourConfig = {
  showProgress: true,
  allowClose: true,
  overlayOpacity: 0.65,
  smoothScroll: true,
  stagePadding: 8,
  stageRadius: 8,
  nextBtnText: "Continue",
  prevBtnText: "Back",
  doneBtnText: "Finish",
  steps: [
    {
      popover: {
        title: "Welcome to Daily",
        description:
          "Let us give you a quick tour of your workspace. This will only take a minute.",
        showButtons: ["next", "close"],
      },
    },
    {
      element: '[data-tour="sidebar"]',
      popover: {
        title: "Navigation",
        description:
          "Your sidebar gives you quick access to all workspace sections — notes, bookmarks, tasks, reminders, and more.",
        side: "right",
        align: "start",
      },
      skipMissingElement: true,
    },
    {
      element: '[data-tour="workspace-select"]',
      popover: {
        title: "Workspace Switcher",
        description:
          "Switch between workspaces or create new ones. Each workspace keeps your content separate and organized.",
        side: "bottom",
        align: "start",
      },
      skipMissingElement: true,
    },
    {
      element: '[data-tour="search"]',
      popover: {
        title: "Search",
        description:
          "Find anything instantly — search across notes, bookmarks, tasks, and more.",
        side: "bottom",
        align: "center",
      },
      skipMissingElement: true,
    },
    {
      element: '[data-tour="quick-actions"]',
      popover: {
        title: "Quick Actions",
        description:
          "Create notes, add tasks, set reminders, or save links in one click from the home screen.",
        side: "top",
        align: "center",
      },
    },
    {
      element: '[data-tour="notifications"]',
      popover: {
        title: "Notifications",
        description:
          "Stay on top of reminders and updates. Your notifications live here.",
        side: "bottom",
        align: "end",
      },
    },
    {
      element: '[data-tour="theme-toggle"]',
      popover: {
        title: "Theme",
        description: "Prefer dark mode? Toggle your theme anytime.",
        side: "bottom",
        align: "end",
      },
    },
  ],
};
