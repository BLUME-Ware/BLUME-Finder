<script lang="ts">
  import type { Hit } from "./lib/api";
  import { date, size, snippetParts } from "./lib/format";
  import { t } from "./lib/i18n";

  interface Props {
    hit: Hit;
    onopen: (path: string) => void;
    onreveal: (path: string) => void;
  }

  let { hit, onopen, onreveal }: Props = $props();

  const meta = $derived([size(hit.size), date(hit.mtime)].filter(Boolean).join(" · "));
</script>

<article>
  <h2>{hit.name}</h2>
  <p class="path">{hit.path} · {meta}</p>
  {#if hit.snippet}
    <p class="snippet">
      {#each snippetParts(hit.snippet) as part, i (i)}
        {#if part.match}<mark>{part.text}</mark>{:else}{part.text}{/if}
      {/each}
    </p>
  {:else}
    <p class="snippet name-only">{t.nameOnlyHit}</p>
  {/if}
  <div class="actions">
    <button type="button" onclick={() => onopen(hit.path)}>{t.open}</button>
    <button type="button" onclick={() => onreveal(hit.path)}>{t.reveal}</button>
  </div>
</article>

<style>
  article {
    margin-bottom: 10px;
    padding: 16px 20px;
    background: var(--surface);
    border-radius: 14px;
  }

  h2 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    overflow-wrap: anywhere;
  }

  .path {
    margin: 2px 0 0;
    font-size: 12px;
    color: var(--muted);
    overflow-wrap: anywhere;
  }

  .snippet {
    margin: 10px 0 0;
    font-size: 14px;
    overflow-wrap: anywhere;
  }

  .name-only {
    color: var(--muted);
    font-style: italic;
  }

  mark {
    padding: 0 2px;
    color: inherit;
    background: var(--highlight);
    border-radius: 3px;
  }

  .actions {
    display: flex;
    gap: 8px;
    margin-top: 12px;
  }

  button {
    padding: 4px 12px;
    font: inherit;
    font-size: 13px;
    color: var(--text);
    background: var(--background);
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: color 0.15s;
  }

  button:hover {
    color: var(--accent);
  }
</style>
