<script lang="ts">
  import type { Hit } from "./lib/api";
  import { date, size, snippetParts } from "./lib/format";
  import { t } from "./lib/i18n";

  interface Props {
    hit: Hit;
    where: string;
    onopen: (path: string) => void;
    onreveal: (path: string) => void;
  }

  let { hit, where, onopen, onreveal }: Props = $props();
</script>

<article>
  <button type="button" class="main" title="{hit.path} · {size(hit.size)}" onclick={() => onopen(hit.path)}>
    <span class="top">
      <span class="name">{hit.name}</span>
      <span class="date">{date(hit.mtime)}</span>
    </span>
    <span class="where">{where}</span>
    {#if hit.snippet}
      <span class="snippet">
        {#each snippetParts(hit.snippet) as part, i (i)}
          {#if part.match}<mark>{part.text}</mark>{:else}{part.text}{/if}
        {/each}
      </span>
    {:else}
      <span class="snippet name-only">{t.nameOnlyHit}</span>
    {/if}
  </button>
  <button type="button" class="reveal" onclick={() => onreveal(hit.path)}>{t.reveal}</button>
</article>

<style>
  article {
    position: relative;
    margin-bottom: 6px;
    animation: appear 0.2s ease-out;
  }

  @keyframes appear {
    from {
      opacity: 0;
    }
  }

  /* The whole row opens the file; showing it in its folder stays one hover away. */
  .main {
    display: block;
    width: 100%;
    padding: 10px 16px;
    font: inherit;
    color: var(--text);
    text-align: left;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 10px;
    cursor: pointer;
    transition: background-color 0.15s;
  }

  .main:hover,
  .main:focus-visible {
    background: color-mix(in srgb, var(--accent) 12%, var(--surface));
    outline: none;
  }

  .top {
    display: flex;
    gap: 12px;
    align-items: baseline;
  }

  .name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    font-size: 15px;
    font-weight: 600;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .date {
    flex: none;
    font-size: 12px;
    color: var(--muted);
  }

  .where,
  .snippet {
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .where {
    font-size: 12px;
    color: var(--muted);
  }

  .snippet {
    margin-top: 4px;
    font-size: 13px;
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

  .reveal {
    position: absolute;
    right: 12px;
    bottom: 8px;
    padding: 2px 8px;
    font: inherit;
    font-size: 12px;
    color: var(--muted);
    background: var(--background);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    opacity: 0;
    transition:
      opacity 0.15s,
      color 0.15s;
  }

  article:hover .reveal,
  .reveal:focus-visible {
    opacity: 1;
  }

  .reveal:hover {
    color: var(--accent);
  }
</style>
