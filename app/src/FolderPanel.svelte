<script lang="ts">
  import { fade, scale } from "svelte/transition";
  import type { Folder } from "./lib/api";
  import { t } from "./lib/i18n";
  import Folders from "./Folders.svelte";

  interface Props {
    folders: Folder[];
    reading: ReadonlySet<string>;
    busy: boolean;
    onadd: () => void;
    onremove: (path: string) => void;
    onclose: () => void;
  }

  let { folders, reading, busy, onadd, onremove, onclose }: Props = $props();
  let panel: HTMLElement;
  const reduced = matchMedia("(prefers-reduced-motion: reduce)").matches;

  $effect(() => panel.focus());

  function onKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") onclose();
  }
</script>

<svelte:window onkeydown={onKeydown} />

<!-- A click outside the panel closes it; the keyboard closes it with Escape. -->
<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="backdrop" onclick={onclose} transition:fade={{ duration: reduced ? 0 : 200 }}>
  <div
    class="panel"
    bind:this={panel}
    transition:scale={{ duration: reduced ? 0 : 200, start: 0.96 }}
    role="dialog"
    aria-modal="true"
    aria-label={t.foldersLabel}
    tabindex="-1"
    onclick={(event) => event.stopPropagation()}
  >
    <h2>{t.foldersLabel}</h2>
    <Folders {folders} {reading} {busy} {onadd} {onremove} />
    <button type="button" class="close" onclick={onclose}>{t.close}</button>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    background: rgba(0, 0, 0, 0.25);
  }

  .panel {
    width: 100%;
    max-width: 480px;
    padding: 20px 24px;
    background: var(--background);
    border: 1px solid var(--border);
    border-radius: 14px;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.25);
    outline: none;
  }

  h2 {
    margin: 0 0 4px;
    font-size: 15px;
    font-weight: 600;
    text-align: center;
  }

  .close {
    display: block;
    margin: 18px auto 0;
    padding: 6px 18px;
    font: inherit;
    font-size: 13px;
    color: var(--text);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
  }
</style>
