<script lang="ts">
  import type { Provider } from '../types/provider';
  import { providerStore, selectedId } from '../stores/providerStore';

  interface Props {
    provider: Provider;
  }

  let { provider }: Props = $props();

  function getDomain(url: string): string {
    try {
      return new URL(url).hostname;
    } catch {
      return url;
    }
  }

  function handleClick() {
    providerStore.select(provider.id);
  }
</script>

<button
  type="button"
  class="provider-card"
  class:selected={$selectedId === provider.id}
  on:click={handleClick}
>
  <div class="accent" aria-hidden="true"></div>
  <div class="body">
    <div class="name">{provider.name}</div>
    <div class="domain">{getDomain(provider.base_url)}</div>
  </div>
</button>

<style>
  .provider-card {
    width: 100%;
    text-align: left;
    padding: 12px 14px;
    border-radius: 12px;
    cursor: pointer;

    display: grid;
    grid-template-columns: 5px 1fr;
    gap: 12px;
    align-items: center;

    border: 1px solid var(--border);
    background: rgba(255, 255, 255, 0.03);
    color: var(--text);

    transition:
      transform 0.15s ease,
      border-color 0.15s ease,
      background 0.15s ease,
      box-shadow 0.15s ease;
    margin-bottom: 8px;
  }

  .accent {
    width: 5px;
    height: 32px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.08);
  }

  .provider-card:hover {
    background: rgba(255, 255, 255, 0.06);
    border-color: var(--border-2);
    transform: translateY(-1px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  }

  .provider-card:active {
    transform: translateY(0);
  }

  .provider-card.selected {
    background: rgba(109, 124, 255, 0.1);
    border-color: rgba(109, 124, 255, 0.35);
    box-shadow: 0 12px 40px rgba(109, 124, 255, 0.1);
  }

  .provider-card.selected .accent {
    background: var(--accent);
    box-shadow: 0 0 8px rgba(109, 124, 255, 0.4);
  }

  .name {
    font-weight: 500;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.9);
    line-height: 1.3;
  }

  .domain {
    font-size: 12px;
    color: var(--muted);
    margin-top: 3px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
