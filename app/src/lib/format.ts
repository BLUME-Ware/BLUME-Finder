import { locale, t } from "./i18n";

export function shortName(path: string): string {
  const parts = path.split(/[\\/]/).filter(Boolean);
  return parts.length ? parts[parts.length - 1] : path;
}

export function size(bytes: number): string {
  if (bytes < 1024) return t.bytes(bytes);
  if (bytes < 1024 * 1024) return t.kilobytes(Math.round(bytes / 1024));
  return t.megabytes((bytes / 1024 / 1024).toFixed(1));
}

export function date(seconds: number): string {
  if (!seconds) return "";
  return new Date(seconds * 1000).toLocaleDateString(locale, {
    day: "numeric",
    month: "short",
    year: "numeric",
  });
}

export const HL_START = "\u0001";
export const HL_END = "\u0002";

export interface SnippetPart {
  text: string;
  match: boolean;
}

// The parts are rendered as text, never as markup, since they come from the user's files.
export function snippetParts(snippet: string): SnippetPart[] {
  const parts: SnippetPart[] = [];
  let rest = snippet;
  while (rest.length) {
    const start = rest.indexOf(HL_START);
    if (start === -1) {
      parts.push({ text: rest, match: false });
      break;
    }
    const end = rest.indexOf(HL_END, start);
    if (end === -1) {
      parts.push({ text: rest.replace(HL_START, ""), match: false });
      break;
    }
    if (start > 0) parts.push({ text: rest.slice(0, start), match: false });
    parts.push({ text: rest.slice(start + 1, end), match: true });
    rest = rest.slice(end + 1);
  }
  return parts;
}
