<script lang="ts">
  import { toasts } from '../stores/toastStore';
</script>

<div class="toast-container">
  {#each $toasts as toast (toast.id)}
    <div
      class="toast"
      class:success={toast.type === 'success'}
      class:error={toast.type === 'error'}
    >
      {toast.message}
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    gap: 8px;
    z-index: 1100;
    pointer-events: none;
  }

  .toast {
    pointer-events: auto;
    padding: 12px 18px;
    border-radius: 12px;
    font-size: 13px;
    font-weight: 500;
    border: 1px solid var(--border);
    background: rgba(18, 18, 18, 0.85);
    color: rgba(255, 255, 255, 0.9);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    animation: in 0.2s ease;
    width: min(400px, calc(100vw - 32px));
  }

  .toast.success {
    border-color: rgba(52, 211, 153, 0.3);
    background: linear-gradient(
      180deg,
      rgba(52, 211, 153, 0.12),
      rgba(18, 18, 18, 0.85)
    );
  }

  .toast.error {
    border-color: rgba(255, 77, 77, 0.3);
    background: linear-gradient(
      180deg,
      rgba(255, 77, 77, 0.12),
      rgba(18, 18, 18, 0.85)
    );
  }

  @keyframes in {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
