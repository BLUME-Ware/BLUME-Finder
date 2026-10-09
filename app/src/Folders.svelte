<script lang="ts">
  import type { Folder } from "./lib/api";
  import { shortName } from "./lib/format";
  import { t } from "./lib/i18n";

  interface Props {
    folders: Folder[];
    busy: boolean;
    onadd: () => void;
    onremove: (path: string) => void;
  }

  let { folders, busy, onadd, onremove }: Props = $props();
</script>

<section class="folders" aria-label={t.foldersLabel}>
  <ul>
    {#each folders as folder (folder.path)}
      <li title={folder.path}>
        <span class="name">{t.folderChip(shortName(folder.path), folder.files)}</span>
        <button
          type="button"
          title={t.removeFolderTitle}
          aria-label={t.removeFolderLabel(shortName(folder.path))}
          onclick={() => onremove(folder.path)}>×</button
        >
      </li>
    {/each}
  </ul>
  <button type="button" class="add" disabled={busy} onclick={onadd}>{t.addFolder}</button>
</section>

<style>
  .folders {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: center;
    gap: 6px;
    margin-top: 14px;
  }

  ul {
    display: contents;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  li {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    max-width: 100%;
    padding: 2px 4px 2px 10px;
    font-size: 12px;
    color: var(--muted);
    background: var(--surface);
    border-radius: 999px;
  }

  .name {
    max-width: 260px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  li button {
    padding: 0 6px;
    font: inherit;
    font-size: 14px;
    line-height: 20px;
    color: var(--muted);
    background: none;
    border: none;
    border-radius: 999px;
    cursor: pointer;
    transition: color 0.15s;
  }

  li button:hover {
    color: var(--error);
  }

  .add {
    padding: 2px 8px;
    font: inherit;
    font-size: 12px;
    color: var(--muted);
    background: none;
    border: none;
    cursor: pointer;
    transition: color 0.15s;
  }

  .add:hover {
    color: var(--accent);
  }

  .add:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
