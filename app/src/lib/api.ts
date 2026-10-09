import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface Folder {
  path: string;
  files: number;
}

export interface Hit {
  path: string;
  name: string;
  ext: string;
  size: number;
  mtime: number;
  snippet: string | null;
  score: number;
}

export interface Report {
  seen: number;
  indexed: number;
  unchanged: number;
  removed: number;
  name_only: number;
  skipped_sensitive: number;
  errors: number;
  millis: number;
}

export interface Progress {
  seen: number;
  indexed: number;
  current: string;
}

export const chooseFolder = () => invoke<string | null>("choose_folder");
export const listFolders = () => invoke<Folder[]>("list_folders");
export const indexFolder = (path: string) => invoke<Report>("index_folder", { path });
export const forgetFolder = (path: string) => invoke<number>("forget_folder", { path });
export const search = (query: string) => invoke<Hit[]>("search", { query });
export const openFile = (path: string) => invoke<void>("open_file", { path });
export const revealFile = (path: string) => invoke<void>("reveal_file", { path });

export const onProgress = (handler: (progress: Progress) => void) =>
  listen<Progress>("progress", (event) => handler(event.payload));
