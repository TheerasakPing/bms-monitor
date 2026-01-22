// BMS Store - Svelte 5 reactive state management
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { BMSData, ChargeDischargeLimits, SocSohData, VoltageCurrentData, CellVoltageData, TemperatureData, OperationStatusData, AccumulatedTimesData, AccumulatedPowerData } from '../types/bms';
import { logCanFrame, getIsRecording } from './debugStore.svelte';

// CAN frame event payload from Rust
interface CanFrameEvent {
  direction: 'tx' | 'rx';
  frame_id: number;
  data: number[];
}

// Connection state
let connected = $state(false);
let selectedPort = $state('');
let availablePorts = $state<string[]>([]);
let adapterType = $state<'usb' | 'bluetooth' | 'itekon' | 'simulation'>('simulation');
let connectionError = $state<string | null>(null);

// BMS data state
let bmsData = $state<BMSData>({
  timestamp: new Date(),
  connected: false,
  limits: null,
  socSoh: null,
  voltageCurrent: null,
  cellVoltage: null,
  temperature: null,
  operationStatus: null,
  accumulatedTimes: null,
  accumulatedPower: null,
  softwareVersion: null,
  alarmStatus: null,
});

// Polling interval
let pollingInterval: number | null = null;
let canFrameUnlisten: UnlistenFn | null = null;

// Export reactive getters
export function getConnected() { return connected; }
export function getSelectedPort() { return selectedPort; }
export function getAvailablePorts() { return availablePorts; }
export function getAdapterType() { return adapterType; }
export function getConnectionError() { return connectionError; }
export function getBmsData() { return bmsData; }

// Setters
export function setSelectedPort(port: string) { selectedPort = port; }
export function setAdapterType(type: 'usb' | 'bluetooth' | 'itekon' | 'simulation') { adapterType = type; }

// List available serial ports
export async function listPorts(): Promise<void> {
  try {
    const result = await invoke<{ success: boolean; data: string[] | null; error: string | null }>('list_ports');
    if (result.success && result.data) {
      availablePorts = result.data;
    }
  } catch (error) {
    console.error('Failed to list ports:', error);
  }
}

// Connect to BMS
export async function connect(): Promise<boolean> {
  try {
    connectionError = null;

    const config = {
      adapter_type: adapterType,
      serial_port: (adapterType !== 'simulation' && adapterType !== 'itekon') ? selectedPort : null,
      serial_baud_rate: 115200,
      bms_address: 0x01,
    };

    const result = await invoke<{ success: boolean; data: boolean | null; error: string | null }>('connect', { config });

    if (result.success) {
      connected = true;

      // Listen for CAN frame events from Rust backend
      canFrameUnlisten = await listen<CanFrameEvent>('can-frame', (event) => {
        if (getIsRecording()) {
          logCanFrame(
            event.payload.direction === 'tx' ? 'TX' : 'RX',
            event.payload.frame_id,
            event.payload.data
          );
        }
      });

      startPolling();
      return true;
    } else {
      connectionError = result.error || 'Connection failed';
      return false;
    }
  } catch (error) {
    connectionError = String(error);
    return false;
  }
}

// Disconnect from BMS
export async function disconnect(): Promise<void> {
  try {
    stopPolling();

    // Stop listening for CAN frame events
    if (canFrameUnlisten) {
      canFrameUnlisten();
      canFrameUnlisten = null;
    }

    await invoke('disconnect');
    connected = false;
    bmsData = {
      timestamp: new Date(),
      connected: false,
      limits: null,
      socSoh: null,
      voltageCurrent: null,
      cellVoltage: null,
      temperature: null,
      operationStatus: null,
      accumulatedTimes: null,
      accumulatedPower: null,
      softwareVersion: null,
      alarmStatus: null,
    };
  } catch (error) {
    console.error('Disconnect error:', error);
  }
}

