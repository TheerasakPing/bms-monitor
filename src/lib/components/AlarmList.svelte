<script lang="ts">
  import { getAlarmSeverity } from '../stores/bmsStore.svelte';

  interface Alarm {
    bit: number;
    description: string;
    severity: number;
  }

  interface Props {
    alarms: number[];
  }

  let { alarms }: Props = $props();

  const alarmDescriptions: Record<number, string> = {
    0: 'Cell over voltage',
    1: 'Cell under voltage',
    2: 'Charging over temperature alarm',
    3: 'Charging low temperature alarm',
    4: 'Discharging over temperature pre-alarm',
    5: 'Discharging low temperature pre-alarm',
    6: 'Discharging over current pre-alarm',
    7: 'Charging over current pre-alarm',
    8: 'Total over voltage pre-alarm',
    9: 'Total under voltage warning',
    10: 'Circuit breaker disconnected',
    11: 'Balanced charging failed',
    12: 'Positive pack voltage imbalance',
    13: 'Negative pack voltage imbalance',
    14: 'BMU communication interruption',
    15: 'Water flooding detection alarm',
    16: 'Water flooding protection',
    18: 'Charging over temp protection',
    19: 'Charging low temp protection',
    20: 'Discharging over temp protection',
    21: 'Discharging low temp protection',
    22: 'Discharge over current L1',
    23: 'Discharge over current L2',
    24: 'Charging over current L1',
    25: 'Charging over current L2',
    26: 'Charging over current L3',
    27: 'Total charge over voltage',
    28: 'Total charge under voltage',
    29: 'Charging DC contactor failure',
    30: 'Discharging DC contactor failure',
    31: 'EPO shut down',
    32: 'Fire protection',
    33: 'Parallel comm abnormality',
    34: 'Parallel address conflict',
    35: 'Insulation monitoring alarm',
    36: 'Hydrogen protection',
    37: 'Battery pack fan malfunction',
    38: 'Fuse temperature high',
    39: 'CAN Hall comm interruption',
    40: 'CAN Hall data failure',
  };

  const activeAlarms = $derived<Alarm[]>(
    alarms.map(bit => ({
      bit,
      description: alarmDescriptions[bit] || `Unknown (bit ${bit})`,
      severity: getAlarmSeverity(bit),
    })).sort((a, b) => b.severity - a.severity)
  );
</script>

<div class="alarm-list">
  {#if activeAlarms.length === 0}
    <div class="no-alarms">
      <span class="icon">&#10003;</span>
      No active alarms
    </div>
  {:else}
    {#each activeAlarms as alarm}
      <div class="alarm-item">
        <div class="alarm-icon alarm-level-{alarm.severity}">
          {#if alarm.severity === 3}
            &#9888;
          {:else if alarm.severity === 2}
            &#9888;
          {:else}
            &#9432;
          {/if}
        </div>
        <div class="alarm-content">
          <div class="alarm-description">{alarm.description}</div>
          <div class="alarm-level">
            Level {alarm.severity}
            {#if alarm.severity === 3}
              - Severe
            {:else if alarm.severity === 2}
              - Moderate
            {:else}
              - Mild
            {/if}
          </div>
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .alarm-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-height: 300px;
    overflow-y: auto;
  }

  .no-alarms {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 2rem;
    color: var(--accent-green);
    background: rgba(34, 197, 94, 0.1);
    border-radius: 8px;
  }

  .no-alarms .icon {
    font-size: 1.25rem;
  }

  .alarm-content {
    flex: 1;
  }

  .alarm-description {
    font-weight: 500;
    font-size: 0.875rem;
  }

  .alarm-level {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }
</style>
