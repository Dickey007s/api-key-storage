<script lang="ts">
  import { providerStore, filteredProviders, loading } from '../stores/providerStore';
  import SearchBox from './SearchBox.svelte';
  import CreateButton from './CreateButton.svelte';
  import ProviderCard from './ProviderCard.svelte';
</script>

<div class="provider-list">
  <div class="top">
    <SearchBox />
    <CreateButton />
  </div>

  <div class="list scroll-area">
    {#if $loading}
      <div class="loading">加载中...</div>
    {:else if $filteredProviders.length === 0}
      <div class="empty">暂无平台</div>
    {:else}
      {#each $filteredProviders as provider (provider.id)}
        <ProviderCard provider={provider} />
      {/each}
    {/if}
  </div>
</div>

<style>
  .provider-list {
    height: 100%;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .top {
    display: grid;
    gap: 10px;
  }

  .list {
    flex: 1;
    min-height: 0;
    margin-top: 14px;

    /* 让滚动条视觉更“内嵌” */
    padding-right: 10px;
  }

  .loading,
  .empty {
    text-align: center;
    color: var(--muted);
    padding: 20px;
    font-size: 13px;
  }
</style>