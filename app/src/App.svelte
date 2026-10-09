<script lang="ts">
  import { onMount } from "svelte";
  import * as api from "./lib/api";
  import { shortName } from "./lib/format";
  import { t } from "./lib/i18n";
  import Folders from "./Folders.svelte";
  import ResultCard from "./ResultCard.svelte";

  let query = $state("");
  let folders = $state<api.Folder[]>([]);
  let hits = $state<api.Hit[]>([]);
  let noResults = $state("");
  let status = $state({ text: "", error: false });
  let busy = $state(0);
  let field: HTMLInputElement;
  let searchNumber = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;

  function say(text: string, error = false) {
    status = { text, error };
  }

  async function loadFolders() {
    folders = await api.listFolders();
  }

  async function index(path: string) {
    busy += 1;
    try {
      const report = await api.indexFolder(path);
      await loadFolders();
      summarize(path, report);
    } catch (e) {
      say(t.cannotRead(shortName(path), String(e)), true);
    } finally {
      busy -= 1;
    }
  }

  function summarize(path: string, report: api.Report) {
    say(
      t.ready(shortName(path), {
        seen: report.seen,
        read: Math.max(report.indexed - report.name_only, 0),
        nameOnly: report.name_only,
        changed: report.indexed > 0,
      }),
    );
    runSearch();
  }

  async function addFolder() {
    const path = await api.chooseFolder();
    if (path) await index(path);
  }

  async function removeFolder(path: string) {
    try {
      await api.forgetFolder(path);
      await loadFolders();
      say(t.removed(shortName(path)));
      runSearch();
    } catch (e) {
      say(t.cannotRemove(String(e)), true);
    }
  }

  async function act(action: (path: string) => Promise<void>, path: string) {
    try {
      await action(path);
    } catch (e) {
      say(t.actionFailed(String(e)), true);
    }
  }

  // A slower answer to an earlier query must not replace the results of a later one.
  async function runSearch() {
    const text = query.trim();
    const mine = ++searchNumber;
    if (!text) {
      hits = [];
      noResults = "";
      return;
    }
    try {
      const found = await api.search(text);
      if (mine !== searchNumber) return;
      hits = found;
      noResults = found.length ? "" : t.noResults(text);
    } catch (e) {
      if (mine === searchNumber) say(t.searchFailed(String(e)), true);
    }
  }

  function scheduleSearch() {
    clearTimeout(timer);
    timer = setTimeout(runSearch, 150);
  }

  function onKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      query = "";
      runSearch();
    }
  }

  onMount(() => {
    api.onProgress((p) => say(t.progress(p.seen, shortName(p.current))));
    field.focus();
    (async () => {
      try {
        await loadFolders();
      } catch (e) {
        say(t.cannotReadIndex(String(e)), true);
        return;
      }
      runSearch();
      // Only changed files are read again.
      for (const folder of folders) await index(folder.path);
    })();
  });
</script>

<main class:active={query.trim() !== ""}>
  <h1>BLUME</h1>
  <input
    bind:this={field}
    bind:value={query}
    oninput={scheduleSearch}
    onkeydown={onKeydown}
    type="search"
    autocomplete="off"
    spellcheck="false"
    placeholder={t.searchPlaceholder}
  />
  <Folders {folders} busy={busy > 0} onadd={addFolder} onremove={removeFolder} />
  <p class="status" class:error={status.error} aria-live="polite">{status.text}</p>
  <div class="results">
    {#if noResults}
      <p class="empty">{noResults}</p>
    {/if}
    {#each hits as hit (hit.path)}
      <ResultCard {hit} onopen={(p) => act(api.openFile, p)} onreveal={(p) => act(api.revealFile, p)} />
    {/each}
  </div>
</main>

<style>
  main {
    width: 100%;
    max-width: 720px;
    margin: 0 auto;
    padding: 26vh 24px 32px;
    transition: padding-top 0.25s ease;
  }

  main.active {
    padding-top: 48px;
  }

  h1 {
    margin: 0 0 24px;
    text-align: center;
    font-size: 56px;
    font-weight: 600;
    letter-spacing: 0.14em;
  }

  input {
    width: 100%;
    padding: 14px 20px;
    font: inherit;
    font-size: 18px;
    color: var(--text);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 14px;
    outline: none;
    transition: border-color 0.15s;
  }

  input::placeholder {
    color: var(--muted);
  }

  input:focus {
    border-color: var(--accent);
  }

  .status {
    margin: 14px 4px 12px;
    font-size: 13px;
    text-align: center;
    color: var(--muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .status.error {
    color: var(--error);
    white-space: normal;
  }

  .empty {
    margin: 32px 4px;
    text-align: center;
    color: var(--muted);
  }
</style>
