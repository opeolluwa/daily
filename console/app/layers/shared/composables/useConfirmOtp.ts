import { useAuthStore } from "@shared/stores/auth";
import { useCountdown } from "@vueuse/core";

export function useConfirmOtp() {
  const route = useRoute();
  const authApi = useAuthApi();
  const authStore = useAuthStore();
  const toast = useToast();

  const flow = computed(() =>
    route.query.flow === "reset" ? "reset" : "verify",
  );

  const otp = ref("");
  const errors = reactive({ otp: "" });
  const loading = ref(false);
  const submitError = ref("");

  const { remaining, start: startCooldown } = useCountdown(120);

  function validate(): boolean {
    errors.otp = /^\d{6}$/.test(otp.value.trim()) ? "" : "";
    return !errors.otp;
  }

  async function handleResend() {
    if (remaining.value > 0 || !authStore.hasPendingToken) return;

    submitError.value = "";
    try {
      const response = await authApi.resendOtp(
        { flow: flow.value },
        authStore.pendingToken,
      );
      authStore.setPendingToken(response.token);
      startCooldown();
      toast.add({ color: "success", icon: "heroicons:check-circle", title: "Code Sent", description: "Check your email for a new verification code" });
    } catch (error) {
      submitError.value = (error as Error).message;
    }
  }

  async function handleSubmit() {
    if (!validate()) return;
    if (!authStore.hasPendingToken) {
      submitError.value =
        "Your verification session has expired. Please start over.";
      return;
    }

    loading.value = true;
    submitError.value = "";
    try {
      const response =
        flow.value === "reset"
          ? await authApi.verifyResetOtp(
              { otp: otp.value.trim() },
              authStore.pendingToken,
            )
          : await authApi.verifyAccount(
              { otp: otp.value.trim() },
              authStore.pendingToken,
            );

      if (flow.value === "reset") {
        authStore.setPendingToken(response.token);
        toast.add({ color: "success", icon: "heroicons:check-circle", title: "Code Verified", description: "Your verification code has been confirmed" });
        await navigateTo("/auth/reset-password?step=set");
      } else {
        authStore.clearPendingToken();
        authStore.setSession(response.token, "", 0);
        toast.add({ color: "success", icon: "heroicons:check-circle", title: "Account Verified", description: "Your account is now active" });
        await navigateTo("/");
      }
    } catch (error) {
      submitError.value = (error as Error).message;
    } finally {
      loading.value = false;
    }
  }

  return {
    flow,
    otp,
    errors,
    loading,
    submitError,
    remaining,
    handleSubmit,
    handleResend,
  };
}