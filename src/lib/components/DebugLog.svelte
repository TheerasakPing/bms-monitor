<script lang="ts">
  import {
    getLogs,
    getIsRecording,
    startRecording,
    stopRecording,
    clearLogs,
    downloadLogs,
    generateTestData,
  } from '../stores/debugStore.svelte';

  let autoScroll = $state(true);
  let filterCommand = $state('all');
  let filterDirection = $state<'all' | 'TX' | 'RX'>('all');
  let logContainer: HTMLDivElement;

  const logs = $derived(getLogs());
  const isRecording = $derived(getIsRecording());

  // Filter logs
  const filteredLogs = $derived(() => {
    return logs.filter(log => {
      if (filterDirection !== 'all' && log.direction !== filterDirection) return false;
      if (filterCommand !== 'all' && log.commandCode !== parseInt(filterCommand, 16)) return false;
      return true;
    });
  });

  // Auto-scroll effect
  $effect(() => {
    if (autoScroll && logContainer && logs.length > 0) {
      logContainer.scrollTop = logContainer.scrollHeight;
    }
  });

  function handleStartStop() {
    if (isRecording) {
      stopRecording();
    } else {
      startRecording();
    }
  }

  function handleDownload(format: 'csv' | 'json') {
    downloadLogs(format);
  }

  function getDirectionClass(direction: 'TX' | 'RX'): string {
    return direction === 'TX' ? 'direction-tx' : 'direction-rx';
  }

  function getCommandClass(code: number): string {
    if (code >= 0x80 && code <= 0x8F) return 'cmd-data';
    if (code === 0xC0) return 'cmd-alarm';
    if (code === 0xD0) return 'cmd-debug';
    return 'cmd-control';
  }
</script>

