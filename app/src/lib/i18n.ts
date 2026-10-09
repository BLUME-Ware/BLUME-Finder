import { en, type Messages } from "./locales/en";
import { fr } from "./locales/fr";

const catalogs: Record<string, Messages> = { en, fr };

function pick(languages: readonly string[]): string {
  for (const language of languages) {
    const base = language.toLowerCase().split("-")[0];
    if (base in catalogs) return base;
  }
  return "en";
}

export const locale = pick(navigator.languages ?? [navigator.language]);
export const t: Messages = catalogs[locale];
