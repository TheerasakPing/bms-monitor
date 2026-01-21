<script lang="ts">
  import { onMount } from 'svelte';
  import GaugeChart from './lib/components/GaugeChart.svelte';
  import DataCard from './lib/components/DataCard.svelte';
  import AlarmList from './lib/components/AlarmList.svelte';
  import StatusPanel from './lib/components/StatusPanel.svelte';
  import ProgressBar from './lib/components/ProgressBar.svelte';
  import DebugLog from './lib/components/DebugLog.svelte';
  import {
    getConnected,
    getSelectedPort,
    getAvailablePorts,
    getAdapterType,
    getConnectionError,
    getBmsData,
    setSelectedPort,
    setAdapterType,
    listPorts,
    connect,
    disconnect,
  } from './lib/stores/bmsStore.svelte';

  let showConnectionPanel = $state(true);
  let currentPage = $state<'dashboard' | 'debug'>('dashboard');

  onMount(() => {
    listPorts();
  });

  async function handleConnect() {
    const success = await connect();
    if (success) {
      showConnectionPanel = false;
    }
  }

  async function handleDisconnect() {
    await disconnect();
    showConnectionPanel = true;
  }

  function formatTime(minutes: number): string {
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    if (hours > 0) {
      return `${hours}h ${mins}m`;
    }
    return `${mins}m`;
  }

  function formatPower(power: number, current: number): string {
    const absValue = Math.abs(power);
    if (current < 0) {
      return `+${absValue.toFixed(1)}`;
    }
    return `-${absValue.toFixed(1)}`;
  }

  // Reactive getters
  const connected = $derived(getConnected());
  const selectedPort = $derived(getSelectedPort());
  const availablePorts = $derived(getAvailablePorts());
  const adapterType = $derived(getAdapterType());
  const connectionError = $derived(getConnectionError());
  const bmsData = $derived(getBmsData());
</script>

