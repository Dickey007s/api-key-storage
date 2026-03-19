<script lang="ts">
  import { providerStore, searchQuery } from '../stores/providerStore';
</script>

<div class="search-box">
  <input
    class="input"
    type="text"
    placeholder="搜索平台..."
    value={$searchQuery}
    on:input={(e) => providerStore.setSearchQuery((e.currentTarget as HTMLInputElement).value)}
  />
</div>

<style>
  .search-box {
    margin-bottom: 0;
  }
</style>
