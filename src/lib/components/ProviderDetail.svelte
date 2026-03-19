<script lang="ts">
  import { providerStore, editing, selectedProvider } from '../stores/providerStore';
  import { toastStore } from '../stores/toastStore';
  import ProviderForm from './ProviderForm.svelte';

  let apiKey = $state('');
  let showConfirm = $state(false);
  let showApiKey = $state(false);

  async function loadApiKey() {
    if ($selectedProvider) {
      try {
        apiKey = await providerStore.getApiKey($selectedProvider.id);
      } catch (e) {
        console.error('加载 API Key 失败:', e);
        toastStore.error('获取密钥失败');
        apiKey = '';
      }
    } else {
      apiKey = '';
    }
  }

  $effect(() => {
    if ($selectedProvider && !$editing) {
      loadApiKey();
    } else {
      apiKey = '';
    }
  });

  function handleEdit() {
    providerStore.startEdit();
  }

  function handleDelete() {
    showConfirm = true;
  }

  async function confirmDelete() {
    if ($selectedProvider) {
      try {
        await providerStore.delete($selectedProvider.id);
        toastStore.success('删除成功');
      } catch {
        toastStore.error('删除失败');
      }
    }
    showConfirm = false;
  }

  async function copyText(text: string, label: string) {
    await providerStore.copyToClipboard(text);
    toastStore.success('已复制 ' + label);
  }
</script>

