// BMS CAN Protocol Types based on Ecube BMS-PCS Communication Protocol V1.20

/** Command types for BMS communication */
export enum BMSCommand {
  /** 0x80 - Charge/discharge voltage and current limits */
  CHARGE_DISCHARGE_LIMITS = 0x80,
  /** 0x81 - SOC, SOH, backup time */
  SOC_SOH = 0x81,
  /** 0x82 - Battery output voltage and current */
  VOLTAGE_CURRENT = 0x82,
  /** 0x83 - Cell voltage max/min */
  CELL_VOLTAGE = 0x83,
  /** 0x84 - Temperature max/min */
  TEMPERATURE = 0x84,
  /** 0x85 - Operation status */
  OPERATION_STATUS = 0x85,
  /** 0x86 - Accumulated charge/discharge times */
  ACCUMULATED_TIMES = 0x86,
  /** 0x87 - Accumulated charge/discharge power */
  ACCUMULATED_POWER = 0x87,
  /** 0x8F - BMS software version */
  SOFTWARE_VERSION = 0x8f,
  /** 0x00 - Shutdown command (initiative report) */
  SHUTDOWN = 0x00,
  /** 0x10 - Force BMS output */
  FORCE_OUTPUT = 0x10,
  /** 0x11 - BMS reset */
  RESET = 0x11,
  /** 0xC0 - Alarm status code */
  ALARM_STATUS = 0xc0,
  /** 0xD0 - Debug status code */
  DEBUG_STATUS = 0xd0,
}

/** System status values */
export enum SystemStatus {
  POWER_ON = 0,
  START = 1,
  ALONE = 2,
  CHARGE = 3,
  DISCHARGE = 4,
  WAIT_TO_CHARGE = 5,
  WAIT_TO_DISCHARGE = 6,
  LOCK = 7,
}

/** Work status values */
export enum WorkStatus {
  EMPTY = 0,
  BOOT = 1,
  SHUT_DOWN = 2,
}

/** Operation status values */
export enum OperationStatus {
  EMPTY = 0,
  NORMAL = 1,
  ALARM = 2,
  FAULT = 3,
}

/** Shutdown reason codes */
export enum ShutdownReason {
  INVALID = 0,
  UNDER_VOLTAGE = 1,
  OVER_CURRENT = 2,
  OVER_TEMPERATURE = 3,
  UNDER_TEMPERATURE = 4,
  OVER_VOLTAGE = 5,
  COMM_ERROR = 6,
}

/** Alarm bit definitions */
export enum AlarmBit {
  CELL_OVER_VOLTAGE = 0,
  CELL_UNDER_VOLTAGE = 1,
  CHARGING_OVER_TEMP_ALARM = 2,
  CHARGING_LOW_TEMP_ALARM = 3,
  DISCHARGING_OVER_TEMP_PREALARM = 4,
  DISCHARGING_LOW_TEMP_PREALARM = 5,
  DISCHARGING_OVER_CURRENT_PREALARM = 6,
  CHARGING_OVER_CURRENT_PREALARM = 7,
  TOTAL_OVER_VOLTAGE_PREALARM = 8,
  TOTAL_UNDER_VOLTAGE_WARNING = 9,
  CIRCUIT_BREAKER_DISCONNECTED = 10,
  BALANCED_CHARGING_FAILED = 11,
  POSITIVE_PACK_VOLTAGE_IMBALANCE = 12,
  NEGATIVE_PACK_VOLTAGE_IMBALANCE = 13,
  BMU_COMMUNICATION_INTERRUPTION = 14,
  WATER_FLOODING_DETECTION_ALARM = 15,
  WATER_FLOODING_PROTECTION = 16,
  CHARGING_OVER_TEMP_PROTECTION = 18,
  CHARGING_LOW_TEMP_PROTECTION = 19,
  DISCHARGING_OVER_TEMP_PROTECTION = 20,
  DISCHARGING_LOW_TEMP_PROTECTION = 21,
  DISCHARGING_OVER_CURRENT_PROTECTION_L1 = 22,
  DISCHARGING_OVER_CURRENT_PROTECTION_L2 = 23,
  CHARGING_OVER_CURRENT_PROTECTION_L1 = 24,
  CHARGING_OVER_CURRENT_PROTECTION_L2 = 25,
  CHARGING_OVER_CURRENT_PROTECTION_L3 = 26,
  TOTAL_CHARGING_OVER_VOLTAGE_PROTECTION = 27,
  TOTAL_CHARGING_UNDER_VOLTAGE_PROTECTION = 28,
  CHARGING_DC_CONTACTOR_FAILURE = 29,
  DISCHARGING_DC_CONTACTOR_FAILURE = 30,
  EPO_SHUTDOWN = 31,
  FIRE_PROTECTION = 32,
  PARALLEL_COMMUNICATION_ABNORMALITY = 33,
  PARALLEL_ADDRESS_CONFLICT = 34,
  INSULATION_MONITORING_ALARM = 35,
  HYDROGEN_PROTECTION = 36,
  BATTERY_PACK_FAN_MALFUNCTION = 37,
  BATTERY_PACK_FUSE_TEMP_HIGH = 38,
  CAN_HALL_COMMUNICATION_INTERRUPTION = 39,
  CAN_HALL_DATA_FAILURE = 40,
}

/** Command 0x80 - Charge/Discharge Limits */
export interface ChargeDischargeLimits {
  /** Charge voltage limit in V (0.1V resolution) */
  chargeVoltageLimit: number;
  /** Charge current limit in A (0.1A resolution) */
  chargeCurrentLimit: number;
  /** Discharge voltage limit in V (0.1V resolution) */
  dischargeVoltageLimit: number;
  /** Discharge current limit in A (0.1A resolution) */
  dischargeCurrentLimit: number;
}