<div class="debug-log">
  <!-- Toolbar -->
  <div class="toolbar">
    <div class="toolbar-left">
      <button
        class="btn {isRecording ? 'btn-danger' : 'btn-primary'}"
        onclick={handleStartStop}
      >
        {#if isRecording}
          <span class="recording-dot"></span>
          Stop Recording
        {:else}
          Start Recording
        {/if}
      </button>

      <button class="btn btn-secondary" onclick={clearLogs} disabled={isRecording}>
        Clear
      </button>

      <button class="btn btn-secondary" onclick={generateTestData} disabled={!isRecording}>
        Test Data
      </button>

      <div class="log-count">
        {logs.length} entries
      </div>
    </div>

    <div class="toolbar-center">
      <label class="filter-label">
        Direction:
        <select class="select" bind:value={filterDirection}>
          <option value="all">All</option>
          <option value="TX">TX (Send)</option>
          <option value="RX">RX (Receive)</option>
        </select>
      </label>

      <label class="filter-label">
        Command:
        <select class="select" bind:value={filterCommand}>
          <option value="all">All</option>
          <option value="80">0x80 - Limits</option>
          <option value="81">0x81 - SOC/SOH</option>
          <option value="82">0x82 - Voltage/Current</option>
          <option value="83">0x83 - Cell Voltage</option>
          <option value="84">0x84 - Temperature</option>
          <option value="85">0x85 - Operation Status</option>
          <option value="86">0x86 - Acc. Times</option>
          <option value="87">0x87 - Acc. Power</option>
          <option value="8F">0x8F - Version</option>
          <option value="C0">0xC0 - Alarm</option>
        </select>
      </label>
    </div>

    <div class="toolbar-right">
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={autoScroll} />
        Auto-scroll
      </label>

      <button class="btn btn-secondary" onclick={() => handleDownload('csv')}>
        Download CSV
      </button>
      <button class="btn btn-secondary" onclick={() => handleDownload('json')}>
        Download JSON
      </button>
    </div>
  </div>

  <!-- Log Table -->
  <div class="log-container" bind:this={logContainer}>
    <table class="log-table">
      <thead>
        <tr>
          <th class="col-time">Time</th>
          <th class="col-dir">Dir</th>
          <th class="col-frame">Frame ID</th>
          <th class="col-cmd">Command</th>
          <th class="col-addr">Src → Dst</th>
          <th class="col-data">Data (Hex)</th>
          <th class="col-parsed">Parsed Value</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredLogs() as log (log.id)}
          <tr class="log-row">
            <td class="col-time">
              {log.timestamp.toLocaleTimeString()}.{log.timestamp.getMilliseconds().toString().padStart(3, '0')}
            </td>
            <td class="col-dir">
              <span class="direction-badge {getDirectionClass(log.direction)}">
                {log.direction}
              </span>
            </td>
            <td class="col-frame mono">{log.frameIdHex}</td>
            <td class="col-cmd">
              <span class="command-badge {getCommandClass(log.commandCode)}">
                0x{log.commandCode.toString(16).toUpperCase().padStart(2, '0')}
              </span>
              <span class="command-name">{log.command}</span>
            </td>
            <td class="col-addr mono">
              0x{log.source.toString(16).toUpperCase().padStart(2, '0')} → 0x{log.destination.toString(16).toUpperCase().padStart(2, '0')}
            </td>
            <td class="col-data mono">{log.dataHex}</td>
            <td class="col-parsed">{log.parsedValue}</td>
          </tr>
        {:else}
          <tr>
            <td colspan="7" class="empty-message">
              {#if isRecording}
                Waiting for CAN frames...
              {:else}
                Click "Start Recording" to capture CAN communication
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <!-- Status Bar -->
  <div class="status-bar">
    <div class="status-item">
      <span class="status-label">Status:</span>
      <span class="status-value {isRecording ? 'status-recording' : 'status-stopped'}">
        {isRecording ? 'Recording' : 'Stopped'}
      </span>
    </div>
    <div class="status-item">
      <span class="status-label">Filtered:</span>
      <span class="status-value">{filteredLogs().length} / {logs.length}</span>
    </div>
  </div>
</div>

<style>
  .debug-log {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-primary);
  }

  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    flex-wrap: wrap;
    gap: 1rem;
  }

  .toolbar-left, .toolbar-center, .toolbar-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .log-count {
    font-size: 0.875rem;
    color: var(--text-secondary);
    padding: 0.25rem 0.75rem;
    background: var(--bg-card);
    border-radius: 4px;
  }

  .filter-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    cursor: pointer;
  }

  .recording-dot {
    display: inline-block;
    width: 8px;
    height: 8px;
    background: white;
    border-radius: 50%;
    margin-right: 0.5rem;
    animation: pulse 1s infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }

  .log-container {
    flex: 1;
    overflow: auto;
    background: var(--bg-card);
  }

  .log-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.8125rem;
  }

  .log-table thead {
    position: sticky;
    top: 0;
    background: var(--bg-secondary);
    z-index: 10;
  }

  .log-table th {
    text-align: left;
    padding: 0.75rem 0.5rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    font-size: 0.75rem;
    border-bottom: 2px solid var(--border-color);
  }

  .log-table td {
    padding: 0.5rem;
    border-bottom: 1px solid var(--border-color);
    vertical-align: middle;
  }

  .log-row:hover {
    background: var(--bg-hover);
  }

  .mono {
    font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
    font-size: 0.75rem;
  }

  .col-time { width: 100px; }
  .col-dir { width: 50px; }
  .col-frame { width: 100px; }
  .col-cmd { width: 180px; }
  .col-addr { width: 100px; }
  .col-data { width: 180px; }
  .col-parsed { flex: 1; }

  .direction-badge {
    display: inline-block;
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .direction-tx {
    background: rgba(147, 51, 234, 0.2);
    color: var(--accent-purple);
  }

  .direction-rx {
    background: rgba(34, 197, 94, 0.2);
    color: var(--accent-green);
  }

  .command-badge {
    display: inline-block;
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-family: monospace;
    margin-right: 0.5rem;
  }

  .cmd-data {
    background: rgba(59, 130, 246, 0.2);
    color: var(--accent-blue);
  }

  .cmd-alarm {
    background: rgba(239, 68, 68, 0.2);
    color: var(--accent-red);
  }

  .cmd-debug {
    background: rgba(234, 179, 8, 0.2);
    color: var(--accent-yellow);
  }

  .cmd-control {
    background: rgba(107, 114, 128, 0.2);
    color: var(--text-secondary);
  }

  .command-name {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .empty-message {
    text-align: center;
    padding: 3rem !important;
    color: var(--text-secondary);
    font-style: italic;
  }

  .status-bar {
    display: flex;
    gap: 2rem;
    padding: 0.5rem 1rem;
    background: var(--bg-secondary);
    border-top: 1px solid var(--border-color);
    font-size: 0.75rem;
  }

  .status-item {
    display: flex;
    gap: 0.5rem;
  }

  .status-label {
    color: var(--text-secondary);
  }

  .status-value {
    font-weight: 600;
  }

  .status-recording {
    color: var(--accent-red);
  }

  .status-stopped {
    color: var(--text-secondary);
  }
</style>
