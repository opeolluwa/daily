<script setup lang="ts">
definePageMeta({ layout: "auth" });

const {
  step,
  form,
  errors,
  loading,
  submitError,
  handleRequestCode,
  handleSetPassword,
} = useResetPassword();
</script>

<template>
  <div class="flex flex-col gap-5">
    <RyderPageHeader
      v-if="step === 'email'"
      title="Reset your password"
      subtitle="Enter your account email and we'll send you a reset code."
    />
    <RyderPageHeader
      v-else
      title="Set a new password"
      subtitle="Choose a new password for your account."
    />

    <form
      class="flex flex-col gap-4"
      @submit.prevent="
        step === 'email' ? handleRequestCode() : handleSetPassword()
      "
    >
      <template v-if="step === 'email'">
        <RyderInput
          v-model="form.email"
          type="email"
          name="email"
          label="Email"
          hint="required"
          placeholder="you@example.com"
          :disabled="loading"
        />
        <p v-if="errors.email" class="text-xs text-red-500 -mt-3">
          {{ errors.email }}
        </p>

        <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

        <RyderButton type="submit" size="md" class="w-full" :loading="loading" :disabled="loading">
          Send reset code
        </RyderButton>
      </template>

      <template v-else>
        <RyderPasswordInput
          v-model="form.password"
          name="password"
          label="New password"
          hint="required"
          placeholder="At least 6 characters"
          :disabled="loading"
        />
        <p v-if="errors.password" class="text-xs text-red-500 -mt-3">
          {{ errors.password }}
        </p>

        <RyderPasswordInput
          v-model="form.confirmPassword"
          name="confirmPassword"
          label="Confirm new password"
          hint="required"
          placeholder="Repeat your new password"
          :disabled="loading"
        />
        <p v-if="errors.confirmPassword" class="text-xs text-red-500 -mt-3">
          {{ errors.confirmPassword }}
        </p>

        <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

        <RyderButton
          type="submit"
          size="md"
          class="w-full"
          :loading="loading"
          :disabled="loading"
        >
          Update password
        </RyderButton>
      </template>
    </form>

    <p class="text-sm text-center text-gray-500 dark:text-gray-400">
      Remembered your password?
      <NuxtLink
        to="/auth/login"
        class="text-primary-500 hover:text-primary-600 font-medium"
      >
        Sign in
      </NuxtLink>
    </p>
  </div>
</template>
