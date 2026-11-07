<script lang="ts" setup>
definePageMeta({ name: "Cloud Backup " });
const state = reactive({
  email: "",
  serverUrl: "",
  publicKey: "",
  secretKey: "",
});
</script>
<template>
  <UForm class="space-y-5">
    <RyderInput v-model="state.email" label="Email" type="email" />
    <RyderInput v-model="state.serverUrl" label="URL" type="text" />
    <RyderInput v-model="state.publicKey" label="Public key" type="text" />
    <RyderInput v-model="state.secretKey" label="Authorization" type="text" />
    <RyderButton block> Continue</RyderButton>
  </UForm>
</template>
