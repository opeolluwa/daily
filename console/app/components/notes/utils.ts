export function isHtml(s: string): boolean {
  const t = s.trimStart();
  return t.startsWith("<") && /<[a-z][\s\S]*>/i.test(t);
}

export function isLikelyMarkdown(text: string): boolean {
  const t = text.trim();
  if (!t) return false;
  const patterns = [
    /^#{1,6}\s/m,
    /\*\*[^*]+\*\*/,
    /_[^_]+_/,
    /`[^`]+`/,
    /```/,
    /^\s{4}.*$/m,
    /^\s*[-*+]\s/m,
    /^\s*\d+\.\s/m,
    /\[[^[\]]*\]\([^)]+\)/,
    /!\[[^[\]]*]\([^)]+\)/,
    /^>\s/m,
    /^---+$/m,
    /^[-*_]\s*[-*_]\s*[-*_]/,
  ];
  return patterns.some((p) => p.test(text));
}
