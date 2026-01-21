<script lang="ts">
  interface Props {
    value: number;
    max?: number;
    size?: number;
    strokeWidth?: number;
    color?: string;
    label?: string;
    unit?: string;
  }

  let {
    value,
    max = 100,
    size = 150,
    strokeWidth = 12,
    color = 'var(--accent-green)',
    label = '',
    unit = '%'
  }: Props = $props();

  const radius = $derived((size - strokeWidth) / 2);
  const circumference = $derived(2 * Math.PI * radius);
  const progress = $derived(Math.min(Math.max(value / max, 0), 1));
  const offset = $derived(circumference - progress * circumference);
</script>

<div class="gauge" style="width: {size}px; height: {size}px;">
  <svg width={size} height={size}>
    <circle
      class="gauge-circle"
      cx={size / 2}
      cy={size / 2}
      r={radius}
      stroke-width={strokeWidth}
    />
    <circle
      class="gauge-progress"
      cx={size / 2}
      cy={size / 2}
      r={radius}
      stroke-width={strokeWidth}
      stroke={color}
      stroke-dasharray={circumference}
      stroke-dashoffset={offset}
    />
  </svg>
  <div class="gauge-text">
    <div class="value-large" style="color: {color};">
      {value.toFixed(0)}
    </div>
    <div class="unit">{unit}</div>
    {#if label}
      <div class="label">{label}</div>
    {/if}
  </div>
</div>
