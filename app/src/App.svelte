<script lang="ts">
  import { onMount } from "svelte";
  import { SvelteSet } from "svelte/reactivity";
  import { fade } from "svelte/transition";
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
  // Both are filled before any await, so a removal and a reading can never start on the same
  // folder at once.
  const reading = new SvelteSet<string>();
  const removing = new Set<string>();
  let field: HTMLInputElement;
  let box: HTMLElement;
  let searchNumber = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let view: HTMLElement;
  let active = $derived(query.trim() !== "");
  let fieldTop: number | undefined;
  let tip = $state("");
  let askConsent = $state(false);
  const reduced = matchMedia("(prefers-reduced-motion: reduce)").matches;

  function say(text: string, error = false) {
    status = { text, error };
  }

  async function loadFolders() {
    folders = await api.listFolders();
  }

  async function index(path: string) {
    busy += 1;
    reading.add(path);
    try {
      const report = await api.indexFolder(path);
      await loadFolders();
      summarize(path, report);
    } catch (e) {
      say(t.cannotRead(shortName(path), String(e)), true);
    } finally {
      reading.delete(path);
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
    if (reading.has(path)) return;
    removing.add(path);
    try {
      await api.forgetFolder(path);
      await loadFolders();
      say(t.removed(shortName(path)));
      runSearch();
    } catch (e) {
      say(t.cannotRemove(String(e)), true);
    } finally {
      removing.delete(path);
    }
  }

  // A refusal is kept in the web view's storage: if that storage is lost, the question is only
  // asked again.
  const declinedKey = "firstLaunchDeclined";

  function declinedBefore() {
    try {
      return localStorage.getItem(declinedKey) === "1";
    } catch {
      return false;
    }
  }

  async function allowDefaultFolders() {
    askConsent = false;
    let paths: string[];
    try {
      paths = await api.defaultFolders();
    } catch (e) {
      say(t.actionFailed(String(e)), true);
      return;
    }
    for (const path of paths) await index(path);
  }

  function declineDefaultFolders() {
    askConsent = false;
    try {
      localStorage.setItem(declinedKey, "1");
    } catch {
      // Without storage the question comes back at the next launch.
    }
  }

  function stillListed(path: string) {
    return !removing.has(path) && folders.some((folder) => folder.path === path);
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
    // The engine ignores one-letter words, so a single letter would only report "no results".
    if (Array.from(text).length < 2) {
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

  // The position is set by the layout alone, so it follows a window resize in the same frame. Only
  // the switch between centered and raised is animated, from where the field was to where it is.
  $effect.pre(() => {
    void active;
    fieldTop = box?.offsetTop;
  });

  $effect(() => {
    void active;
    if (fieldTop === undefined) return;
    const shift = fieldTop - box.offsetTop;
    if (shift === 0) return;
    view.animate([{ transform: `translateY(${shift}px)` }, { transform: "none" }], {
      duration: 300,
      easing: "cubic-bezier(0.2, 0, 0, 1)",
    });
  });

  // Each tip is typed, held, then erased. With reduced motion the tips are only swapped.
  function cycleTips() {
    let index = 0;
    let length = 0;
    let erasing = false;
    let handle: ReturnType<typeof setTimeout>;
    const step = () => {
      const text = t.searchTips[index];
      if (reduced) {
        tip = text;
        index = (index + 1) % t.searchTips.length;
        handle = setTimeout(step, 3500);
        return;
      }
      length += erasing ? -1 : 1;
      tip = text.slice(0, length);
      if (!erasing && length === text.length) {
        erasing = true;
        handle = setTimeout(step, 2000);
      } else if (erasing && length === 0) {
        erasing = false;
        index = (index + 1) % t.searchTips.length;
        handle = setTimeout(step, 400);
      } else {
        handle = setTimeout(step, erasing ? 20 : 45);
      }
    };
    step();
    return () => clearTimeout(handle);
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
      askConsent = folders.length === 0 && !declinedBefore();
      runSearch();
      // Only changed files are read again. A folder removed since launch is skipped, or reading
      // it would bring it back.
      for (const path of folders.map((folder) => folder.path)) {
        if (stillListed(path)) await index(path);
      }
    })();
    return cycleTips();
  });
</script>

<main bind:this={view} class:active>
  <h1>BLUME</h1>
  <div class="field" bind:this={box}>
    <input
      bind:this={field}
      bind:value={query}
      oninput={scheduleSearch}
      onkeydown={onKeydown}
      type="search"
      autocomplete="off"
      spellcheck="false"
      aria-label={t.searchLabel}
    />
    <!-- A placeholder cannot fade letter by letter, so the tip is drawn over the empty field. -->
    {#if !query}
      <span class="tip" aria-hidden="true">
        {#each Array.from(tip) as letter, i (i)}<span transition:fade={{ duration: reduced ? 0 : 220 }}
            >{letter}</span
          >{/each}
      </span>
    {/if}
  </div>
  {#if askConsent}
    <div class="consent">
      <p>{t.consentText}</p>
      <button type="button" class="allow" onclick={allowDefaultFolders}>{t.consentAllow}</button>
      <button type="button" onclick={declineDefaultFolders}>{t.consentLater}</button>
    </div>
  {:else}
    <Folders {folders} {reading} busy={busy > 0} onadd={addFolder} onremove={removeFolder} />
  {/if}
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
    /* 136.5px is the height of the title and its margin plus half the search field, so the middle
       of the field sits at the vertical center of the window. */
    padding: calc(50vh - 136.5px) 24px 32px;
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

  .field {
    position: relative;
  }

  input {
    display: block;
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

  .tip {
    position: absolute;
    top: 50%;
    left: 21px;
    right: 21px;
    overflow: hidden;
    font-size: 18px;
    color: var(--muted);
    white-space: pre;
    transform: translateY(-50%);
    pointer-events: none;
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

  .consent {
    margin-top: 18px;
    font-size: 13px;
    text-align: center;
    color: var(--muted);
  }

  .consent p {
    margin: 0 0 12px;
  }

  .consent button {
    margin: 0 4px;
    padding: 6px 16px;
    font: inherit;
    color: var(--text);
    background: var(--surface);
    border: none;
    border-radius: 8px;
    cursor: pointer;
  }

  .consent .allow {
    color: #ffffff;
    background: var(--accent);
  }

  .empty {
    margin: 32px 4px;
    text-align: center;
    color: var(--muted);
  }
</style>
