import { marked } from "marked";
import katex from "katex";
import {
  Extension,
  StarterKit,
  BaseKeymap,
  Superscript,
  Subscript,
  Text,
  TextStyle,
  TextAlign,
  TextColor,
  Code,
  Heading,
  ListItem,
  BulletList,
  OrderedList,
  Link,
  Placeholder,
  UniqueID,
  BlockColor,
  ListIndent,
  PluginKey,
} from "@domternal/core";
import { Plugin } from "@domternal/pm/state";
import { DOMParser as PMDOMParser } from "@domternal/pm/model";
import { Details } from "@domternal/extension-details";
import { CodeBlockLowlight } from "@domternal/extension-code-block-lowlight";
import { createLowlight, all } from "lowlight";
import { Table } from "@domternal/extension-table";
import { Image } from "@domternal/extension-image";
import {
  MathInline,
  MathBlock,
  createKatexRenderer,
} from "@domternal/extension-math";
import "katex/dist/katex.min.css";

import {
  Emoji,
  emojis,
  createEmojiSuggestionRenderer,
} from "@domternal/extension-emoji";
import {
  BlockContextMenu,
  KeyboardReorder,
  SmartPaste,
} from "@domternal/extension-block-controls";

import { isLikelyMarkdown } from "./utils";

const lowlight = createLowlight(all);
const mathRenderer = createKatexRenderer(katex);

const MarkdownPaste = Extension.create({
  name: "markdownPaste",

  addProseMirrorPlugins() {
    return [
      new Plugin({
        key: new PluginKey("markdownPaste"),
        props: {
          handlePaste(view, event, _slice) {
            const clipboard = event.clipboardData;
            if (!clipboard) return false;

            if (clipboard.getData("text/html")) return false;

            const text = clipboard.getData("text/plain");
            if (!text || !isLikelyMarkdown(text)) return false;

            const mdHtml = marked.parse(text) as string;
            const wrapper = document.createElement("div");
            wrapper.innerHTML = mdHtml;
            const parsed = PMDOMParser.fromSchema(view.state.schema).parse(
              wrapper,
            );

            const tr = view.state.tr.replaceSelectionWith(parsed);
            view.dispatch(tr.scrollIntoView());
            return true;
          },
        },
      }),
    ];
  },
});

export const extensions = [
  StarterKit,
  UniqueID,
  BlockColor,
  ListIndent,
  Table,
  ListItem.configure({
    HTMLAttributes: { class: "notes_list_item" },
  }),
  BulletList.configure({
    HTMLAttributes: { class: "note_list_unordered" },
  }),
  OrderedList.configure({
    HTMLAttributes: { class: "note_list_ordered" },
  }),
  Superscript,
  Subscript,
  Text,
  BaseKeymap,
  Details,
  TextStyle,
  Code,
  TextAlign,
  TextColor,
  BlockContextMenu,

  SmartPaste,
  MarkdownPaste,
  KeyboardReorder,
  Heading.configure({
    levels: [1, 2, 3, 4, 5, 6],
    HTMLAttributes: { class: "notes_heading" },
  }),
  CodeBlockLowlight.configure({ lowlight }),
  MathInline.configure({ renderer: mathRenderer }),
  MathBlock.configure({ renderer: mathRenderer }),
  Emoji.configure({
    emojis,
    suggestion: { render: createEmojiSuggestionRenderer() },
  }),
  Link.configure({
    protocols: ["http:", "https:"],
    openOnClick: true,
    autolink: true,
    linkOnPaste: true,
    defaultProtocol: "https",
  }),
  Image.configure({
    //TODO: replace with actual upload handler that uploads to server and returns URL
    uploadHandler: async (file) => {
      const form = new FormData();
      form.append("file", file);
      const res = await fetch("/api/upload", { method: "POST", body: form });
      const { url } = await res.json();
      return url;
    },
  }),

  Placeholder.configure({
    placeholder: ({ node }) => {
      if (node.type.name === "heading") return "Enter a heading...";
      if (node.type.name === "codeBlock") return "// Write code here";
      if (node.type.name === "table") return "";
      return "Type something...";
    },
  }),
];
