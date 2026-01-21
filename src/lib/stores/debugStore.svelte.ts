// Debug Log Store - Svelte 5 reactive state management
import { invoke } from '@tauri-apps/api/core';

export interface CanLogEntry {
  id: number;
  timestamp: Date;
  direction: 'TX' | 'RX';
  frameId: number;
  frameIdHex: string;
  command: string;
  commandCode: number;
  source: number;
  destination: number;
  data: number[];
  dataHex: string;
  parsedValue: string;
}

// State
let logs = $state<CanLogEntry[]>([]);
let isRecording = $state(false);
let logIdCounter = $state(0);
let maxLogs = $state(1000); // Maximum logs to keep in memory

// Export reactive getters
export function getLogs() { return logs; }
export function getIsRecording() { return isRecording; }
export function getMaxLogs() { return maxLogs; }

// Start recording
export function startRecording(): void {
  isRecording = true;
  logs = [];
  logIdCounter = 0;
}

// Stop recording
export function stopRecording(): void {
  isRecording = false;
}

// Clear logs
export function clearLogs(): void {
  logs = [];
  logIdCounter = 0;
}

// Set max logs
export function setMaxLogs(max: number): void {
  maxLogs = max;
}

// Add a log entry
export function addLogEntry(entry: Omit<CanLogEntry, 'id' | 'timestamp' | 'frameIdHex' | 'dataHex' | 'command' | 'parsedValue'>): void {
  if (!isRecording) return;

  const newEntry: CanLogEntry = {
    ...entry,
    id: logIdCounter++,
    timestamp: new Date(),
    frameIdHex: `0x${entry.frameId.toString(16).toUpperCase().padStart(8, '0')}`,
    dataHex: entry.data.map(b => b.toString(16).toUpperCase().padStart(2, '0')).join(' '),
    command: getCommandName(entry.commandCode),
    parsedValue: parseDataValue(entry.commandCode, entry.data),
  };

  logs = [...logs, newEntry].slice(-maxLogs);
}

// Parse CAN frame and add to log
export function logCanFrame(
  direction: 'TX' | 'RX',
  frameId: number,
  data: number[]
): void {
  if (!isRecording) return;

  // Parse frame ID (29-bit extended)
  const ptp = (frameId >> 28) & 1;
  const commandCode = (frameId >> 20) & 0xFF;
  const destination = (frameId >> 12) & 0xFF;
  const source = (frameId >> 4) & 0xFF;

  addLogEntry({
    direction,
    frameId,
    commandCode,
    source,
    destination,
    data,
  });
}

// Get command name from code
function getCommandName(code: number): string {
  const commands: Record<number, string> = {
    0x00: 'Shutdown',
    0x10: 'Force Output',
    0x11: 'Reset',
    0x80: 'Charge/Discharge Limits',
    0x81: 'SOC/SOH',
    0x82: 'Voltage/Current',
    0x83: 'Cell Voltage',
    0x84: 'Temperature',
    0x85: 'Operation Status',
    0x86: 'Accumulated Times',
    0x87: 'Accumulated Power',
    0x8F: 'Software Version',
    0xC0: 'Alarm Status',
    0xD0: 'Debug Status',
  };
  return commands[code] || `Unknown (0x${code.toString(16).toUpperCase()})`;
}

