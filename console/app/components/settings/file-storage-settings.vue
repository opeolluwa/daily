<script setup lang="ts">
import {
  useBackupSettingsStore,
  type BackupProvider,
  type CloudProvider,
} from "~/stores/backup-settings";

const providerOptions: {
  key: BackupProvider;
  label: string;
  desc: string;
  icon: string;
}[] = [
  {
    key: "local",
    label: "Local only",
    desc: "Data stays on this device, no cloud backup.",
    icon: "heroicons:computer-desktop",
  },
  {
    key: "cloud",
    label: "Local + Cloud backup",
    desc: "Save locally, back up images to your cloud storage.",
    icon: "heroicons:cloud-arrow-up",
  },
];

const cloudProviderOptions: {
  key: CloudProvider;
  label: string;
  desc: string;
}[] = [
  {
    key: "s3",
    label: "Amazon S3",
    desc: "AWS S3 or any S3-compatible service (MinIO, R2, etc.)",
  },
  {
    key: "imagekit",
    label: "ImageKit",
    desc: "ImageKit CDN with automatic optimisation.",
  },
];

const backupStore = useBackupSettingsStore();

onMounted(() => backupStore.init());
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg p-5">
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-1">
        File Storage
      </h2>
      <p class="text-xs text-gray-400 mb-4">
        Choose where your whiteboard images are stored.
      </p>

      <!-- Provider options -->
      <div class="flex flex-col gap-2 mb-5">
        <button
          v-for="opt in providerOptions"
          :key="opt.key"
          class="flex items-start gap-3 p-3 rounded-lg border transition-colors text-left"
          :class="
            backupStore.provider === opt.key
              ? 'border-primary-400 bg-primary-50 dark:bg-primary-950 dark:border-primary-600'
              : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700'
          "
          @click="backupStore.provider = opt.key"
        >
          <div
            class="mt-0.5 size-8 rounded-md flex items-center justify-center shrink-0"
            :class="
              backupStore.provider === opt.key
                ? 'bg-primary-100 dark:bg-primary-900'
                : 'bg-gray-100 dark:bg-gray-700'
            "
          >
            <UIcon
              :name="opt.icon"
              class="size-4"
              :class="
                backupStore.provider === opt.key
                  ? 'text-primary-600 dark:text-primary-400'
                  : 'text-gray-500 dark:text-gray-400'
              "
            />
          </div>
          <div class="flex-1 min-w-0">
            <p
              class="text-sm font-medium"
              :class="
                backupStore.provider === opt.key
                  ? 'text-primary-700 dark:text-primary-300'
                  : 'text-gray-700 dark:text-gray-200'
              "
            >
              {{ opt.label }}
            </p>
            <p class="text-xs text-gray-400 mt-0.5">{{ opt.desc }}</p>
          </div>
          <UIcon
            v-if="backupStore.provider === opt.key"
            name="heroicons:check-circle"
            class="size-4 text-primary-500 shrink-0 mt-1"
          />
        </button>
      </div>

      <!-- Error -->
      <div
        v-if="backupStore.error"
        class="text-xs p-3 mb-4 rounded-lg bg-red-50 dark:bg-red-950 text-red-700 dark:text-red-300"
      >
        {{ backupStore.error }}
      </div>

      <!-- Cloud configuration -->
      <div v-if="backupStore.provider === 'cloud'" class="flex flex-col gap-4">
        <!-- Cloud provider selector -->
        <div>
          <label
            class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-2"
          >
            Cloud provider
          </label>
          <div class="flex gap-2">
            <button
              v-for="cp in cloudProviderOptions"
              :key="cp.key"
              class="flex-1 p-3 rounded-lg border text-left transition-colors"
              :class="
                backupStore.cloudProvider === cp.key
                  ? 'border-primary-400 bg-primary-50 dark:bg-primary-950 dark:border-primary-600'
                  : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700'
              "
              @click="backupStore.cloudProvider = cp.key"
            >
              <p
                class="text-sm font-medium"
                :class="
                  backupStore.cloudProvider === cp.key
                    ? 'text-primary-700 dark:text-primary-300'
                    : 'text-gray-700 dark:text-gray-200'
                "
              >
                {{ cp.label }}
              </p>
              <p class="text-xs text-gray-400 mt-0.5">{{ cp.desc }}</p>
            </button>
          </div>
        </div>

        <p class="text-xs text-gray-400">
          Secrets are stored in your operating system's keychain and are never
          shown again after saving.
        </p>

        <!-- S3 config fields -->
        <div
          v-if="backupStore.cloudProvider === 's3'"
          class="flex flex-col gap-3"
        >
          <div class="grid grid-cols-2 gap-3">
            <div>
              <label
                class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
                >Bucket</label
              >
              <input
                v-model="backupStore.s3Bucket"
                type="text"
                placeholder="my-bucket"
                class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
              />
            </div>
            <div>
              <label
                class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
                >Region</label
              >
              <input
                v-model="backupStore.s3Region"
                type="text"
                placeholder="us-east-1"
                class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
              />
            </div>
          </div>
          <div>
            <label
              class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
              >Access Key ID</label
            >
            <input
              v-model="backupStore.s3AccessKeyId"
              type="password"
              autocomplete="off"
              spellcheck="false"
              :placeholder="
                backupStore.hasStoredSecretFor('s3')
                  ? '•••••••• saved — leave blank to keep'
                  : 'AKIA...'
              "
              class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
            />
          </div>
          <div>
            <label
              class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
              >Secret Access Key</label
            >
            <input
              v-model="backupStore.s3SecretAccessKey"
              type="password"
              autocomplete="off"
              spellcheck="false"
              :placeholder="
                backupStore.hasStoredSecretFor('s3')
                  ? '•••••••• saved — leave blank to keep'
                  : 'secret'
              "
              class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
            />
          </div>
          <div>
            <label
              class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
            >
              Custom endpoint
              <span class="text-gray-400 font-normal">(optional)</span>
            </label>
            <input
              v-model="backupStore.s3Endpoint"
              type="text"
              placeholder="https://s3.example.com"
              class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
            />
          </div>
          <div>
            <label
              class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
            >
              Public base URL / CDN
              <span class="text-gray-400 font-normal">(optional)</span>
            </label>
            <input
              v-model="backupStore.s3PublicBaseUrl"
              type="text"
              placeholder="https://cdn.example.com"
              class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
            />
          </div>
          <div>
            <label
              class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
            >
              Key prefix
              <span class="text-gray-400 font-normal">(optional)</span>
            </label>
            <input
              v-model="backupStore.s3KeyPrefix"
              type="text"
              placeholder="uploads/whiteboards"
              class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
            />
          </div>
        </div>

        <!-- ImageKit config fields -->
        <div
          v-if="backupStore.cloudProvider === 'imagekit'"
          class="flex flex-col gap-3"
        >
          <div>
            <label
              class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
              >Public Key</label
            >
            <input
              v-model="backupStore.imagekitPublicKey"
              type="text"
              placeholder="public_..."
              class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
            />
          </div>
          <div>
            <label
              class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
              >Private Key</label
            >
            <input
              v-model="backupStore.imagekitPrivateKey"
              type="password"
              autocomplete="off"
              spellcheck="false"
              :placeholder="
                backupStore.hasStoredSecretFor('imagekit')
                  ? '•••••••• saved — leave blank to keep'
                  : 'private_...'
              "
              class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
            />
          </div>
          <div>
            <label
              class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
              >Upload URL</label
            >
            <input
              v-model="backupStore.imagekitUploadUrl"
              type="text"
              placeholder="https://upload.imagekit.io/api/v1/files/upload"
              class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
            />
          </div>
          <div>
            <label
              class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
            >
              Folder
              <span class="text-gray-400 font-normal">(optional)</span>
            </label>
            <input
              v-model="backupStore.imagekitFolder"
              type="text"
              placeholder="/whiteboards"
              class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 placeholder-gray-400"
            />
          </div>
        </div>

        <!-- Status indicator -->
        <div
          v-if="backupStore.cloudActive"
          class="flex items-center gap-2 text-xs text-green-600 dark:text-green-400"
        >
          <div class="size-2 rounded-full bg-green-500" />
          Cloud backup active
        </div>
        <div
          v-else-if="backupStore.hasStoredSecrets"
          class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400"
        >
          <div class="size-2 rounded-full bg-gray-400" />
          Credentials saved, cloud backup is off
        </div>

        <!-- Test result -->
        <div
          v-if="backupStore.testResult"
          class="text-xs p-3 rounded-lg"
          :class="
            backupStore.testResult.startsWith('Connected')
              ? 'bg-green-50 dark:bg-green-950 text-green-700 dark:text-green-300'
              : 'bg-red-50 dark:bg-red-950 text-red-700 dark:text-red-300'
          "
        >
          {{ backupStore.testResult }}
        </div>

        <!-- Action buttons -->
        <div class="flex flex-wrap gap-2">
          <button
            class="px-4 py-2 text-sm font-medium rounded-lg border border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors disabled:opacity-50"
            :disabled="backupStore.testing || !backupStore.available"
            @click="backupStore.testConnection()"
          >
            {{ backupStore.testing ? "Testing..." : "Test connection" }}
          </button>
          <button
            class="px-4 py-2 text-sm font-medium rounded-lg bg-primary-500 text-white hover:bg-primary-600 transition-colors disabled:opacity-50"
            :disabled="backupStore.saving || !backupStore.available"
            @click="backupStore.save()"
          >
            {{ backupStore.saving ? "Saving..." : "Save" }}
          </button>
          <button
            v-if="backupStore.hasStoredSecrets"
            class="px-4 py-2 text-sm font-medium rounded-lg border border-red-200 dark:border-red-900 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-950 transition-colors disabled:opacity-50"
            :disabled="backupStore.saving"
            @click="backupStore.removeCredentials()"
          >
            Remove credentials
          </button>
        </div>
      </div>

      <!-- Local mode: simple save -->
      <div v-if="backupStore.provider === 'local'" class="mt-2">
        <button
          class="px-4 py-2 text-sm font-medium rounded-lg bg-primary-500 text-white hover:bg-primary-600 transition-colors disabled:opacity-50"
          :disabled="backupStore.saving || !backupStore.available"
          @click="backupStore.save()"
        >
          {{ backupStore.saving ? "Saving..." : "Save" }}
        </button>
      </div>
    </div>
  </div>
</template>
