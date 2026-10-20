<script lang="ts">
  import { onMount } from "svelte";
  import { SvelteSet } from "svelte/reactivity";
  import { fade } from "svelte/transition";
  import * as api from "./lib/api";
  import { folderName, setKnownFolders } from "./lib/format";
  import { t } from "./lib/i18n";
  import { setMacMenu } from "./lib/menu";
  import FolderPanel from "./FolderPanel.svelte";
  import ResultCard from "./ResultCard.svelte";

  let query = $state("");
  let folders = $state<api.Folder[]>([]);
  let hits = $state<api.Hit[]>([]);
  let showAll = $state(false);
  let partial = $state(false);
  let noResults = $state("");
  // The footer is there from the first frame at launch, then fades once the index is ready.
  let status = $state({ text: t.updatingStart, error: false, progress: true, shown: true });
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
  let panelOpen = $state(false);
  let list = $state<HTMLElement>();
  let fadeTop = $state(false);
  let fadeBottom = $state(false);
  let statusTimer: ReturnType<typeof setTimeout> | undefined;
  const reduced = matchMedia("(prefers-reduced-motion: reduce)").matches;

  // Messages fade after a few seconds so the window keeps only the search. Errors stay until the
  // next message, and progress lines never cover an error that is still shown.
  function say(text: string, kind: "info" | "progress" | "error" = "info") {
    if (kind === "progress" && status.error && status.shown) return;
    status = { text, error: kind === "error", progress: kind === "progress", shown: true };
    clearTimeout(statusTimer);
    if (kind !== "error") statusTimer = setTimeout(() => (status.shown = false), 4000);
  }

  // Answers can arrive out of order when an indexing and a removal overlap; only the latest
  // request may set the list, or a removed folder could come back.
  let foldersRequest = 0;

  // The offer comes back whenever no folder is left, unless it was declined.
  async function loadFolders() {
    const mine = ++foldersRequest;
    const listed = await api.listFolders();
    if (mine !== foldersRequest) return;
    folders = listed;
    askConsent = folders.length === 0 && !declinedBefore();
  }

  async function index(path: string, report = true): Promise<boolean> {
    busy += 1;
    reading.add(path);
    try {
      const summary = await api.indexFolder(path);
      await loadFolders();
      if (report) summarize(path, summary);
      else runSearch();
      return true;
    } catch (e) {
      say(t.cannotRead(folderName(path), String(e)), "error");
      return false;
    } finally {
      reading.delete(path);
      busy -= 1;
    }
  }

  function summarize(path: string, report: api.Report) {
    say(
      t.ready(folderName(path), {
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
      say(t.removed(folderName(path)));
      runSearch();
    } catch (e) {
      say(t.cannotRemove(String(e)), "error");
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
    let known: api.KnownFolder[];
    try {
      known = await api.defaultFolders();
    } catch (e) {
      say(t.actionFailed(String(e)), "error");
      return;
    }
    setKnownFolders(known);
    for (const folder of known) await index(folder.path);
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

  // Where a result lives, from its indexed folder: "Documents › Maison" rather than the full path.
  function location(path: string) {
    const root = folders.find(
      (folder) => path.startsWith(folder.path) && /[\\/]/.test(path[folder.path.length] ?? ""),
    );
    if (!root) return path;
    const inner = path.slice(root.path.length + 1).split(/[\\/]/).slice(0, -1);
    return [folderName(root.path), ...inner].join(" › ");
  }

  async function act(action: (path: string) => Promise<void>, path: string) {
    try {
      await action(path);
    } catch (e) {
      say(t.actionFailed(String(e)), "error");
    }
  }

  // A slower answer to an earlier query must not replace the results of a later one.
  let shownQuery = "";

  async function runSearch() {
    const text = query.trim();
    const mine = ++searchNumber;
    // The engine ignores one-letter words, so a single letter would only report "no results".
    if (Array.from(text).length < 2) {
      hits = [];
      partial = false;
      noResults = "";
      return;
    }
    try {
      const found = await api.search(text);
      if (mine !== searchNumber) return;
      hits = found.hits;
      partial = !found.every_word && found.hits.length > 0;
      // A new query collapses the list; the same query run again, after a folder is read, keeps
      // what the user expanded.
      if (text !== shownQuery) showAll = false;
      shownQuery = text;
      noResults = found.hits.length ? "" : t.noResults(text);
    } catch (e) {
      if (mine === searchNumber) say(t.searchFailed(String(e)), "error");
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

  // The folders live in the application menu on macOS, like settings in other Mac applications.
  // Elsewhere the same shortcut opens them, without adding a menu bar to the window.
  const onMac = document.documentElement.classList.contains("mac");

  // The edges of the list fade only where rows are actually hidden: above once it is scrolled,
  // below while more rows follow.
  function updateFades() {
    if (!list) return;
    fadeTop = list.scrollTop > 0;
    fadeBottom = list.scrollTop + list.clientHeight < list.scrollHeight - 1;
  }

  $effect(() => {
    void [hits, showAll, partial, noResults];
    updateFades();
  });

  function closePanel() {
    panelOpen = false;
    field.focus();
  }

  // If the macOS menu could not be built, Command-comma still opens the folders.
  let menuReady = false;

  function onWindowKeydown(event: KeyboardEvent) {
    if (event.key !== ",") return;
    if (onMac ? event.metaKey && !menuReady : event.ctrlKey) panelOpen = true;
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
    api.onProgress((p) => say(t.updating(p.seen), "progress"));
    if (onMac)
      setMacMenu(() => (panelOpen = true))
        .then(() => (menuReady = true))
        .catch(() => {});
    field.focus();
    (async () => {
      try {
        setKnownFolders(await api.defaultFolders());
      } catch {
        // The folders then keep the names they have on disk.
      }
      try {
        await loadFolders();
      } catch (e) {
        say(t.cannotReadIndex(String(e)), "error");
        return;
      }
      if (!folders.length) status.shown = false;
      runSearch();
      // Only changed files are read again. A folder removed since launch is skipped, or reading
      // it would bring it back.
      let allRead = true;
      for (const path of folders.map((folder) => folder.path)) {
        if (stillListed(path) && !(await index(path, false))) allRead = false;
      }
      // An unreadable folder keeps its error on screen instead of a "ready" that would hide it.
      if (folders.length && allRead) {
        const total = folders.reduce((sum, folder) => sum + folder.files, 0);
        say(t.indexReady(folders.length, total));
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
      <button type="button" class="allow" disabled={busy > 0} onclick={allowDefaultFolders}
        >{t.consentAllow}</button
      >
      <button type="button" onclick={declineDefaultFolders}>{t.consentLater}</button>
    </div>
  {/if}
  <div
    class="results"
    class:fade-top={fadeTop}
    class:fade-bottom={fadeBottom}
    bind:this={list}
    onscroll={updateFades}
  >
    {#if noResults}
      <p class="empty">{noResults}</p>
    {/if}
    {#if partial}
      <p class="partial">{t.partialMatch}</p>
    {/if}
    {#each showAll ? hits : hits.slice(0, 3) as hit (hit.path)}
      <ResultCard
        {hit}
        where={location(hit.path)}
        onopen={(p) => act(api.openFile, p)}
        onreveal={(p) => act(api.revealFile, p)}
      />
    {/each}
    {#if !showAll && hits.length > 3}
      <button type="button" class="more" onclick={() => (showAll = true)}>{t.showMore(hits.length - 3)}</button>
    {/if}
  </div>
</main>

<svelte:window onkeydown={onWindowKeydown} onresize={updateFades} />

<p
  class="status"
  class:error={status.error}
  class:hidden={!status.shown}
  aria-live={status.progress ? "off" : "polite"}
>
  {status.text}
</p>

{#if panelOpen}
  <FolderPanel
    {folders}
    {reading}
    busy={busy > 0}
    onadd={addFolder}
    onremove={removeFolder}
    onclose={closePanel}
  />
{/if}

<style>
  main {
    width: 100%;
    max-width: 720px;
    margin: 0 auto;
    /* 136.5px is the height of the title and its margin plus half the search field, so the middle
       of the field sits at the vertical center of the window. */
    padding: calc(50vh - 136.5px) 24px 48px;
  }

  /* During a search the page itself stays still: only the results scroll, under the field. */
  main.active {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding-top: 48px;
    padding-bottom: 0;
  }

  /* The gradient spans the word only, as on the icon: dark on the left, light on the right. */
  h1 {
    width: fit-content;
    margin: 0 auto 24px;
    font-size: 56px;
    font-weight: 600;
    letter-spacing: 0.14em;
    color: transparent;
    background: linear-gradient(90deg, var(--title-from), var(--title-to));
    -webkit-background-clip: text;
    background-clip: text;
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
    position: fixed;
    right: 0;
    bottom: 12px;
    left: 0;
    margin: 0;
    padding: 0 24px;
    font-size: 12px;
    text-align: center;
    color: var(--muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    pointer-events: none;
    transition: opacity 0.6s ease;
  }

  .status.hidden {
    opacity: 0;
  }

  @media (prefers-reduced-motion: reduce) {
    .status {
      transition: none;
    }
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

  .consent .allow:disabled {
    opacity: 0.5;
    cursor: default;
  }

  .more {
    display: block;
    margin: 4px auto 0;
    padding: 4px 8px;
    font: inherit;
    font-size: 13px;
    color: var(--muted);
    background: none;
    border: none;
    cursor: pointer;
    transition: color 0.15s;
  }

  .more:hover {
    color: var(--accent);
  }

  .results {
    flex: 1;
    min-height: 0;
    margin-top: 12px;
    padding: 12px 0 48px;
    overflow-y: auto;
  }

  .results.fade-top {
    mask-image: linear-gradient(to bottom, transparent 0, #000 40px);
  }

  .results.fade-bottom {
    mask-image: linear-gradient(to bottom, #000 calc(100% - 48px), transparent);
  }

  .results.fade-top.fade-bottom {
    mask-image: linear-gradient(to bottom, transparent 0, #000 40px, #000 calc(100% - 48px), transparent);
  }

  .partial {
    margin: 0 4px 12px;
    font-size: 13px;
    text-align: center;
    color: var(--muted);
  }

  .empty {
    margin: 32px 4px;
    text-align: center;
    color: var(--muted);
  }
</style>
