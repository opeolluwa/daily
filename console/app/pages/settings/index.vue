<script setup lang="ts">
definePageMeta({ layout: "settings" });
</script>

<template>
  <div>
    <SettingsNavList class="lg:hidden" />

    <div class="hidden lg:block">
      <SettingsProfileSettings />
    </div>
  </div>
</template>
