import { useAuthStore } from "~/stores/auth";

export function useResetPassword() {
  const route = useRoute();
  const authApi = useAuthApi();
  const authStore = useAuthStore();
  const toast = useToast();

  const step = computed(() => (route.query.step === "set" ? "set" : "email"));

  const form = reactive({ email: "", password: "", confirmPassword: "" });
  const errors = reactive({ email: "", password: "", confirmPassword: "" });
  const loading = ref(false);
  const submitError = ref("");

  function validateEmail(): boolean {
    errors.email = emailValidator(form.email)
      ? ""
      : "A valid email is required";
    return !errors.email;
  }

  function validatePassword(): boolean {
    errors.password =
      form.password.length >= 6 ? "" : "Password must be at least 6 characters";
    errors.confirmPassword =
      form.confirmPassword === form.password ? "" : "Passwords do not match";
    return !errors.password && !errors.confirmPassword;
  }

  async function handleRequestCode() {
    if (!validateEmail()) return;
    loading.value = true;
    submitError.value = "";
    try {
      const response = await authApi.forgottenPassword({
        email: form.email.trim(),
      });
      authStore.setPendingToken(response.token);
      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Reset Code Sent",
        description: "Check your email for the reset code",
        duration: 5000,
      });
      await navigateTo("/auth/confirm-otp?flow=reset");
    } catch (error) {
      submitError.value = (error as Error).message;
    } finally {
      loading.value = false;
    }
  }

  async function handleSetPassword() {
    if (!validatePassword()) return;
    if (!authStore.hasPendingToken) {
      submitError.value = "Your reset session has expired. Please start over.";
      return;
    }

    loading.value = true;
    submitError.value = "";
    try {
      await authApi.setNewPassword(
        {
          password: form.password,
          confirmPassword: form.confirmPassword,
        },
        authStore.pendingToken,
      );
      authStore.clearPendingToken();
      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Password Updated",
        description: "Your password has been changed",
      });
      await navigateTo("/auth/login");
    } catch (error) {
      submitError.value = (error as Error).message;
    } finally {
      loading.value = false;
    }
  }

  return {
    step,
    form,
    errors,
    loading,
    submitError,
    handleRequestCode,
    handleSetPassword,
  };
}
