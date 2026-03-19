<script lang="ts">
  import './app.css';
  import { onMount } from 'svelte';
  import { providerStore, creating } from './lib/stores/providerStore';
  import ProviderList from './lib/components/ProviderList.svelte';
  import ProviderDetail from './lib/components/ProviderDetail.svelte';
  import ProviderForm from './lib/components/ProviderForm.svelte';
  import Toast from './lib/components/Toast.svelte';

  onMount(() => {
    providerStore.load();
  });
</script>

<main class="app">
  <div class="shell">
    <aside class="sidebar card">
      <div class="sidebar-header">
        <h1>API Key Storage</h1>
        <div class="subtitle">Modern Bento • Dark Minimal</div>
      </div>
      <ProviderList />
    </aside>

    <section class="content card">
      <div class="content-inner scroll-area">
        {#if $creating}
          <ProviderForm mode="create" />
        {:else}
          <ProviderDetail />
        {/if}
      </div>
    </section>
  </div>

  <Toast />
</main>

<style>
  .app {
    height: 100vh;
    padding: 20px;
  }

  .shell {
    height: 100%;
    display: grid;
    grid-template-columns: 300px 1fr;
    gap: 16px;
    width: min(1200px, 100%);
    margin: 0 auto;
  }

  .sidebar {
    padding: 20px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .sidebar-header {
    margin-bottom: 18px;
  }

  .sidebar-header h1 {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.03em;
    text-transform: uppercase;
    color: var(--muted);
  }

  .subtitle {
    margin-top: 6px;
    font-size: 11px;
    color: var(--muted-2);
  }

  .content {
    padding: 0;
    overflow: hidden;
  }

  .content-inner {
    height: 100%;
    width: 100%;
  }

  @media (max-width: 760px) {
    .app {
      padding: 12px;
    }

    .shell {
      grid-template-columns: 1fr;
    }

    .content-inner {
      width: 100%;
    }
  }
</style>
