<template>
  <div
    class="fixed inset-0 z-[9999] flex flex-col items-center justify-center bg-white dark:bg-gray-900"
  >
    <div class="flex flex-col items-center gap-6">
      <!-- Logo -->
      <AppLogo class="size-20 rounded-2xl shadow-sm" />

      <!-- App name -->
      <div class="flex flex-col items-center gap-1">
        <h1
          class="text-2xl font-semibold tracking-tight text-gray-900 dark:text-white"
        >
          Daily
        </h1>
        <p class="text-sm text-gray-400 dark:text-gray-500">
          Loading your workspace…
        </p>
      </div>

      <!-- Dots loader -->
      <div class="flex items-center gap-1.5">
        <span
          class="size-1.5 rounded-full bg-primary-400 animate-bounce [animation-delay:-0.3s]"
        />
        <span
          class="size-1.5 rounded-full bg-primary-400 animate-bounce [animation-delay:-0.15s]"
        />
        <span class="size-1.5 rounded-full bg-primary-400 animate-bounce" />
      </div>
    </div>
  </div>
</template>
