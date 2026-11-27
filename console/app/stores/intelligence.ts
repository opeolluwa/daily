import {
  invoke as tauriInvoke,
  Channel,
  isTauri,
} from "@tauri-apps/api/core";
import { defineStore } from "pinia";

export interface ProviderConfig {
  kind: "ollama" | "openai" | "anthropic";
  model: string;
  endpoint?: string;
  apiKey?: string;
}

interface ChatMessage {
  id: string;
  role: "system" | "user" | "assistant";
  content: string;
  createdAt: string;
}

interface StreamEvent {
  kind: "delta" | "done" | "error";
  content?: string;
  model?: string;
  message?: string;
}

export interface AssistantMessage {
  id: string;
  role: "user" | "assistant";
  parts: Array<{ type: "text"; id: string; text: string }>;
}

type ChatStatus = "ready" | "submitted" | "streaming" | "error";

function browserNoOp(command: string) {
  console.warn(`[intelligence] "${command}" is not available in the browser`);
}

export const useIntelligenceStore = defineStore("intelligence_store", {
  state: () => ({
    configured: false,
    healthy: false,
    config: null as ProviderConfig | null,
    models: [] as string[],
    messages: [] as AssistantMessage[],
    history: [] as ChatMessage[],
    status: "ready" as ChatStatus,
    error: null as string | null,
  }),

  actions: {
    async configure(config: ProviderConfig) {
      this.error = null;
      if (!isTauri()) {
        browserNoOp("intelligence_configure");
        return;
      }
      try {
        await tauriInvoke("intelligence_configure", { config });
        this.configured = true;
        this.config = config;
        await this.checkHealth();
      } catch (error) {
        this.error = String(error);
        this.configured = false;
      }
    },

    async checkHealth() {
      if (!isTauri()) {
        browserNoOp("intelligence_health_check");
        this.healthy = false;
        return;
      }
      try {
        this.healthy = await tauriInvoke<boolean>("intelligence_health_check");
      } catch {
        this.healthy = false;
      }
    },

    async fetchModels() {
      if (!isTauri()) {
        browserNoOp("intelligence_list_models");
        this.models = [];
        return;
      }
      try {
        this.models = await tauriInvoke<string[]>("intelligence_list_models");
      } catch (error) {
        this.error = String(error);
        this.models = [];
      }
    },

    async fetchConfig() {
      if (!isTauri()) {
        browserNoOp("intelligence_get_config");
        this.configured = false;
        return;
      }
      try {
        const fetched = await tauriInvoke<ProviderConfig | null>(
          "intelligence_get_config",
        );
        this.config = fetched
          ? { ...fetched, apiKey: fetched.apiKey ?? this.config?.apiKey }
          : null;
        this.configured = this.config !== null;
      } catch {
        this.configured = false;
      }
    },

    async sendMessage(content: string, includeContext = false) {
      if (!isTauri()) {
        browserNoOp("intelligence_agent_chat_stream");
        this.error = "The assistant is only available in the desktop app";
        this.status = "error";
        return;
      }

      const userMsg: AssistantMessage = {
        id: crypto.randomUUID(),
        role: "user",
        parts: [{ type: "text", id: crypto.randomUUID(), text: content }],
      };
      this.messages.push(userMsg);

      const assistantId = crypto.randomUUID();
      const partId = crypto.randomUUID();
      const assistantMsg: AssistantMessage = {
        id: assistantId,
        role: "assistant",
        parts: [{ type: "text", id: partId, text: "" }],
      };
      this.messages.push(assistantMsg);

      this.status = "streaming";
      this.error = null;

      const onEvent = new Channel<StreamEvent>();
      onEvent.onmessage = (event: StreamEvent) => {
        const msg = this.messages.find((m) => m.id === assistantId);
        if (!msg) return;

        switch (event.kind) {
          case "delta":
            msg.parts[0].text += event.content ?? "";
            break;
          case "done":
            this.history.push(
              { id: userMsg.id, role: "user", content, createdAt: new Date().toISOString() },
              { id: assistantId, role: "assistant", content: msg.parts[0].text, createdAt: new Date().toISOString() },
            );
            this.status = "ready";
            break;
          case "error":
            this.error = event.message ?? "Unknown error";
            this.status = "error";
            break;
        }
      };

      try {
        await tauriInvoke("intelligence_agent_chat_stream", {
          userMessage: content,
          history: this.history,
          includeContext,
          onEvent,
        });
      } catch (error) {
        this.error = String(error);
        this.status = "error";
      } finally {
        if (this.status === "streaming" || this.status === "submitted") {
          this.status = "ready";
        }
      }
    },

    clearConversation() {
      this.messages = [];
      this.history = [];
      this.error = null;
      this.status = "ready";
    },
  },
});
