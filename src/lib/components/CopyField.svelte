<script lang="ts">
  import { providerStore } from '../stores/providerStore';
  import { toastStore } from '../stores/toastStore';

  interface Props {
    label: string;
    value: string;
    copyable?: boolean;
    masked?: boolean;
  }

  let { label, value, copyable = true, masked = false }: Props = $props();

  let showValue = $state(!masked);

  async function handleCopy() {
    if (!value) return;
    try {
      await providerStore.copyToClipboard(value);
      toastStore.success('已复制 ' + label);
    } catch {
      toastStore.error('复制失败');
    }
  }

  function toggleVisibility() {
    showValue = !showValue;
  }
</script>

<div class="copy-field">
  <div class="label-row">
    <div class="label">{label}</div>
    <div class="actions">
      {#if masked}
        <button
          class="chip"
          type="button"
          on:click={toggleVisibility}
          title={showValue ? '隐藏' : '显示'}
        >
          {showValue ? '隐藏' : '显示'}
        </button>
      {/if}
      {#if copyable}
        <button class="chip" type="button" on:click={handleCopy} title="复制">复制</button>
      {/if}
    </div>
  </div>

  <div class="value">
    {#if !value}
      <span class="empty">未设置</span>
    {:else if masked && !showValue}
      {'*'.repeat(Math.min(value.length, 24))}
    {:else}
      {value}
    {/if}
  </div>
</div>

<style>
  .copy-field {
    border: 1px solid var(--border);
    border-radius: 14px;
    padding: 14px 16px;
    background: rgba(0, 0, 0, 0.2);
  }

  .label-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 10px;
  }

  .label {
    font-size: 11px;
    color: var(--muted);
    font-weight: 500;
    letter-spacing: 0.03em;
    text-transform: uppercase;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .actions {
    display: flex;
    gap: 6px;
    flex-wrap: nowrap;
    flex-shrink: 0;
  }

  .value {
    font-family: var(--mono);
    font-size: 13px;
    color: rgba(255, 255, 255, 0.88);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    line-height: 1.5;
  }

  .empty {
    color: var(--muted-2);
    font-style: italic;
  }
</style>
