<script setup lang="ts">
definePageMeta({ layout: "auth" });

const { form, errors, loading, submitError, handleSubmit } = useLogin(
  undefined,
  { rememberEmail: true },
);
</script>

<template>
  <div class="flex flex-col flex-1">
    <RyderPageHeader
      title="Welcome back"
      subtitle="Sign in to your Lunar account to continue."
    />

    <form @submit.prevent="handleSubmit">
      <RyderInput
        v-model="form.email"
        type="email"
        name="email"
        label="Email"
        placeholder="you@example.com"
        size="lg"
        :disabled="loading"
      />
      <p v-if="errors.email" class="text-xs text-red-500 -mt-3">
        {{ errors.email }}
      </p>

      <div>
        <RyderPasswordInput
          v-model="form.password"
          name="password"
          label="Password"
          placeholder="••••••••"
          size="lg"
          :disabled="loading"
        />
        <div class="flex justify-end mt-1">
          <NuxtLink
            to="/auth/reset-password"
            class="text-xs text-primary-500 hover:text-primary-600 font-medium"
          >
            Forgot password?
          </NuxtLink>
        </div>
      </div>
      <p v-if="errors.password" class="text-xs text-red-500 -mt-3">
        {{ errors.password }}
      </p>

      <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

      <RyderButton
        type="submit"
        class="w-full text-center align-center"
        :loading="loading"
        :disabled="loading"
        size="md"
      >
        Sign in
      </RyderButton>
    </form>

    <p
      class="text-sm text-left text-gray-500 dark:text-gray-400 w-full left-0 pt-4"
    >
      Don't have an account?
      <NuxtLink
        to="/auth/signup"
        class="text-primary-500 hover:text-primary-600 font-medium"
      >
        Sign up
      </NuxtLink>
    </p>
  </div>
</template>