/** Command 0x81 - SOC/SOH Data */
export interface SocSohData {
  /** State of Charge in % */
  soc: number;
  /** State of Health in % */
  soh: number;
  /** Battery backup time in minutes */
  backupTimeMinutes: number;
}

/** Command 0x82 - Voltage/Current Data */
export interface VoltageCurrentData {
  /** Battery output voltage in V (0.1V resolution) */
  voltage: number;
  /** Battery output current in A (0.1A resolution, positive=discharge, negative=charge) */
  current: number;
  /** Power in kW (calculated) */
  power: number;
}

/** Command 0x83 - Cell Voltage Data */
export interface CellVoltageData {
  /** Maximum cell voltage in V (0.001V resolution) */
  maxVoltage: number;
  /** PACK number containing max voltage cell */
  maxVoltagePackNo: number;
  /** Cell number with max voltage */
  maxVoltageCellNo: number;
  /** Minimum cell voltage in V (0.001V resolution) */
  minVoltage: number;
  /** PACK number containing min voltage cell */
  minVoltagePackNo: number;
  /** Cell number with min voltage */
  minVoltageCellNo: number;
  /** Voltage difference between max and min */
  voltageDelta: number;
}

/** Command 0x84 - Temperature Data */
export interface TemperatureData {
  /** Maximum temperature in °C (0.1°C resolution) */
  maxTemperature: number;
  /** PACK number with max temperature */
  maxTempPackNo: number;
  /** Sensor number with max temperature */
  maxTempSensorNo: number;
  /** Minimum temperature in °C (0.1°C resolution) */
  minTemperature: number;
  /** PACK number with min temperature */
  minTempPackNo: number;
  /** Sensor number with min temperature */
  minTempSensorNo: number;
  /** Temperature difference between max and min */
  tempDelta: number;
}

/** Command 0x85 - Operation Status Data */
export interface OperationStatusData {
  /** System status code */
  systemStatus: SystemStatus;
  /** Work status code */
  workStatus: WorkStatus;
  /** Operation status code */
  operationStatus: OperationStatus;
  /** Discharge prohibited (can be allowed after OT/UT cleared) */
  dischargeProhibited: boolean;
  /** Charging prohibited */
  chargeProhibited: boolean;
  /** Discharge prohibited (cannot be allowed after OC/UV cleared) */
  dischargeProhibitedHard: boolean;
}

/** Command 0x86 - Accumulated Times */
export interface AccumulatedTimesData {
  /** Number of full charge cycles */
  chargeTimes: number;
  /** Number of full discharge cycles */
  dischargeTimes: number;
}

/** Command 0x87 - Accumulated Power */
export interface AccumulatedPowerData {
  /** Accumulated charging energy in kWh (0.1kWh resolution) */
  chargeEnergy: number;
  /** Accumulated discharging energy in kWh (0.1kWh resolution) */
  dischargeEnergy: number;
}

/** Command 0xC0 - Alarm Status */
export interface AlarmStatus {
  /** Raw 64-bit alarm status */
  rawStatus: bigint;
  /** Active alarm bits */
  activeAlarms: AlarmBit[];
  /** Severity level (1=mild, 2=moderate, 3=severe) */
  maxSeverity: number;
}

/** Complete BMS Data */
export interface BMSData {
  /** Timestamp of last update */
  timestamp: Date;
  /** Connection status */
  connected: boolean;
  /** Charge/discharge limits */
  limits: ChargeDischargeLimits | null;
  /** SOC/SOH data */
  socSoh: SocSohData | null;
  /** Voltage/current data */
  voltageCurrent: VoltageCurrentData | null;
  /** Cell voltage data */
  cellVoltage: CellVoltageData | null;
  /** Temperature data */
  temperature: TemperatureData | null;
  /** Operation status */
  operationStatus: OperationStatusData | null;
  /** Accumulated times */
  accumulatedTimes: AccumulatedTimesData | null;
  /** Accumulated power */
  accumulatedPower: AccumulatedPowerData | null;
  /** BMS software version */
  softwareVersion: string | null;
  /** Alarm status */
  alarmStatus: AlarmStatus | null;
}

/** CAN Frame structure */
export interface CANFrame {
  /** 29-bit extended identifier */
  id: number;
  /** Data bytes (1-8 bytes) */
  data: Uint8Array;
  /** Timestamp */
  timestamp: number;
}

/** Parsed CAN Frame ID */
export interface ParsedFrameId {
  /** Point-to-point flag */
  ptp: boolean;
  /** Command code */
  command: number;
  /** Destination address */
  destinationAddress: number;
  /** Source address */
  sourceAddress: number;
  /** Continuation flag */
  cnt: boolean;
}

/** Serial port configuration */
export interface SerialPortConfig {
  /** Port name (e.g., COM3, /dev/ttyUSB0) */
  port: string;
  /** Baud rate (default 125000 for CAN) */
  baudRate: number;
}

/** USB-CAN adapter configuration */
export interface CANAdapterConfig {
  /** Adapter type */
  adapterType: "I+BT" | "I+USB" | "SocketCAN" | "iTEKON" | "Simulation" | "Other";
  /** CAN baud rate (default 125000) */
  canBaudRate: number;
  /** Serial port for USB adapters */
  serialPort?: SerialPortConfig;
  /** SocketCAN interface name (e.g., can0) */
  socketCanInterface?: string;
}
