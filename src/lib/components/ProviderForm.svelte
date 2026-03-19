<script lang="ts">
  import { providerStore } from '../stores/providerStore';
  import { toastStore } from '../stores/toastStore';

  interface ProviderLike {
    id: string;
    name: string;
    base_url: string;
    remark?: string;
  }

  interface Props {
    provider?: ProviderLike;
    mode: 'create' | 'edit';
  }

  let { provider = undefined, mode }: Props = $props();

  let inputName = $state('');
  let inputBaseUrl = $state('');
  let inputApiKey = $state('');
  let inputRemark = $state('');
  let showApiKey = $state(false);
  let errors = $state({} as Record<string, string>);

  // runes mode: 用 $effect 替代 $:
  $effect(() => {
    if (mode === 'edit' && provider) {
      inputName = provider.name;
      inputBaseUrl = provider.base_url;
      inputRemark = provider.remark ?? '';
    }
  });

  function validate(): boolean {
    errors = {};

    if (!inputName.trim()) {
      errors.name = '平台名称不能为空';
    }

    if (!inputBaseUrl.trim()) {
      errors.baseUrl = 'Base URL 不能为空';
    } else {
      try {
        const url = new URL(inputBaseUrl.trim());
        if (!['http:', 'https:'].includes(url.protocol)) {
          errors.baseUrl = 'Base URL 必须以 http:// 或 https:// 开头';
        }
      } catch {
        errors.baseUrl = 'Base URL 格式无效';
      }
    }

    if (mode === 'create' && !inputApiKey.trim()) {
      errors.apiKey = 'API Key 不能为空';
    }

    return Object.keys(errors).length === 0;
  }

  async function handleSubmit() {
    if (!validate()) return;

    try {
      const payload = {
        name: inputName.trim(),
        base_url: inputBaseUrl.trim(),
        api_key: inputApiKey.trim(),
        remark: inputRemark.trim() || undefined
      };

      if (mode === 'create') {
        await providerStore.create(payload);
        toastStore.success('创建成功');
        inputName = '';
        inputBaseUrl = '';
        inputApiKey = '';
        inputRemark = '';
        showApiKey = false;
      } else if (provider) {
        await providerStore.update({ id: provider.id, ...payload });
        toastStore.success('保存成功');
      }
    } catch (e) {
      toastStore.error('操作失败: ' + e);
    }
  }

  function handleCancel() {
    if (mode === 'create') {
      providerStore.cancelCreate();
      return;
    }

    providerStore.cancelEdit();
  }
</script>

<form class="form card" autocomplete="off" on:submit|preventDefault={handleSubmit}>
  <div class="header">
    <h3>{mode === 'create' ? '新建平台' : '编辑平台'}</h3>
    <div class="muted">字段会安全存储在本地</div>
  </div>

  <div class="field">
    <label>平台名称 *</label>
    <input
      class="input"
      type="text"
      name="provider-name"
      bind:value={inputName}
      placeholder="如: OpenAI"
      autocomplete="off"
    />
    {#if errors.name}
      <span class="error">{errors.name}</span>
    {/if}
  </div>

  <div class="field">
    <label>Base URL *</label>
    <input
      class="input"
      type="url"
      name="provider-base-url"
      bind:value={inputBaseUrl}
      placeholder="https://api.openai.com/v1"
      autocomplete="url"
      spellcheck="false"
    />
    {#if errors.baseUrl}
      <span class="error">{errors.baseUrl}</span>
    {/if}
  </div>

  <div class="field">
    <label>API Key {mode === 'create' ? '*' : '(留空则不修改)'}</label>
    <div class="input-row">
      <input
        class="input"
        type={showApiKey ? 'text' : 'password'}
        name="provider-api-key"
        bind:value={inputApiKey}
        placeholder="sk-..."
        autocomplete="new-password"
        spellcheck="false"
      />
      <button type="button" class="btn btn-ghost" on:click={() => (showApiKey = !showApiKey)}>
        {showApiKey ? '隐藏' : '显示'}
      </button>
    </div>
    {#if errors.apiKey}
      <span class="error">{errors.apiKey}</span>
    {/if}
  </div>

  <div class="field">
    <label>备注</label>
    <textarea
      class="textarea"
      name="provider-remark"
      bind:value={inputRemark}
      placeholder="可选备注"
      rows="3"
    ></textarea>
  </div>

  <div class="actions">
    <button type="button" class="btn" on:click={handleCancel}>取消</button>
    <button type="submit" class="btn btn-primary">{mode === 'create' ? '创建' : '保存'}</button>
  </div>
</form>

<style>
  .form {
    max-width: 480px;
    margin: 0 auto;
    padding: 24px;
  }

  .header {
    display: grid;
    gap: 8px;
    margin-bottom: 24px;
  }

  h3 {
    margin: 0;
    font-size: 20px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.95);
  }

  .field {
    margin-bottom: 16px;
  }

  label {
    display: block;
    font-size: 11px;
    color: var(--muted);
    margin-bottom: 8px;
    font-weight: 500;
    letter-spacing: 0.03em;
    text-transform: uppercase;
  }

  .input-row {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 8px;
    align-items: center;
  }

  .input-row .btn {
    height: 40px;
    padding: 0 14px;
    border-radius: 10px;
  }

  .error {
    display: block;
    color: rgba(255, 77, 77, 0.9);
    font-size: 12px;
    margin-top: 8px;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 24px;
  }
</style>
