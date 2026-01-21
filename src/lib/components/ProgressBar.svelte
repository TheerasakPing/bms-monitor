<script lang="ts">
  interface Props {
    value: number;
    min?: number;
    max?: number;
    warningLow?: number;
    warningHigh?: number;
    dangerLow?: number;
    dangerHigh?: number;
    unit?: string;
    showValue?: boolean;
  }

  let {
    value,
    min = 0,
    max = 100,
    warningLow = 20,
    warningHigh = 80,
    dangerLow = 10,
    dangerHigh = 90,
    unit = '',
    showValue = true
  }: Props = $props();

  const percentage = $derived(Math.min(Math.max((value - min) / (max - min) * 100, 0), 100));

  const barColor = $derived(() => {
    if (value <= dangerLow || value >= dangerHigh) return 'var(--accent-red)';
    if (value <= warningLow || value >= warningHigh) return 'var(--accent-yellow)';
    return 'var(--accent-green)';
  });
</script>

<div class="progress-container">
  <div class="progress-bar">
    <div
      class="progress-fill"
      style="width: {percentage}%; background: {barColor()}"
    ></div>
  </div>
  {#if showValue}
    <div class="progress-value" style="color: {barColor()}">
      {value.toFixed(1)}{unit}
    </div>
  {/if}
</div>

<style>
  .progress-container {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .progress-bar {
    flex: 1;
  }

  .progress-value {
    min-width: 80px;
    text-align: right;
    font-weight: 600;
    font-size: 0.875rem;
  }
</style>