{#if $editing}
  <ProviderForm provider={$selectedProvider ?? undefined} mode="edit" />
{:else if $selectedProvider}
  <div class="detail">
    <!-- 主卡片 -->
    <div class="main-card card">
      <!-- 头部区域 -->
      <header class="header">
        <div class="title-group">
          <h1>{$selectedProvider.name}</h1>
          <span class="domain">{$selectedProvider.base_url}</span>
        </div>
        <button class="btn btn-secondary" on:click={handleEdit}>编辑</button>
      </header>

      <!-- 凭证区域 -->
      <section class="credential-section">
        <h2 class="section-label">凭证</h2>
        <div class="credential-grid">
          <div class="credential-row">
            <div class="credential-info">
              <span class="credential-name">Base URL</span>
              <code class="credential-value">{$selectedProvider.base_url}</code>
            </div>
            <button class="btn-small" on:click={() => copyText($selectedProvider.base_url, 'URL')}>
              复制
            </button>
          </div>
          <div class="credential-row">
            <div class="credential-info">
              <span class="credential-name">API Key</span>
              <code class="credential-value">
                {#if showApiKey}
                  {apiKey || '未设置'}
                {:else}
                  {#if apiKey}
                    {'•'.repeat(Math.min(apiKey.length, 40))}
                  {:else}
                    未设置
                  {/if}
                {/if}
              </code>
            </div>
            <div class="credential-actions">
              {#if apiKey}
                <button class="btn-small" on:click={() => (showApiKey = !showApiKey)}>
                  {showApiKey ? '隐藏' : '显示'}
                </button>
                <button class="btn-small" on:click={() => copyText(apiKey, 'Key')}>
                  复制
                </button>
              {/if}
            </div>
          </div>
        </div>
      </section>

      <!-- 备注区域 -->
      <section class="remark-section">
        <h2 class="section-label">备注</h2>
        <div class="remark-box">
          {#if $selectedProvider.remark}
            {$selectedProvider.remark}
          {:else}
            <span class="placeholder">暂无备注</span>
          {/if}
        </div>
      </section>

      <!-- 底部元信息 -->
      <footer class="meta-footer">
        <span class="meta-item">
          <span class="meta-label">创建</span>
          <span class="meta-value">{new Date($selectedProvider.created_at).toLocaleDateString('zh-CN')}</span>
        </span>
        <span class="meta-divider"></span>
        <span class="meta-item">
          <span class="meta-label">更新</span>
          <span class="meta-value">{new Date($selectedProvider.updated_at).toLocaleDateString('zh-CN')}</span>
        </span>
      </footer>
    </div>

    <!-- 危险操作区 -->
    <div class="danger-card card">
      <div class="danger-content">
        <div class="danger-info">
          <span class="danger-label">危险操作</span>
          <span class="danger-desc">删除后数据无法恢复</span>
        </div>
        <button class="btn btn-danger" on:click={handleDelete}>删除平台</button>
      </div>
    </div>

    <!-- 删除确认弹窗 -->
    {#if showConfirm}
      <div class="modal-overlay" on:click={() => (showConfirm = false)}>
        <div class="modal-card card" on:click|stopPropagation>
          <h3>确认删除</h3>
          <p>删除「{$selectedProvider.name}」后无法恢复，确定继续？</p>
          <div class="modal-actions">
            <button class="btn" on:click={() => (showConfirm = false)}>取消</button>
            <button class="btn btn-danger" on:click={confirmDelete}>确认删除</button>
          </div>
        </div>
      </div>
    {/if}
  </div>
{:else}
  <div class="empty-state">
    <div class="empty-content">
      <div class="empty-icon">📋</div>
      <h3>选择一个平台</h3>
      <p>或点击左侧「新建平台」开始使用</p>
    </div>
  </div>
{/if}

<style>
  /* ===== 布局容器 ===== */
  .detail {
    height: 100%;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    overflow: auto;
  }

  .main-card {
    padding: 32px;
    flex: 1;
    min-height: 0;
  }

  /* ===== 头部区域 ===== */
  .header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 32px;
  }

  .title-group {
    flex: 1;
    min-width: 0;
  }

  h1 {
    margin: 0;
    font-size: 28px;
    font-weight: 700;
    color: #fff;
    letter-spacing: -0.02em;
    line-height: 1.2;
  }

  .domain {
    display: block;
    margin-top: 8px;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.5);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* ===== 通用标签 ===== */
  .section-label {
    margin: 0 0 16px;
    font-size: 12px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.4);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  /* ===== 凭证区域 ===== */
  .credential-section {
    margin-bottom: 32px;
  }

  .credential-grid {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .credential-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 16px 20px;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
  }

  .credential-info {
    flex: 1;
    min-width: 0;
  }

  .credential-name {
    display: block;
    font-size: 11px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.4);
    margin-bottom: 6px;
    letter-spacing: 0.05em;
  }

  .credential-value {
    display: block;
    font-family: ui-monospace, monospace;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.85);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .credential-actions {
    display: flex;
    gap: 8px;
    flex-shrink: 0;
  }

  .btn-small {
    padding: 6px 12px;
    font-size: 12px;
    font-weight: 500;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.05);
    color: rgba(255, 255, 255, 0.7);
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.15s ease;
  }

  .btn-small:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.15);
    color: #fff;
  }

  /* ===== 备注区域 ===== */
  .remark-section {
    margin-bottom: 32px;
  }

  .remark-box {
    padding: 16px 20px;
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 12px;
    font-size: 14px;
    line-height: 1.7;
    color: rgba(255, 255, 255, 0.8);
    white-space: pre-wrap;
    min-height: 60px;
  }

  .placeholder {
    color: rgba(255, 255, 255, 0.3);
    font-style: italic;
  }

  /* ===== 底部元信息 ===== */
  .meta-footer {
    display: flex;
    align-items: center;
    gap: 16px;
    padding-top: 24px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
  }

  .meta-item {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .meta-label {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.4);
  }

  .meta-value {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.6);
    font-variant-numeric: tabular-nums;
  }

  .meta-divider {
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.15);
  }

  /* ===== 危险操作区 ===== */
  .danger-card {
    padding: 20px 24px;
    border-color: rgba(255, 77, 77, 0.2);
    background: rgba(255, 77, 77, 0.05);
    flex-shrink: 0;
  }

  .danger-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .danger-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .danger-label {
    font-size: 13px;
    font-weight: 600;
    color: rgba(255, 77, 77, 0.9);
  }

  .danger-desc {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.4);
  }

  /* ===== 按钮样式 ===== */
  .btn {
    padding: 10px 18px;
    font-size: 14px;
    font-weight: 500;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.05);
    color: rgba(255, 255, 255, 0.85);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.15);
  }

  .btn-secondary {
    background: rgba(109, 124, 255, 0.15);
    border-color: rgba(109, 124, 255, 0.3);
    color: rgba(109, 124, 255, 1);
  }

  .btn-secondary:hover {
    background: rgba(109, 124, 255, 0.25);
    border-color: rgba(109, 124, 255, 0.4);
  }

  .btn-danger {
    background: rgba(255, 77, 77, 0.15);
    border-color: rgba(255, 77, 77, 0.3);
    color: rgba(255, 77, 77, 1);
  }

  .btn-danger:hover {
    background: rgba(255, 77, 77, 0.25);
    border-color: rgba(255, 77, 77, 0.4);
  }

  /* ===== 弹窗 ===== */
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 24px;
  }

  .modal-card {
    width: min(400px, 100%);
    padding: 28px;
  }

  .modal-card h3 {
    margin: 0 0 12px;
    font-size: 18px;
    font-weight: 600;
    color: #fff;
  }

  .modal-card p {
    margin: 0 0 24px;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.6);
    line-height: 1.5;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
  }

  /* ===== 空状态 ===== */
  .empty-state {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 32px;
  }

  .empty-content {
    text-align: center;
  }

  .empty-icon {
    font-size: 48px;
    margin-bottom: 16px;
    opacity: 0.5;
  }

  .empty-content h3 {
    margin: 0 0 8px;
    font-size: 18px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.85);
  }

  .empty-content p {
    margin: 0;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.4);
  }

  /* ===== 响应式 ===== */
  @media (max-width: 760px) {
    .main-card {
      padding: 24px;
    }

    h1 {
      font-size: 22px;
    }

    .danger-content {
      flex-direction: column;
      align-items: flex-start;
    }
  }
</style>