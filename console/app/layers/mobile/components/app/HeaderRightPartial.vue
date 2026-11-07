<template>
  <div class="flex gap-x-1.5">
    <RyderButton
      size="md"
      color="neutral"
      variant="ghost"
      aria-label="Search"
      @click="navigateTo('/search')"
    >
      <UIcon name="heroicons:magnifying-glass" class="size-5" />
    </RyderButton>

    <RyderButton
      size="md"
      color="neutral"
      variant="ghost"
      aria-label="Notifications"
      @click="navigateTo('/notifications')"
    >
      <UIcon name="heroicons:bell" class="size-5" />
    </RyderButton>

    <UUser
      size="sm"
      class="cursor-pointer"
      :avatar="{ src: 'https://i.pravatar.cc/150?u=john-doe' }"
    />
  </div>
</template>