// Fetch BMS data from backend
export async function fetchBmsData(): Promise<void> {
  if (!connected) return;

  try {
    // Query all data
    await invoke('query_all_data');

    // Get the data - Rust now serializes with camelCase
    const data = await invoke<{
      timestamp: number;
      connected: boolean;
      limits: ChargeDischargeLimits | null;
      socSoh: SocSohData | null;
      voltageCurrent: VoltageCurrentData | null;
      cellVoltage: CellVoltageData | null;
      temperature: TemperatureData | null;
      operationStatus: OperationStatusData | null;
      accumulatedTimes: AccumulatedTimesData | null;
      accumulatedPower: AccumulatedPowerData | null;
      softwareVersion: string | null;
      alarmStatus: { rawStatus: number; activeAlarms: number[]; maxSeverity: number } | null;
    }>('get_bms_data');

    // Data is already in camelCase from Rust
    bmsData = {
      timestamp: new Date(data.timestamp),
      connected: data.connected,
      limits: data.limits,
      socSoh: data.socSoh,
      voltageCurrent: data.voltageCurrent,
      cellVoltage: data.cellVoltage,
      temperature: data.temperature,
      operationStatus: data.operationStatus,
      accumulatedTimes: data.accumulatedTimes,
      accumulatedPower: data.accumulatedPower,
      softwareVersion: data.softwareVersion,
      alarmStatus: data.alarmStatus ? {
        rawStatus: BigInt(data.alarmStatus.rawStatus),
        activeAlarms: data.alarmStatus.activeAlarms,
        maxSeverity: data.alarmStatus.maxSeverity,
      } : null,
    };
  } catch (error) {
    console.error('Failed to fetch BMS data:', error);
  }
}

// Start polling for data
function startPolling(): void {
  if (pollingInterval) return;

  pollingInterval = window.setInterval(() => {
    fetchBmsData();
  }, 2000); // Poll every 2 seconds to avoid overload

  // Fetch immediately
  fetchBmsData();
}

// Stop polling
function stopPolling(): void {
  if (pollingInterval) {
    clearInterval(pollingInterval);
    pollingInterval = null;
  }
}

// Get alarm description
export async function getAlarmDescription(bit: number): Promise<string> {
  const descriptions: Record<number, string> = {
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
    12: 'Positive battery pack voltage imbalance',
    13: 'Negative battery pack voltage imbalance',
    14: 'BMU communication interruption',
    15: 'Water flooding detection alarm',
    16: 'Water flooding detection and protection',
    18: 'Charging over temperature protection',
    19: 'Charging low temperature protection',
    20: 'Discharging over temperature protection',
    21: 'Discharging low temperature protection',
    22: 'Discharging over current protection level 1',
    23: 'Discharging over current protection level 2',
    24: 'Charging over current protection level 1',
    25: 'Charging over current protection level 2',
    26: 'Charging over current protection level 3',
    27: 'Total charging over voltage protection',
    28: 'Total charging under voltage protection',
    29: 'Charging DC contactor failure',
    30: 'Discharging DC contactor failure',
    31: 'EPO shut down',
    32: 'Fire protection',
    33: 'Parallel communication abnormality',
    34: 'Parallel address conflict',
    35: 'Insulation monitoring alarm',
    36: 'Hydrogen protection',
    37: 'Battery pack fan malfunction',
    38: 'Battery pack fuse temperature too high',
    39: 'CAN Hall communication interruption',
    40: 'CAN Hall data failure',
  };

  return descriptions[bit] || `Unknown alarm (bit ${bit})`;
}

// Get alarm severity
export function getAlarmSeverity(bit: number): number {
  const level3 = [0, 1, 14, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
  const level2 = [2, 3, 4, 5, 6, 7, 8, 9];

  if (level3.includes(bit)) return 3;
  if (level2.includes(bit)) return 2;
  return 1;
}

// Get system status name
export function getSystemStatusName(status: number): string {
  const names: Record<number, string> = {
    0: 'Power On',
    1: 'Start',
    2: 'Alone',
    3: 'Charging',
    4: 'Discharging',
    5: 'Wait to Charge',
    6: 'Wait to Discharge',
    7: 'Lock',
  };
  return names[status] || 'Unknown';
}

// Get work status name
export function getWorkStatusName(status: number): string {
  const names: Record<number, string> = {
    0: 'Empty',
    1: 'Boot',
    2: 'Shut Down',
  };
  return names[status] || 'Unknown';
}

// Get operation status name
export function getOperationStatusName(status: number): string {
  const names: Record<number, string> = {
    0: 'Empty',
    1: 'Normal',
    2: 'Alarm',
    3: 'Fault',
  };
  return names[status] || 'Unknown';
}
