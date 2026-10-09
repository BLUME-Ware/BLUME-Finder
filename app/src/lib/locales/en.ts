export interface Summary {
  seen: number;
  read: number;
  nameOnly: number;
  changed: boolean;
}

const files = (n: number) => `${n} ${n === 1 ? "file" : "files"}`;

export const en = {
  searchPlaceholder:
    'Describe what you are looking for: "rental contract", "train tickets Toulouse"…',
  foldersLabel: "Indexed folders",
  addFolder: "Add a folder",
  folderChip: (name: string, count: number) => `${name} · ${files(count)}`,
  removeFolderTitle: "Remove this folder from the index (the files are not touched)",
  removeFolderLabel: (name: string) => `Remove ${name} from the index`,
  progress: (seen: number, current: string) => `Reading… ${files(seen)} seen. ${current}`,
  ready: (name: string, s: Summary) => {
    let text = `"${name}" is ready: ${files(s.seen)}`;
    if (s.changed) {
      text += `, ${s.read} read in full`;
      if (s.nameOnly > 0) text += ` and ${s.nameOnly} findable by name only`;
    } else {
      text += ", nothing changed";
    }
    return `${text}.`;
  },
  cannotRead: (name: string, error: string) => `Cannot read "${name}": ${error}`,
  removed: (name: string) => `"${name}" is removed from the index. The files were not touched.`,
  cannotRemove: (error: string) => `Cannot remove this folder: ${error}`,
  cannotReadIndex: (error: string) => `Cannot read the index: ${error}`,
  noResults: (query: string) =>
    `No results for "${query}". Scans and images are found by their name only.`,
  searchFailed: (error: string) => `Search failed: ${error}`,
  nameOnlyHit: "Found by its name (the content of this type of file is not read).",
  open: "Open",
  reveal: "Show in folder",
  actionFailed: (error: string) => `Action failed: ${error}`,
  bytes: (n: number) => `${n} B`,
  kilobytes: (n: number) => `${n} KB`,
  megabytes: (n: string) => `${n} MB`,
};

export type Messages = typeof en;
