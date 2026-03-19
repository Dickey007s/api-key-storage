<script lang="ts">
  import { providerStore } from '../stores/providerStore';
</script>

<button class="create-btn btn btn-primary" type="button" on:click={() => providerStore.startCreate()}>
  + 新建平台
</button>

<style>
  .create-btn {
    width: 100%;
    border-radius: 12px;
    font-weight: 500;
  }
</style>
