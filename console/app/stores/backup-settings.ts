import { defineStore } from "pinia";
import { invoke } from "~/utils/invoke";

export type BackupProvider = "local" | "cloud";
export type CloudProvider = "imagekit" | "s3";

export interface ImagekitCloudConfig {
  publicKey: string;
  privateKey: string;
  uploadUrl: string;
  folder?: string;
}

export interface S3CloudConfig {
  bucket: string;
  region: string;
  accessKeyId: string;
  secretAccessKey: string;
  endpoint?: string;
  publicBaseUrl?: string;
  keyPrefix?: string;
}

export interface CloudStorageConfig {
  provider: CloudProvider;
  imagekit?: ImagekitCloudConfig;
  s3?: S3CloudConfig;
}

interface BackupServerConfig {
  provider: BackupProvider;
  cloud?: CloudStorageConfig;
}

interface CloudStorageStatus {
  active: boolean;
  provider: string | null;
}

const STRONGHOLD_KEY = "sync-server";

export const useBackupSettingsStore = defineStore("backup_settings", {
  state: () => ({
    initialized: false,
    provider: "local" as BackupProvider,
    cloudProvider: "s3" as CloudProvider,
    savedConfig: null as BackupServerConfig | null,
    cloudActive: false,
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
    savedConfigExists: (state) => state.savedConfig !== null,
    isCloudConfigured: (state) => state.cloudActive,
  },

  actions: {
    async init() {
      if (this.initialized) return;

      const stronghold = useStronghold();

      try {
        await stronghold.init(STRONGHOLD_KEY);
        this.initialized = true;
      } catch (error) {
        console.error("Failed to initialize stronghold:", error);
        return;
      }

      try {
        const config =
          await stronghold.getItem<BackupServerConfig>(STRONGHOLD_KEY);
        if (config) {
          this.savedConfig = config;
          this.provider = config.provider;

          if (config.cloud) {
            this.cloudProvider = config.cloud.provider;
            this.restoreCloudFields(config.cloud);

            if (config.provider === "cloud") {
              await this.activateCloudStorage();
            }
          }
        }
      } catch (error) {
        console.error("Failed to load backup config:", error);
      }

      // Check current cloud storage status from backend
      try {
        const status = await invoke<CloudStorageStatus>(
          "get_cloud_storage_status",
        );
        this.cloudActive = status.active;
      } catch {
        // Backend not available (browser mode)
      }
    },

    restoreCloudFields(cloud: CloudStorageConfig) {
      if (cloud.imagekit) {
        this.imagekitPublicKey = cloud.imagekit.publicKey;
        this.imagekitPrivateKey = cloud.imagekit.privateKey;
        this.imagekitUploadUrl = cloud.imagekit.uploadUrl;
        this.imagekitFolder = cloud.imagekit.folder ?? "";
      }
      if (cloud.s3) {
        this.s3Bucket = cloud.s3.bucket;
        this.s3Region = cloud.s3.region;
        this.s3AccessKeyId = cloud.s3.accessKeyId;
        this.s3SecretAccessKey = cloud.s3.secretAccessKey;
        this.s3Endpoint = cloud.s3.endpoint ?? "";
        this.s3PublicBaseUrl = cloud.s3.publicBaseUrl ?? "";
        this.s3KeyPrefix = cloud.s3.keyPrefix ?? "";
      }
    },

    buildCloudConfig(): CloudStorageConfig {
      const config: CloudStorageConfig = {
        provider: this.cloudProvider,
      };

      if (this.cloudProvider === "imagekit") {
        config.imagekit = {
          publicKey: this.imagekitPublicKey,
          privateKey: this.imagekitPrivateKey,
          uploadUrl: this.imagekitUploadUrl,
          folder: this.imagekitFolder || undefined,
        };
      } else {
        config.s3 = {
          bucket: this.s3Bucket,
          region: this.s3Region,
          accessKeyId: this.s3AccessKeyId,
          secretAccessKey: this.s3SecretAccessKey,
          endpoint: this.s3Endpoint || undefined,
          publicBaseUrl: this.s3PublicBaseUrl || undefined,
          keyPrefix: this.s3KeyPrefix || undefined,
        };
      }

      return config;
    },

    async activateCloudStorage() {
      const cloud = this.buildCloudConfig();
      try {
        const status = await invoke<CloudStorageStatus>(
          "configure_cloud_storage",
          { config: cloud },
        );
        this.cloudActive = status.active;
      } catch (error) {
        console.error("Failed to activate cloud storage:", error);
        this.cloudActive = false;
      }
    },

    async save() {
      this.saving = true;
      const stronghold = useStronghold();

      try {
        const payload: BackupServerConfig = {
          provider: this.provider,
        };

        if (this.provider === "cloud") {
          payload.cloud = this.buildCloudConfig();
          await this.activateCloudStorage();
        } else {
          // Disable cloud storage in backend
          try {
            await invoke("disable_cloud_storage");
            this.cloudActive = false;
          } catch {
            // Browser mode
          }
        }

        await stronghold.setItem(STRONGHOLD_KEY, payload);
        this.savedConfig = payload;
      } finally {
        this.saving = false;
      }
    },

    async testConnection() {
      this.testing = true;
      this.testResult = null;

      try {
        // Temporarily activate so we can test
        await this.activateCloudStorage();
        const url = await invoke<string>("test_cloud_connection");
        this.testResult = `Connected successfully. Test file: ${url}`;
      } catch (error) {
        this.testResult = `Connection failed: ${error}`;
      } finally {
        this.testing = false;
      }
    },
  },
});