// Parse data value based on command
function parseDataValue(commandCode: number, data: number[]): string {
  if (data.length < 2) return 'Insufficient data';

  try {
    switch (commandCode) {
      case 0x80: { // Charge/Discharge Limits
        if (data.length < 8) return 'Insufficient data';
        const chargeV = ((data[1] << 8) | data[0]) * 0.1;
        const chargeI = ((data[3] << 8) | data[2]) * 0.1;
        const dischargeV = ((data[5] << 8) | data[4]) * 0.1;
        const dischargeI = ((data[7] << 8) | data[6]) * 0.1;
        return `Charge: ${chargeV.toFixed(1)}V/${chargeI.toFixed(1)}A, Discharge: ${dischargeV.toFixed(1)}V/${dischargeI.toFixed(1)}A`;
      }

      case 0x81: { // SOC/SOH
        if (data.length < 6) return 'Insufficient data';
        const soc = ((data[1] << 8) | data[0]) * 0.1;
        const soh = ((data[3] << 8) | data[2]) * 0.1;
        const backup = (data[5] << 8) | data[4];
        return `SOC: ${soc.toFixed(1)}%, SOH: ${soh.toFixed(1)}%, Backup: ${backup}min`;
      }

      case 0x82: { // Voltage/Current
        if (data.length < 4) return 'Insufficient data';
        const voltage = ((data[1] << 8) | data[0]) * 0.1;
        const currentRaw = (data[3] << 8) | data[2];
        const current = currentRaw > 32767 ? (currentRaw - 65536) * 0.1 : currentRaw * 0.1;
        const power = (voltage * Math.abs(current) / 1000).toFixed(2);
        return `Voltage: ${voltage.toFixed(1)}V, Current: ${current.toFixed(1)}A, Power: ${power}kW`;
      }

      case 0x83: { // Cell Voltage
        if (data.length < 8) return 'Insufficient data';
        const maxV = ((data[1] << 8) | data[0]) * 0.001;
        const maxPack = data[2];
        const maxCell = data[3];
        const minV = ((data[5] << 8) | data[4]) * 0.001;
        const minPack = data[6];
        const minCell = data[7];
        return `Max: ${maxV.toFixed(3)}V (P${maxPack}C${maxCell}), Min: ${minV.toFixed(3)}V (P${minPack}C${minCell})`;
      }

      case 0x84: { // Temperature
        if (data.length < 8) return 'Insufficient data';
        const maxTRaw = (data[1] << 8) | data[0];
        const maxT = maxTRaw > 32767 ? (maxTRaw - 65536) * 0.1 : maxTRaw * 0.1;
        const maxTPack = data[2];
        const maxTSensor = data[3];
        const minTRaw = (data[5] << 8) | data[4];
        const minT = minTRaw > 32767 ? (minTRaw - 65536) * 0.1 : minTRaw * 0.1;
        const minTPack = data[6];
        const minTSensor = data[7];
        return `Max: ${maxT.toFixed(1)}°C (P${maxTPack}S${maxTSensor}), Min: ${minT.toFixed(1)}°C (P${minTPack}S${minTSensor})`;
      }

      case 0x85: { // Operation Status
        if (data.length < 4) return 'Insufficient data';
        const sysStatus = ['PowerOn', 'Start', 'Alone', 'Charge', 'Discharge', 'WaitCharge', 'WaitDischarge', 'Lock'][data[0]] || 'Unknown';
        const workStatus = ['Empty', 'Boot', 'Shutdown'][data[1]] || 'Unknown';
        const opStatus = ['Empty', 'Normal', 'Alarm', 'Fault'][data[2]] || 'Unknown';
        return `System: ${sysStatus}, Work: ${workStatus}, Op: ${opStatus}`;
      }

      case 0x86: { // Accumulated Times
        if (data.length < 4) return 'Insufficient data';
        const chargeTimes = (data[1] << 8) | data[0];
        const dischargeTimes = (data[3] << 8) | data[2];
        return `Charge cycles: ${chargeTimes}, Discharge cycles: ${dischargeTimes}`;
      }

      case 0x87: { // Accumulated Power
        if (data.length < 8) return 'Insufficient data';
        const chargeEnergy = ((data[3] << 24) | (data[2] << 16) | (data[1] << 8) | data[0]) * 0.1;
        const dischargeEnergy = ((data[7] << 24) | (data[6] << 16) | (data[5] << 8) | data[4]) * 0.1;
        return `Charge: ${chargeEnergy.toFixed(1)}kWh, Discharge: ${dischargeEnergy.toFixed(1)}kWh`;
      }

      case 0x8F: { // Software Version
        const version = data.filter(b => b !== 0).map(b => String.fromCharCode(b)).join('');
        return `Version: ${version}`;
      }

      case 0xC0: { // Alarm Status
        const alarmBits: string[] = [];
        for (let i = 0; i < 8; i++) {
          for (let bit = 0; bit < 8; bit++) {
            if ((data[i] >> bit) & 1) {
              alarmBits.push(`Bit${i * 8 + bit}`);
            }
          }
        }
        return alarmBits.length > 0 ? `Alarms: ${alarmBits.join(', ')}` : 'No alarms';
      }

      default:
        return `Raw: ${data.map(b => b.toString(16).toUpperCase().padStart(2, '0')).join(' ')}`;
    }
  } catch {
    return 'Parse error';
  }
}

