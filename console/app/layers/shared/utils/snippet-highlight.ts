import hljs from "highlight.js/lib/common";

export const hlLanguageMap: Record<string, string> = {
  C: "c",
  "C++": "cpp",
  "C#": "csharp",
  Rust: "rust",
  Go: "go",
  Python: "python",
  Ruby: "ruby",
  PHP: "php",
  JavaScript: "javascript",
  TypeScript: "typescript",
  JSX: "javascript",
  TSX: "typescript",
  HTML: "xml",
  CSS: "css",
  SCSS: "scss",
  Less: "less",
  Bash: "bash",
  Zsh: "bash",
  PowerShell: "powershell",
  SQL: "sql",
  JSON: "json",
  YAML: "yaml",
  XML: "xml",
  Markdown: "markdown",
  Java: "java",
  Swift: "swift",
  Kotlin: "kotlin",
  Scala: "scala",
  Haskell: "haskell",
  Erlang: "erlang",
  Elixir: "elixir",
  R: "r",
  "Objective-C": "objectivec",
  GraphQL: "graphql",
  Docker: "dockerfile",
  "Docker Compose": "yaml",
  Makefile: "makefile",
  Vue: "xml",
  Svelte: "xml",
  React: "javascript",
  "Node.js": "javascript",
  Deno: "javascript",
  Bun: "javascript",
  Angular: "typescript",
};

export function highlightSnippet(
  code: string,
  language?: string | null,
): string {
  const lang = language ? hlLanguageMap[language] : undefined;
  if (lang && hljs.getLanguage(lang)) {
    return hljs.highlight(code, { language: lang }).value;
  }
  return hljs.highlightAuto(code).value;
}
