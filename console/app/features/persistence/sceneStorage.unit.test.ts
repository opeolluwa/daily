import type { ExcalidrawElement } from "@drawvue/core";
import { createTestElement } from "@drawvue/core/test-utils";
import {
  buildPersistedScene,
  hashSceneElements,
  isValidPersistedScene,
  migrateScene,
  normalizeElements,
  serializeElements,
} from "./sceneStorage";
import { CURRENT_SCHEMA_VERSION, ELEMENT_COUNT_WARN_THRESHOLD } from "./types";
import type { CurrentPersistedScene, PersistedScene } from "./types";

describe("sceneStorage", () => {
  // ---------------------------------------------------------------------------
  // isValidPersistedScene
  // ---------------------------------------------------------------------------

  describe("isValidPersistedScene", () => {
    it("returns true for valid scene with all required fields", () => {
      const scene: PersistedScene = {
        schemaVersion: 1,
        elements: [],
        savedAt: Date.now(),
        elementCount: 0,
      };
      expect(isValidPersistedScene(scene)).toBe(true);
    });

    it("returns false for null", () => {
      expect(isValidPersistedScene(null)).toBe(false);
    });

    it("returns false for non-objects and primitives", () => {
      expect(isValidPersistedScene(undefined)).toBe(false);
      expect(isValidPersistedScene(42)).toBe(false);
      expect(isValidPersistedScene("string")).toBe(false);
      expect(isValidPersistedScene(true)).toBe(false);
    });

    it("returns false for missing schemaVersion", () => {
      expect(isValidPersistedScene({ elements: [], savedAt: 1 })).toBe(false);
    });

    it("returns false for wrong-type schemaVersion", () => {
      expect(
        isValidPersistedScene({ schemaVersion: "1", elements: [], savedAt: 1 }),
      ).toBe(false);
    });

    it("returns false for missing elements", () => {
      expect(isValidPersistedScene({ schemaVersion: 1, savedAt: 1 })).toBe(
        false,
      );
    });

    it("returns false for wrong-type elements", () => {
      expect(
        isValidPersistedScene({
          schemaVersion: 1,
          elements: "not-array",
          savedAt: 1,
        }),
      ).toBe(false);
    });

    it("returns false for missing savedAt", () => {
      expect(isValidPersistedScene({ schemaVersion: 1, elements: [] })).toBe(
        false,
      );
    });

    it("returns false for wrong-type savedAt", () => {
      expect(
        isValidPersistedScene({
          schemaVersion: 1,
          elements: [],
          savedAt: "now",
        }),
      ).toBe(false);
    });
  });

  // ---------------------------------------------------------------------------
  // normalizeElements
  // ---------------------------------------------------------------------------

  describe("normalizeElements", () => {
    it("passes through valid elements", () => {
      const el = createTestElement({ id: "a" });
      const result = normalizeElements([el]);
      expect(result).toHaveLength(1);
      expect(result[0]!.id).toBe("a");
    });

    it("filters out null and non-object entries", () => {
      const el = createTestElement({ id: "valid" });
      const result = normalizeElements([null, undefined, 42, "str", el]);
      expect(result).toHaveLength(1);
      expect(result[0]!.id).toBe("valid");
    });

    it("filters out elements with missing or empty id", () => {
      const noId = { type: "rectangle" };
      const emptyId = { id: "", type: "rectangle" };
      const valid = createTestElement({ id: "ok" });
      const result = normalizeElements([noId, emptyId, valid]);
      expect(result).toHaveLength(1);
      expect(result[0]!.id).toBe("ok");
    });

    it("filters out elements with missing or empty type", () => {
      const noType = { id: "a" };
      const emptyType = { id: "b", type: "" };
      const valid = createTestElement({ id: "c" });
      const result = normalizeElements([noType, emptyType, valid]);
      expect(result).toHaveLength(1);
      expect(result[0]!.id).toBe("c");
    });

    it("preserves valid elements in mixed array", () => {
      const a = createTestElement({ id: "a" });
      const b = createTestElement({ id: "b" });
      const result = normalizeElements([null, a, { id: "" }, b, 123]);
      expect(result).toHaveLength(2);
      expect(result.map((e) => e.id)).toEqual(["a", "b"]);
    });
  });

  // ---------------------------------------------------------------------------
  // serializeElements
  // ---------------------------------------------------------------------------

  describe("serializeElements", () => {
    it("serializes non-deleted elements via JSON round-trip", () => {
      const el = createTestElement({ id: "a", isDeleted: false });
      const [error, result] = serializeElements([el]);
      expect(error).toBeNull();
      expect(result).toHaveLength(1);
      expect(result![0]!.id).toBe("a");
    });

    it("filters deleted elements", () => {
      const alive = createTestElement({ id: "alive", isDeleted: false });
      const deleted = createTestElement({ id: "dead", isDeleted: true });
      const [error, result] = serializeElements([alive, deleted]);
      expect(error).toBeNull();
      expect(result).toHaveLength(1);
      expect(result![0]!.id).toBe("alive");
    });

    it("returns error tuple on non-serializable customData", () => {
      const circular = {} as Record<string, unknown>;
      circular.self = circular;
      const el = createTestElement({ id: "bad", customData: circular });
      const [error, result] = serializeElements([el]);
      expect(error).toBeInstanceOf(Error);
      expect(result).toBeNull();
    });

    it("warns when element count exceeds threshold", () => {
      const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});

      const elements: ExcalidrawElement[] = Array.from(
        { length: ELEMENT_COUNT_WARN_THRESHOLD + 1 },
        (_, i) => createTestElement({ id: `el-${i}` }),
      );

      serializeElements(elements);
      expect(warnSpy).toHaveBeenCalledWith(
        expect.stringContaining("Element count"),
      );

      warnSpy.mockRestore();
    });
  });

  // ---------------------------------------------------------------------------
  // migrateScene
  // ---------------------------------------------------------------------------

  describe("migrateScene", () => {
    it("returns scene unchanged when at current version", () => {
      const scene: CurrentPersistedScene = {
        schemaVersion: CURRENT_SCHEMA_VERSION,
        elements: [],
        savedAt: 1000,
        elementCount: 0,
      };
      const result = migrateScene(scene);
      expect(result).toBe(scene);
    });
  });

  // ---------------------------------------------------------------------------
  // buildPersistedScene
  // ---------------------------------------------------------------------------

  describe("buildPersistedScene", () => {
    it("builds an envelope with the current schema version", () => {
      const el = createTestElement({ id: "a" });
      const scene = buildPersistedScene([el]);

      expect(scene.schemaVersion).toBe(CURRENT_SCHEMA_VERSION);
      expect(scene.elements).toHaveLength(1);
      expect(scene.elements[0]!.id).toBe("a");
      expect(scene.elementCount).toBe(1);
      expect(typeof scene.savedAt).toBe("number");
    });

    it("copies the elements array instead of reusing it", () => {
      const elements = [createTestElement({ id: "a" })];
      const scene = buildPersistedScene(elements);
      expect(scene.elements).not.toBe(elements);
      expect(scene.elements).toEqual(elements);
    });
  });

  // ---------------------------------------------------------------------------
  // hashSceneElements
  // ---------------------------------------------------------------------------

  describe("hashSceneElements", () => {
    it("returns a stable hash for the same elements", () => {
      const elements = [createTestElement({ id: "a" })];
      expect(hashSceneElements(elements)).toBe(hashSceneElements(elements));
    });

    it("changes when elements change", () => {
      const before = hashSceneElements([createTestElement({ id: "a" })]);
      const after = hashSceneElements([
        createTestElement({ id: "a" }),
        createTestElement({ id: "b" }),
      ]);
      expect(after).not.toBe(before);
    });

    it("ignores deleted elements", () => {
      const alive = createTestElement({ id: "a", isDeleted: false });
      const withDeleted = [
        alive,
        createTestElement({ id: "dead", isDeleted: true }),
      ];
      expect(hashSceneElements(withDeleted)).toBe(hashSceneElements([alive]));
    });
  });
}); // end describe("sceneStorage")