// Export logs to CSV
export function exportToCsv(): string {
  const headers = ['ID', 'Timestamp', 'Direction', 'Frame ID', 'Command Code', 'Command', 'Source', 'Destination', 'Data (Hex)', 'Parsed Value'];
  const rows = logs.map(log => [
    log.id,
    log.timestamp.toISOString(),
    log.direction,
    log.frameIdHex,
    `0x${log.commandCode.toString(16).toUpperCase().padStart(2, '0')}`,
    log.command,
    `0x${log.source.toString(16).toUpperCase().padStart(2, '0')}`,
    `0x${log.destination.toString(16).toUpperCase().padStart(2, '0')}`,
    log.dataHex,
    `"${log.parsedValue.replace(/"/g, '""')}"`,
  ]);

  return [headers.join(','), ...rows.map(row => row.join(','))].join('\n');
}

// Export logs to JSON
export function exportToJson(): string {
  return JSON.stringify(logs.map(log => ({
    ...log,
    timestamp: log.timestamp.toISOString(),
  })), null, 2);
}

// Download logs as file
export function downloadLogs(format: 'csv' | 'json'): void {
  const content = format === 'csv' ? exportToCsv() : exportToJson();
  const mimeType = format === 'csv' ? 'text/csv' : 'application/json';
  const filename = `bms_debug_log_${new Date().toISOString().replace(/[:.]/g, '-')}.${format}`;

  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

// Generate test data for demonstration (simulation mode)
export function generateTestData(): void {
  if (!isRecording) return;

  const testFrames = [
    { cmd: 0x80, data: [0x90, 0x21, 0xE8, 0x03, 0x40, 0x1A, 0xE8, 0x03] }, // Limits
    { cmd: 0x81, data: [0x50, 0x00, 0x64, 0x00, 0x3C, 0x00, 0x00, 0x00] }, // SOC/SOH
    { cmd: 0x82, data: [0xB9, 0x1F, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00] }, // Voltage/Current
    { cmd: 0x83, data: [0x42, 0x0D, 0x01, 0x05, 0x38, 0x0D, 0x02, 0x08] }, // Cell Voltage
    { cmd: 0x84, data: [0x0E, 0x01, 0x01, 0x03, 0xF8, 0x00, 0x02, 0x05] }, // Temperature
    { cmd: 0x85, data: [0x04, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00] }, // Status
    { cmd: 0xC0, data: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] }, // Alarm
  ];

  // Generate TX (query) and RX (response) pairs
  for (const frame of testFrames) {
    // TX - Query frame from Host (0x80) to BMS (0x01)
    const txFrameId = (1 << 28) | (frame.cmd << 20) | (0x01 << 12) | (0x80 << 4);
    logCanFrame('TX', txFrameId, []);

    // RX - Response frame from BMS (0x01) to Host (0x80)
    const rxFrameId = (1 << 28) | (frame.cmd << 20) | (0x80 << 12) | (0x01 << 4);
    logCanFrame('RX', rxFrameId, frame.data);
  }
}
