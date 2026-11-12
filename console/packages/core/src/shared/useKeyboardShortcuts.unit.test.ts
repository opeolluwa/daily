import { useKeyboardShortcuts } from "./useKeyboardShortcuts";

type KeydownHandler = (e: KeyboardEvent) => void;

const { handlers, mockUseEventListener } = vi.hoisted(() => {
  const handlers = new Map<string, KeydownHandler[]>();
  const mockUseEventListener = (_target: unknown, event: string, handler: KeydownHandler): void => {
    const existing = handlers.get(event) ?? [];
    existing.push(handler);
    handlers.set(event, existing);
  };
  return { handlers, mockUseEventListener };
});

vi.mock("@vueuse/core", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@vueuse/core")>();
  return { ...actual, useEventListener: mockUseEventListener };
});

function fireKeydown(overrides: Partial<KeyboardEvent> = {}): KeyboardEvent {
  const event = {
    key: "",
    metaKey: false,
    ctrlKey: false,
    shiftKey: false,
    altKey: false,
    target: { tagName: "CANVAS" },
    preventDefault: vi.fn(),
    ...overrides,
  } as unknown as KeyboardEvent;

  for (const handler of handlers.get("keydown") ?? []) {
    handler(event);
  }

  return event;
}

describe("useKeyboardShortcuts", () => {
  beforeEach(() => {
    handlers.clear();
    vi.stubGlobal("document", {});
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("prevents the browser default and invokes the handler for a matched shortcut", () => {
    const handler = vi.fn();
    useKeyboardShortcuts({ backspace: handler });

    const event = fireKeydown({ key: "Backspace" });

    expect(handler).toHaveBeenCalledWith(event);
    expect(event.preventDefault).toHaveBeenCalled();
  });

  it("prevents the browser default for the Delete key", () => {
    const handler = vi.fn();
    useKeyboardShortcuts({ delete: handler });

    const event = fireKeydown({ key: "Delete" });

    expect(handler).toHaveBeenCalledWith(event);
    expect(event.preventDefault).toHaveBeenCalled();
  });

  it("does not prevent the browser default for an unmatched key", () => {
    const handler = vi.fn();
    useKeyboardShortcuts({ backspace: handler });

    const event = fireKeydown({ key: "x" });

    expect(handler).not.toHaveBeenCalled();
    expect(event.preventDefault).not.toHaveBeenCalled();
  });

  it("skips shortcuts when an input element has focus", () => {
    class FakeHTMLElement {
      tagName: string;
      isContentEditable = false;

      constructor(tagName: string) {
        this.tagName = tagName;
      }
    }
    vi.stubGlobal("HTMLElement", FakeHTMLElement);

    const handler = vi.fn();
    useKeyboardShortcuts({ backspace: handler });

    const event = fireKeydown({
      key: "Backspace",
      target: new FakeHTMLElement("INPUT") as unknown as EventTarget,
    });

    expect(handler).not.toHaveBeenCalled();
    expect(event.preventDefault).not.toHaveBeenCalled();
  });

  it("runs shortcuts marked with usingInput when an input has focus", () => {
    class FakeHTMLElement {
      tagName: string;
      isContentEditable = false;

      constructor(tagName: string) {
        this.tagName = tagName;
      }
    }
    vi.stubGlobal("HTMLElement", FakeHTMLElement);

    const handler = vi.fn();
    useKeyboardShortcuts({ backspace: { handler, usingInput: true } });

    const event = fireKeydown({
      key: "Backspace",
      target: new FakeHTMLElement("TEXTAREA") as unknown as EventTarget,
    });

    expect(handler).toHaveBeenCalledWith(event);
    expect(event.preventDefault).toHaveBeenCalled();
  });
});
