import type { ExcalidrawElement, Result } from "@drawvue/core";
import {
  getNonDeletedElements,
  hashElementsVersion,
  tryCatchSync,
} from "@drawvue/core";
import type { CurrentPersistedScene, PersistedScene } from "./types";
import { CURRENT_SCHEMA_VERSION, ELEMENT_COUNT_WARN_THRESHOLD } from "./types";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

// ---------------------------------------------------------------------------
// Migrations — compiler-enforced via MigrationMap
// ---------------------------------------------------------------------------

export function migrateScene(stored: PersistedScene): CurrentPersistedScene {
  if (stored.schemaVersion === CURRENT_SCHEMA_VERSION) {
    return stored;
  }

  // Dead code today — MigrationMap is empty. When V2 is added,
  // TypeScript will force adding a case here.
  throw new Error(`No migration for schema version ${stored.schemaVersion}`);
}

// ---------------------------------------------------------------------------
// Validation — manual type guards, normalize-on-read
// ---------------------------------------------------------------------------

export function isValidPersistedScene(data: unknown): data is PersistedScene {
  if (!isRecord(data)) return false;
  if (typeof data.schemaVersion !== "number") return false;
  if (!Array.isArray(data.elements)) return false;
  if (typeof data.savedAt !== "number") return false;
  return true;
}

export function normalizeElements(
  elements: readonly unknown[],
): ExcalidrawElement[] {
  return elements.filter((el): el is ExcalidrawElement => {
    if (!isRecord(el)) return false;

    if (typeof el.id !== "string" || el.id.length === 0) {
      console.warn("[persistence] Skipping element with missing/invalid id");
      return false;
    }

    if (typeof el.type !== "string" || el.type.length === 0) {
      console.warn(
        "[persistence] Skipping element with missing/invalid type:",
        el.id,
      );
      return false;
    }

    return true;
  });
}

// ---------------------------------------------------------------------------
// Serialization — JSON round-trip to strip non-serializable customData
// ---------------------------------------------------------------------------

export function serializeElements(
  elements: readonly ExcalidrawElement[],
): Result<ExcalidrawElement[]> {
  const nonDeleted = [...getNonDeletedElements(elements)];

  if (nonDeleted.length > ELEMENT_COUNT_WARN_THRESHOLD) {
    console.warn(
      `[persistence] Element count (${nonDeleted.length}) exceeds threshold (${ELEMENT_COUNT_WARN_THRESHOLD}). Consider per-element storage for better performance.`,
    );
  }

  return tryCatchSync(() => {
    const json = JSON.stringify(nonDeleted);
    const parsed: unknown = JSON.parse(json);
    if (!Array.isArray(parsed))
      throw new Error("Serialization produced non-array");
    return parsed as ExcalidrawElement[];
  });
}

// ---------------------------------------------------------------------------
// Scene envelope stored in the whiteboards.payload column
// ---------------------------------------------------------------------------

export function buildPersistedScene(
  elements: readonly ExcalidrawElement[],
): CurrentPersistedScene {
  return {
    schemaVersion: CURRENT_SCHEMA_VERSION,
    elements: [...elements],
    savedAt: Date.now(),
    elementCount: elements.length,
  };
}

export function hashSceneElements(
  elements: readonly ExcalidrawElement[],
): number {
  return hashElementsVersion(getNonDeletedElements(elements));
}