<div class="app">
  <!-- Header -->
  <header class="header">
    <div class="header-left">
      <div class="header-title">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="2" y="7" width="20" height="10" rx="2" />
          <rect x="22" y="10" width="2" height="4" />
          <rect x="5" y="10" width="3" height="4" fill="currentColor" />
          <rect x="10" y="10" width="3" height="4" fill="currentColor" />
          <rect x="15" y="10" width="3" height="4" fill="currentColor" />
        </svg>
        BMS Battery Monitor
      </div>

      <!-- Navigation Tabs -->
      <nav class="nav-tabs">
        <button
          class="nav-tab"
          class:active={currentPage === 'dashboard'}
          onclick={() => currentPage = 'dashboard'}
        >
          Dashboard
        </button>
        <button
          class="nav-tab"
          class:active={currentPage === 'debug'}
          onclick={() => currentPage = 'debug'}
        >
          Debug Log
        </button>
      </nav>
    </div>

    <div class="connection-status">
      <div class="status-dot" class:connected class:disconnected={!connected}></div>
      <span>{connected ? 'Connected' : 'Disconnected'}</span>

      {#if connected}
        <button class="btn btn-danger" onclick={handleDisconnect}>
          Disconnect
        </button>
      {:else}
        <button class="btn btn-secondary" onclick={() => showConnectionPanel = !showConnectionPanel}>
          {showConnectionPanel ? 'Hide' : 'Connect'}
        </button>
      {/if}
    </div>
  </header>

  <!-- Connection Panel -->
  {#if showConnectionPanel && !connected}
    <div class="connection-panel">
      <div class="connection-form">
        <div class="form-group">
          <label>Adapter Type</label>
          <select
            class="select"
            value={adapterType}
            onchange={(e) => setAdapterType((e.target as HTMLSelectElement).value as 'usb' | 'bluetooth' | 'simulation')}
          >
            <option value="simulation">Simulation (Demo)</option>
            <option value="usb">USB-CAN (I+ Series)</option>
            <option value="bluetooth">Bluetooth CAN (I+BT)</option>
          </select>
        </div>

        {#if adapterType !== 'simulation'}
          <div class="form-group">
            <label>Serial Port</label>
            <div class="port-select">
              <select
                class="select"
                value={selectedPort}
                onchange={(e) => setSelectedPort((e.target as HTMLSelectElement).value)}
              >
                <option value="">Select port...</option>
                {#each availablePorts as port}
                  <option value={port}>{port}</option>
                {/each}
              </select>
              <button class="btn btn-secondary" onclick={listPorts}>
                Refresh
              </button>
            </div>
          </div>
        {/if}

        {#if connectionError}
          <div class="error-message">{connectionError}</div>
        {/if}

        <button class="btn btn-primary" onclick={handleConnect}>
          Connect
        </button>
      </div>
    </div>
  {/if}

  <!-- Main Content -->
  <main class="main-content">
    {#if currentPage === 'dashboard'}
    <!-- Dashboard View -->
    <div class="dashboard-grid">
      <!-- SOC Gauge -->
      <div class="col-span-2 card flex flex-col items-center">
        <div class="card-title">State of Charge</div>
        <GaugeChart
          value={bmsData.socSoh?.soc ?? 0}
          max={100}
          color={bmsData.socSoh?.soc && bmsData.socSoh.soc > 20 ? 'var(--accent-green)' : 'var(--accent-red)'}
          unit="%"
          label="SOC"
        />
        <div class="mt-2 label">
          Backup: {bmsData.socSoh ? formatTime(bmsData.socSoh.backupTimeMinutes) : '--'}
        </div>
      </div>

      <!-- SOH Gauge -->
      <div class="col-span-2 card flex flex-col items-center">
        <div class="card-title">State of Health</div>
        <GaugeChart
          value={bmsData.socSoh?.soh ?? 0}
          max={100}
          color={bmsData.socSoh?.soh && bmsData.socSoh.soh > 80 ? 'var(--accent-blue)' : 'var(--accent-yellow)'}
          unit="%"
          label="SOH"
        />
      </div>

      <!-- Voltage/Current/Power -->
      <div class="col-span-4 card">
        <div class="card-title">Battery Output</div>
        <div class="grid grid-3 gap-4">
          <div class="stat-item">
            <div class="value-large" style="color: var(--accent-blue);">
              {bmsData.voltageCurrent?.voltage.toFixed(1) ?? '--'}
            </div>
            <div class="unit">V</div>
            <div class="label">Voltage</div>
          </div>
          <div class="stat-item">
            <div class="value-large" style="color: {bmsData.voltageCurrent?.current && bmsData.voltageCurrent.current < 0 ? 'var(--accent-green)' : 'var(--accent-purple)'}">
              {bmsData.voltageCurrent?.current.toFixed(1) ?? '--'}
            </div>
            <div class="unit">A</div>
            <div class="label">{bmsData.voltageCurrent?.current && bmsData.voltageCurrent.current < 0 ? 'Charging' : 'Discharging'}</div>
          </div>
          <div class="stat-item">
            <div class="value-large" style="color: var(--accent-orange);">
              {bmsData.voltageCurrent ? formatPower(bmsData.voltageCurrent.power, bmsData.voltageCurrent.current) : '--'}
            </div>
            <div class="unit">kW</div>
            <div class="label">Power</div>
          </div>
        </div>
      </div>

      <!-- Limits -->
      <div class="col-span-4 card">
        <div class="card-title">Charge/Discharge Limits</div>
        <div class="limits-grid">
          <div class="limit-row">
            <span class="limit-label">Charge Voltage</span>
            <span class="limit-value">{bmsData.limits?.chargeVoltageLimit.toFixed(1) ?? '--'} V</span>
          </div>
          <div class="limit-row">
            <span class="limit-label">Charge Current</span>
            <span class="limit-value">{bmsData.limits?.chargeCurrentLimit.toFixed(1) ?? '--'} A</span>
          </div>
          <div class="limit-row">
            <span class="limit-label">Discharge Voltage</span>
            <span class="limit-value">{bmsData.limits?.dischargeVoltageLimit.toFixed(1) ?? '--'} V</span>
          </div>
          <div class="limit-row">
            <span class="limit-label">Discharge Current</span>
            <span class="limit-value">{bmsData.limits?.dischargeCurrentLimit.toFixed(1) ?? '--'} A</span>
          </div>
        </div>
      </div>

      <!-- Cell Voltage -->
      <div class="col-span-4 card">
        <div class="card-title">Cell Voltage</div>
        <div class="cell-info">
          <div class="cell-row">
            <span class="cell-label">Maximum</span>
            <span class="cell-value" style="color: var(--accent-green);">
              {bmsData.cellVoltage?.maxVoltage.toFixed(3) ?? '--'} V
            </span>
            <span class="cell-location">
              Pack {bmsData.cellVoltage?.maxVoltagePackNo ?? '-'} / Cell {bmsData.cellVoltage?.maxVoltageCellNo ?? '-'}
            </span>
          </div>
          <div class="cell-row">
            <span class="cell-label">Minimum</span>
            <span class="cell-value" style="color: var(--accent-yellow);">
              {bmsData.cellVoltage?.minVoltage.toFixed(3) ?? '--'} V
            </span>
            <span class="cell-location">
              Pack {bmsData.cellVoltage?.minVoltagePackNo ?? '-'} / Cell {bmsData.cellVoltage?.minVoltageCellNo ?? '-'}
            </span>
          </div>
          <div class="cell-row">
            <span class="cell-label">Delta</span>
            <span class="cell-value" style="color: {bmsData.cellVoltage?.voltageDelta && bmsData.cellVoltage.voltageDelta > 0.05 ? 'var(--accent-red)' : 'var(--accent-blue)'}">
              {bmsData.cellVoltage?.voltageDelta ? (bmsData.cellVoltage.voltageDelta * 1000).toFixed(0) : '--'} mV
            </span>
          </div>
        </div>
      </div>

      <!-- Temperature -->
      <div class="col-span-4 card">
        <div class="card-title">Temperature</div>
        <div class="temp-info">
          <div class="temp-row">
            <span class="temp-label">Maximum</span>
            <span class="temp-value" style="color: {bmsData.temperature?.maxTemperature && bmsData.temperature.maxTemperature > 40 ? 'var(--accent-red)' : 'var(--accent-green)'}">
              {bmsData.temperature?.maxTemperature.toFixed(1) ?? '--'} °C
            </span>
            <span class="temp-location">
              Pack {bmsData.temperature?.maxTempPackNo ?? '-'} / Sensor {bmsData.temperature?.maxTempSensorNo ?? '-'}
            </span>
          </div>
          <div class="temp-row">
            <span class="temp-label">Minimum</span>
            <span class="temp-value" style="color: {bmsData.temperature?.minTemperature && bmsData.temperature.minTemperature < 10 ? 'var(--accent-blue)' : 'var(--accent-green)'}">
              {bmsData.temperature?.minTemperature.toFixed(1) ?? '--'} °C
            </span>
            <span class="temp-location">
              Pack {bmsData.temperature?.minTempPackNo ?? '-'} / Sensor {bmsData.temperature?.minTempSensorNo ?? '-'}
            </span>
          </div>
          <div class="temp-row">
            <span class="temp-label">Delta</span>
            <span class="temp-value" style="color: {bmsData.temperature?.tempDelta && bmsData.temperature.tempDelta > 5 ? 'var(--accent-yellow)' : 'var(--accent-blue)'}">
              {bmsData.temperature?.tempDelta.toFixed(1) ?? '--'} °C
            </span>
          </div>
        </div>
      </div>

      <!-- Status Panel -->
      <div class="col-span-4">
        <StatusPanel status={bmsData.operationStatus} />
      </div>

      <!-- Accumulated Data -->
      <div class="col-span-4 card">
        <div class="card-title">Accumulated Data</div>
        <div class="accumulated-grid">
          <div class="accumulated-item">
            <div class="accumulated-label">Charge Cycles</div>
            <div class="accumulated-value">{bmsData.accumulatedTimes?.chargeTimes ?? '--'}</div>
          </div>
          <div class="accumulated-item">
            <div class="accumulated-label">Discharge Cycles</div>
            <div class="accumulated-value">{bmsData.accumulatedTimes?.dischargeTimes ?? '--'}</div>
          </div>
          <div class="accumulated-item">
            <div class="accumulated-label">Charge Energy</div>
            <div class="accumulated-value">{bmsData.accumulatedPower?.chargeEnergy.toFixed(1) ?? '--'} kWh</div>
          </div>
          <div class="accumulated-item">
            <div class="accumulated-label">Discharge Energy</div>
            <div class="accumulated-value">{bmsData.accumulatedPower?.dischargeEnergy.toFixed(1) ?? '--'} kWh</div>
          </div>
        </div>
      </div>

      <!-- Alarms -->
      <div class="col-span-4 card">
        <div class="card-title">
          Alarms
          {#if bmsData.alarmStatus?.activeAlarms && bmsData.alarmStatus.activeAlarms.length > 0}
            <span class="badge badge-red">{bmsData.alarmStatus.activeAlarms.length}</span>
          {/if}
        </div>
        <AlarmList alarms={bmsData.alarmStatus?.activeAlarms ?? []} />
      </div>

      <!-- Software Version -->
      <div class="col-span-4 card">
        <div class="card-title">System Information</div>
        <div class="system-info">
          <div class="info-row">
            <span class="info-label">BMS Version</span>
            <span class="info-value">{bmsData.softwareVersion ?? '--'}</span>
          </div>
          <div class="info-row">
            <span class="info-label">Last Update</span>
            <span class="info-value">{bmsData.connected ? bmsData.timestamp.toLocaleTimeString() : '--'}</span>
          </div>
          <div class="info-row">
            <span class="info-label">Protocol</span>
            <span class="info-value">CAN 2.0B @ 125Kbps</span>
          </div>
        </div>
      </div>
    </div>
    {:else}
    <!-- Debug Log View -->
    <DebugLog />
    {/if}
  </main>
</div>

<style>
  .app {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .connection-panel {
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    padding: 1.5rem 2rem;
  }

  .connection-form {
    display: flex;
    gap: 1.5rem;
    align-items: flex-end;
    flex-wrap: wrap;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
  }

  .port-select {
    display: flex;
    gap: 0.5rem;
  }

  .error-message {
    color: var(--accent-red);
    font-size: 0.875rem;
    padding: 0.5rem 1rem;
    background: rgba(239, 68, 68, 0.1);
    border-radius: 8px;
  }

  .stat-item {
    text-align: center;
  }

  .stat-item .unit {
    display: block;
    margin-top: 0.25rem;
  }

  .limits-grid, .cell-info, .temp-info, .accumulated-grid, .system-info {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .limit-row, .cell-row, .temp-row, .info-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .limit-label, .cell-label, .temp-label, .info-label, .accumulated-label {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .limit-value, .cell-value, .temp-value, .info-value {
    font-weight: 600;
    font-size: 1rem;
  }

  .cell-location, .temp-location {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .accumulated-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }

  .accumulated-item {
    text-align: center;
    padding: 1rem;
    background: var(--bg-card);
    border-radius: 8px;
  }

  .accumulated-value {
    font-size: 1.25rem;
    font-weight: 700;
    margin-top: 0.5rem;
    color: var(--accent-blue);
  }

  .card-title .badge {
    margin-left: 0.5rem;
  }
</style>
