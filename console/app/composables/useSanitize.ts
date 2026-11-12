import DOMPurify from "dompurify";

export function sanitizeHtml(html: string): string {
  return DOMPurify.sanitize(html);
}

export function sanitizeSvg(html: string): string {
  return DOMPurify.sanitize(html, {
    USE_PROFILES: { svg: true, svgFilters: true },
  });
}
