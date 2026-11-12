import { defineStore } from "pinia";
import { invoke } from "~/utils/invoke";

export type BackupProvider = "local" | "cloud";
export type CloudProvider = "imagekit" | "s3";

export interface ImagekitCloudConfig {
  publicKey: string;
  uploadUrl: string;
  folder?: string;
}

export interface S3CloudConfig {
  bucket: string;
  region: string;
  endpoint?: string;
  publicBaseUrl?: string;
  keyPrefix?: string;
}

export interface CloudStorageConfig {
  provider: CloudProvider;
  imagekit?: ImagekitCloudConfig;
  s3?: S3CloudConfig;
}

export interface ImagekitCloudRequest extends ImagekitCloudConfig {
  privateKey?: string;
}

export interface S3CloudRequest extends S3CloudConfig {
  accessKeyId?: string;
  secretAccessKey?: string;
}

export interface CloudStorageRequest {
  provider: CloudProvider;
  imagekit?: ImagekitCloudRequest;
  s3?: S3CloudRequest;
}

export interface CloudStorageStatus {
  active: boolean;
  enabled: boolean;
  provider: string | null;
  hasCredentials: boolean;
  config: CloudStorageConfig | null;
}

export const useBackupSettingsStore = defineStore("backup_settings", {
  state: () => ({
    initialized: false,
    available: true,
    error: null as string | null,

    provider: "local" as BackupProvider,
    cloudProvider: "s3" as CloudProvider,
    cloudActive: false,
    hasStoredSecrets: false,
    storedProvider: null as CloudProvider | null,

    testResult: null as string | null,
    testing: false,
    saving: false,

    // ImageKit fields
    imagekitPublicKey: "",
    imagekitPrivateKey: "",
    imagekitUploadUrl: "https://upload.imagekit.io/api/v1/files/upload",
    imagekitFolder: "",

    // S3 fields
    s3Bucket: "",
    s3Region: "",
    s3AccessKeyId: "",
    s3SecretAccessKey: "",
    s3Endpoint: "",
    s3PublicBaseUrl: "",
    s3KeyPrefix: "",
  }),

  getters: {
    savedConfigExists: (state) => state.hasStoredSecrets,
    isCloudConfigured: (state) => state.cloudActive,
    hasStoredSecretFor: (state) => (provider: CloudProvider) =>
      state.hasStoredSecrets && state.storedProvider === provider,
  },

  actions: {
    applyStatus(status: CloudStorageStatus) {
      this.cloudActive = status.active;
      this.provider = status.enabled ? "cloud" : "local";
      this.hasStoredSecrets = status.hasCredentials;
      this.storedProvider = (status.provider as CloudProvider | null) ?? null;

      if (status.config) {
        this.cloudProvider = status.config.provider;
        this.restoreCloudFields(status.config);
      }
    },

    async init() {
      if (this.initialized) return;

      try {
        const status =
          await invoke<CloudStorageStatus>("get_cloud_storage_status");
        this.applyStatus(status);
      } catch {
        this.available = false;
        this.error =
          "Cloud backup is only available in the desktop app. Credentials are stored in your operating system's keychain.";
      } finally {
        this.initialized = true;
      }
    },

    restoreCloudFields(cloud: CloudStorageConfig) {
      if (cloud.imagekit) {
        this.imagekitPublicKey = cloud.imagekit.publicKey;
        this.imagekitUploadUrl = cloud.imagekit.uploadUrl;
        this.imagekitFolder = cloud.imagekit.folder ?? "";
      }
      if (cloud.s3) {
        this.s3Bucket = cloud.s3.bucket;
        this.s3Region = cloud.s3.region;
        this.s3Endpoint = cloud.s3.endpoint ?? "";
        this.s3PublicBaseUrl = cloud.s3.publicBaseUrl ?? "";
        this.s3KeyPrefix = cloud.s3.keyPrefix ?? "";
      }
    },

    clearSecretInputs() {
      this.imagekitPrivateKey = "";
      this.s3AccessKeyId = "";
      this.s3SecretAccessKey = "";
    },

    buildCloudRequest(): CloudStorageRequest {
      if (this.cloudProvider === "imagekit") {
        return {
          provider: "imagekit",
          imagekit: {
            publicKey: this.imagekitPublicKey,
            uploadUrl: this.imagekitUploadUrl,
            folder: this.imagekitFolder || undefined,
            privateKey: this.imagekitPrivateKey || undefined,
          },
        };
      }

      return {
        provider: "s3",
        s3: {
          bucket: this.s3Bucket,
          region: this.s3Region,
          endpoint: this.s3Endpoint || undefined,
          publicBaseUrl: this.s3PublicBaseUrl || undefined,
          keyPrefix: this.s3KeyPrefix || undefined,
          accessKeyId: this.s3AccessKeyId || undefined,
          secretAccessKey: this.s3SecretAccessKey || undefined,
        },
      };
    },

    async save() {
      this.saving = true;
      this.error = null;

      try {
        const status =
          this.provider === "cloud"
            ? await invoke<CloudStorageStatus>("save_cloud_storage", {
                config: this.buildCloudRequest(),
              })
            : await invoke<CloudStorageStatus>("disable_cloud_storage");

        this.applyStatus(status);
        this.clearSecretInputs();
      } catch (error) {
        this.error = `Could not save backup settings: ${error}`;
      } finally {
        this.saving = false;
      }
    },

    async testConnection() {
      this.testing = true;
      this.testResult = null;
      this.error = null;

      try {
        const url = await invoke<string>("test_cloud_storage", {
          config: this.buildCloudRequest(),
        });
        this.testResult = `Connected successfully. Test file: ${url}`;
      } catch (error) {
        this.testResult = `Connection failed: ${error}`;
      } finally {
        this.testing = false;
      }
    },

    async removeCredentials() {
      this.saving = true;
      this.error = null;

      try {
        const status =
          await invoke<CloudStorageStatus>("clear_cloud_storage");
        this.applyStatus(status);
        this.clearSecretInputs();
        this.testResult = null;
      } catch (error) {
        this.error = `Could not remove credentials: ${error}`;
      } finally {
        this.saving = false;
      }
    },
  },
});
