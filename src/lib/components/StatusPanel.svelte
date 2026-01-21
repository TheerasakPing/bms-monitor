<script lang="ts">
  import { getSystemStatusName, getWorkStatusName, getOperationStatusName } from '../stores/bmsStore.svelte';
  import type { OperationStatusData, SystemStatus, WorkStatus, OperationStatus } from '../types/bms';

  interface Props {
    status: OperationStatusData | null;
  }

  let { status }: Props = $props();

  const systemStatusColor = $derived(() => {
    if (!status) return 'var(--text-secondary)';
    switch (status.systemStatus) {
      case 3: return 'var(--accent-blue)'; // Charging
      case 4: return 'var(--accent-purple)'; // Discharging
      case 7: return 'var(--accent-red)'; // Lock
      default: return 'var(--accent-green)';
    }
  });

  const operationStatusColor = $derived(() => {
    if (!status) return 'var(--text-secondary)';
    switch (status.operationStatus) {
      case 1: return 'var(--accent-green)'; // Normal
      case 2: return 'var(--accent-yellow)'; // Alarm
      case 3: return 'var(--accent-red)'; // Fault
      default: return 'var(--text-secondary)';
    }
  });

  const badgeClass = $derived(() => {
    if (!status) return 'badge-gray';
    switch (status.operationStatus) {
      case 1: return 'badge-green';
      case 2: return 'badge-yellow';
      case 3: return 'badge-red';
      default: return 'badge-gray';
    }
  });
</script>

<div class="status-panel">
  <div class="status-row">
    <div class="status-item">
      <div class="status-label">System Status</div>
      <div class="status-value" style="color: {systemStatusColor()}">
        {status ? getSystemStatusName(status.systemStatus as number) : '--'}
      </div>
    </div>

    <div class="status-item">
      <div class="status-label">Work Status</div>
      <div class="status-value">
        {status ? getWorkStatusName(status.workStatus as number) : '--'}
      </div>
    </div>

    <div class="status-item">
      <div class="status-label">Operation</div>
      <span class="badge {badgeClass()}">
        {status ? getOperationStatusName(status.operationStatus as number) : '--'}
      </span>
    </div>
  </div>

  <div class="divider"></div>

  <div class="prohibition-flags">
    <div class="flag-item" class:active={status?.chargeProhibited}>
      <span class="flag-icon">&#9889;</span>
      <span>Charge</span>
      <span class="flag-status">
        {status?.chargeProhibited ? 'Prohibited' : 'Allowed'}
      </span>
    </div>

    <div class="flag-item" class:active={status?.dischargeProhibited || status?.dischargeProhibitedHard}>
      <span class="flag-icon">&#9889;</span>
      <span>Discharge</span>
      <span class="flag-status">
        {#if status?.dischargeProhibitedHard}
          Prohibited (Hard)
        {:else if status?.dischargeProhibited}
          Prohibited
        {:else}
          Allowed
        {/if}
      </span>
    </div>
  </div>
</div>

<style>
  .status-panel {
    background: var(--bg-secondary);
    border-radius: 12px;
    padding: 1.5rem;
    border: 1px solid var(--border-color);
  }

  .status-row {
    display: flex;
    gap: 2rem;
    flex-wrap: wrap;
  }

  .status-item {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .status-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
  }

  .status-value {
    font-size: 1.25rem;
    font-weight: 600;
  }

  .prohibition-flags {
    display: flex;
    gap: 1.5rem;
    flex-wrap: wrap;
  }

  .flag-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: rgba(34, 197, 94, 0.1);
    border-radius: 8px;
    font-size: 0.875rem;
  }

  .flag-item.active {
    background: rgba(239, 68, 68, 0.1);
    color: var(--accent-red);
  }

  .flag-icon {
    font-size: 1rem;
  }

  .flag-status {
    font-weight: 600;
    margin-left: auto;
  }
</style>
