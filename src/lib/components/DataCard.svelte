<script lang="ts">
  interface Props {
    title: string;
    value: string | number;
    unit?: string;
    subtitle?: string;
    color?: string;
    icon?: string;
  }

  let {
    title,
    value,
    unit = '',
    subtitle = '',
    color = 'var(--text-primary)',
    icon = ''
  }: Props = $props();
</script>

<div class="data-card card">
  <div class="card-title">
    {#if icon}
      <span class="icon">{icon}</span>
    {/if}
    {title}
  </div>
  <div class="data-value" style="color: {color};">
    <span class="value-large">{value}</span>
    {#if unit}
      <span class="unit">{unit}</span>
    {/if}
  </div>
  {#if subtitle}
    <div class="data-subtitle">{subtitle}</div>
  {/if}
</div>

<style>
  .data-card {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .data-value {
    display: flex;
    align-items: baseline;
  }

  .data-subtitle {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .icon {
    margin-right: 0.5rem;
  }
</style>
