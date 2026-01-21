//! BMS Protocol Types and Constants
//! Based on Ecube BMS-PCS Communication Protocol V1.20

use serde::{Deserialize, Serialize};

/// CAN baud rate for BMS communication (125Kbps)
pub const CAN_BAUD_RATE: u32 = 125_000;

/// BMS Command codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum BmsCommand {
    /// 0x80 - Charge/discharge voltage and current limits
    ChargeDischargeLimits = 0x80,
    /// 0x81 - SOC, SOH, backup time
    SocSoh = 0x81,
    /// 0x82 - Battery output voltage and current
    VoltageCurrent = 0x82,
    /// 0x83 - Cell voltage max/min
    CellVoltage = 0x83,
    /// 0x84 - Temperature max/min
    Temperature = 0x84,
    /// 0x85 - Operation status
    OperationStatus = 0x85,
    /// 0x86 - Accumulated charge/discharge times
    AccumulatedTimes = 0x86,
    /// 0x87 - Accumulated charge/discharge power
    AccumulatedPower = 0x87,
    /// 0x8F - BMS software version
    SoftwareVersion = 0x8F,
    /// 0x00 - Shutdown command (initiative report)
    Shutdown = 0x00,
    /// 0x10 - Force BMS output
    ForceOutput = 0x10,
    /// 0x11 - BMS reset
    Reset = 0x11,
    /// 0xC0 - Alarm status code
    AlarmStatus = 0xC0,
    /// 0xD0 - Debug status code
    DebugStatus = 0xD0,
}

impl TryFrom<u8> for BmsCommand {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x80 => Ok(BmsCommand::ChargeDischargeLimits),
            0x81 => Ok(BmsCommand::SocSoh),
            0x82 => Ok(BmsCommand::VoltageCurrent),
            0x83 => Ok(BmsCommand::CellVoltage),
            0x84 => Ok(BmsCommand::Temperature),
            0x85 => Ok(BmsCommand::OperationStatus),
            0x86 => Ok(BmsCommand::AccumulatedTimes),
            0x87 => Ok(BmsCommand::AccumulatedPower),
            0x8F => Ok(BmsCommand::SoftwareVersion),
            0x00 => Ok(BmsCommand::Shutdown),
            0x10 => Ok(BmsCommand::ForceOutput),
            0x11 => Ok(BmsCommand::Reset),
            0xC0 => Ok(BmsCommand::AlarmStatus),
            0xD0 => Ok(BmsCommand::DebugStatus),
            _ => Err(()),
        }
    }
}

/// System status values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum SystemStatus {
    PowerOn = 0,
    Start = 1,
    Alone = 2,
    Charge = 3,
    Discharge = 4,
    WaitToCharge = 5,
    WaitToDischarge = 6,
    Lock = 7,
}

impl From<u8> for SystemStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => SystemStatus::PowerOn,
            1 => SystemStatus::Start,
            2 => SystemStatus::Alone,
            3 => SystemStatus::Charge,
            4 => SystemStatus::Discharge,
            5 => SystemStatus::WaitToCharge,
            6 => SystemStatus::WaitToDischarge,
            7 => SystemStatus::Lock,
            _ => SystemStatus::PowerOn,
        }
    }
}

/// Work status values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum WorkStatus {
    Empty = 0,
    Boot = 1,
    ShutDown = 2,
}

impl From<u8> for WorkStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => WorkStatus::Empty,
            1 => WorkStatus::Boot,
            2 => WorkStatus::ShutDown,
            _ => WorkStatus::Empty,
        }
    }
}

/// Operation status values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum OperationStatusCode {
    Empty = 0,
    Normal = 1,
    Alarm = 2,
    Fault = 3,
}

impl From<u8> for OperationStatusCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OperationStatusCode::Empty,
            1 => OperationStatusCode::Normal,
            2 => OperationStatusCode::Alarm,
            3 => OperationStatusCode::Fault,
            _ => OperationStatusCode::Empty,
        }
    }
}

/// Shutdown reason codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ShutdownReason {
    Invalid = 0,
    UnderVoltage = 1,
    OverCurrent = 2,
    OverTemperature = 3,
    UnderTemperature = 4,
    OverVoltage = 5,
    CommError = 6,
}

impl From<u8> for ShutdownReason {
    fn from(value: u8) -> Self {
        match value {
            1 => ShutdownReason::UnderVoltage,
            2 => ShutdownReason::OverCurrent,
            3 => ShutdownReason::OverTemperature,
            4 => ShutdownReason::UnderTemperature,
            5 => ShutdownReason::OverVoltage,
            6 => ShutdownReason::CommError,
            _ => ShutdownReason::Invalid,
        }
    }
}

/// Alarm bit definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum AlarmBit {
    CellOverVoltage = 0,
    CellUnderVoltage = 1,
    ChargingOverTempAlarm = 2,
    ChargingLowTempAlarm = 3,
    DischargingOverTempPrealarm = 4,
    DischargingLowTempPrealarm = 5,
    DischargingOverCurrentPrealarm = 6,
    ChargingOverCurrentPrealarm = 7,
    TotalOverVoltagePrealarm = 8,
    TotalUnderVoltageWarning = 9,
    CircuitBreakerDisconnected = 10,
    BalancedChargingFailed = 11,
    PositivePackVoltageImbalance = 12,
    NegativePackVoltageImbalance = 13,
    BmuCommunicationInterruption = 14,
    WaterFloodingDetectionAlarm = 15,
    WaterFloodingProtection = 16,
    ChargingOverTempProtection = 18,
    ChargingLowTempProtection = 19,
    DischargingOverTempProtection = 20,
    DischargingLowTempProtection = 21,
    DischargingOverCurrentProtectionL1 = 22,
    DischargingOverCurrentProtectionL2 = 23,
    ChargingOverCurrentProtectionL1 = 24,
    ChargingOverCurrentProtectionL2 = 25,
    ChargingOverCurrentProtectionL3 = 26,
    TotalChargingOverVoltageProtection = 27,
    TotalChargingUnderVoltageProtection = 28,
    ChargingDcContactorFailure = 29,
    DischargingDcContactorFailure = 30,
    EpoShutdown = 31,
    FireProtection = 32,
    ParallelCommunicationAbnormality = 33,
    ParallelAddressConflict = 34,
    InsulationMonitoringAlarm = 35,
    HydrogenProtection = 36,
    BatteryPackFanMalfunction = 37,
    BatteryPackFuseTempHigh = 38,
    CanHallCommunicationInterruption = 39,
    CanHallDataFailure = 40,
}

/// Get alarm severity level (1=mild, 2=moderate, 3=severe)
pub fn get_alarm_severity(alarm: AlarmBit) -> u8 {
    match alarm {
        AlarmBit::CellOverVoltage
        | AlarmBit::CellUnderVoltage
        | AlarmBit::BmuCommunicationInterruption
        | AlarmBit::ChargingOverTempProtection
        | AlarmBit::ChargingLowTempProtection
        | AlarmBit::DischargingOverTempProtection
        | AlarmBit::DischargingLowTempProtection
        | AlarmBit::DischargingOverCurrentProtectionL1
        | AlarmBit::DischargingOverCurrentProtectionL2
        | AlarmBit::ChargingOverCurrentProtectionL1
        | AlarmBit::ChargingOverCurrentProtectionL2
        | AlarmBit::ChargingOverCurrentProtectionL3
        | AlarmBit::TotalChargingOverVoltageProtection
        | AlarmBit::TotalChargingUnderVoltageProtection
        | AlarmBit::ChargingDcContactorFailure
        | AlarmBit::DischargingDcContactorFailure
        | AlarmBit::EpoShutdown
        | AlarmBit::FireProtection => 3,

        AlarmBit::ChargingOverTempAlarm
        | AlarmBit::ChargingLowTempAlarm
        | AlarmBit::DischargingOverTempPrealarm
        | AlarmBit::DischargingLowTempPrealarm
        | AlarmBit::DischargingOverCurrentPrealarm
        | AlarmBit::ChargingOverCurrentPrealarm
        | AlarmBit::TotalOverVoltagePrealarm
        | AlarmBit::TotalUnderVoltageWarning => 2,

        _ => 1,
    }
}

/// Command 0x80 - Charge/Discharge Limits
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeDischargeLimits {
    /// Charge voltage limit in V (0.1V resolution)
    pub charge_voltage_limit: f32,
    /// Charge current limit in A (0.1A resolution)
    pub charge_current_limit: f32,
    /// Discharge voltage limit in V (0.1V resolution)
    pub discharge_voltage_limit: f32,
    /// Discharge current limit in A (0.1A resolution)
    pub discharge_current_limit: f32,
}

/// Command 0x81 - SOC/SOH Data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocSohData {
    /// State of Charge in %
    pub soc: u16,
    /// State of Health in %
    pub soh: u16,
    /// Battery backup time in minutes
    pub backup_time_minutes: u16,
}

/// Command 0x82 - Voltage/Current Data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoltageCurrentData {
    /// Battery output voltage in V (0.1V resolution)
    pub voltage: f32,
    /// Battery output current in A (0.1A resolution, positive=discharge, negative=charge)
    pub current: f32,
    /// Power in kW (calculated)
    pub power: f32,
}

/// Command 0x83 - Cell Voltage Data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellVoltageData {
    /// Maximum cell voltage in V (0.001V resolution)
    pub max_voltage: f32,
    /// PACK number containing max voltage cell
    pub max_voltage_pack_no: u8,
    /// Cell number with max voltage
    pub max_voltage_cell_no: u8,
    /// Minimum cell voltage in V (0.001V resolution)
    pub min_voltage: f32,
    /// PACK number containing min voltage cell
    pub min_voltage_pack_no: u8,
    /// Cell number with min voltage
    pub min_voltage_cell_no: u8,
    /// Voltage difference between max and min
    pub voltage_delta: f32,
}

/// Command 0x84 - Temperature Data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureData {
    /// Maximum temperature in °C (0.1°C resolution)
    pub max_temperature: f32,
    /// PACK number with max temperature
    pub max_temp_pack_no: u8,
    /// Sensor number with max temperature
    pub max_temp_sensor_no: u8,
    /// Minimum temperature in °C (0.1°C resolution)
    pub min_temperature: f32,
    /// PACK number with min temperature
    pub min_temp_pack_no: u8,
    /// Sensor number with min temperature
    pub min_temp_sensor_no: u8,
    /// Temperature difference between max and min
    pub temp_delta: f32,
}

/// Command 0x85 - Operation Status Data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationStatusData {
    /// System status code
    pub system_status: SystemStatus,
    /// Work status code
    pub work_status: WorkStatus,
    /// Operation status code
    pub operation_status: OperationStatusCode,
    /// Discharge prohibited (can be allowed after OT/UT cleared)
    pub discharge_prohibited: bool,
    /// Charging prohibited
    pub charge_prohibited: bool,
    /// Discharge prohibited (cannot be allowed after OC/UV cleared)
    pub discharge_prohibited_hard: bool,
}

/// Command 0x86 - Accumulated Times
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccumulatedTimesData {
    /// Number of full charge cycles
    pub charge_times: u16,
    /// Number of full discharge cycles
    pub discharge_times: u16,
}

/// Command 0x87 - Accumulated Power
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccumulatedPowerData {
    /// Accumulated charging energy in kWh (0.1kWh resolution)
    pub charge_energy: f32,
    /// Accumulated discharging energy in kWh (0.1kWh resolution)
    pub discharge_energy: f32,
}

/// Command 0xC0 - Alarm Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlarmStatus {
    /// Raw 64-bit alarm status
    pub raw_status: u64,
    /// Active alarm indices
    pub active_alarms: Vec<u8>,
    /// Severity level (1=mild, 2=moderate, 3=severe)
    pub max_severity: u8,
}

/// Complete BMS Data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BmsData {
    /// Timestamp of last update
    pub timestamp: i64,
    /// Connection status
    pub connected: bool,
    /// Charge/discharge limits
    pub limits: Option<ChargeDischargeLimits>,
    /// SOC/SOH data
    pub soc_soh: Option<SocSohData>,
    /// Voltage/current data
    pub voltage_current: Option<VoltageCurrentData>,
    /// Cell voltage data
    pub cell_voltage: Option<CellVoltageData>,
    /// Temperature data
    pub temperature: Option<TemperatureData>,
    /// Operation status
    pub operation_status: Option<OperationStatusData>,
    /// Accumulated times
    pub accumulated_times: Option<AccumulatedTimesData>,
    /// Accumulated power
    pub accumulated_power: Option<AccumulatedPowerData>,
    /// BMS software version
    pub software_version: Option<String>,
    /// Alarm status
    pub alarm_status: Option<AlarmStatus>,
}

/// CAN Frame structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanFrame {
    /// 29-bit extended identifier
    pub id: u32,
    /// Data bytes (1-8 bytes)
    pub data: Vec<u8>,
    /// Timestamp
    pub timestamp: i64,
}

/// Parsed CAN Frame ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedFrameId {
    /// Point-to-point flag
    pub ptp: bool,
    /// Command code
    pub command: u8,
    /// Destination address
    pub destination_address: u8,
    /// Source address
    pub source_address: u8,
    /// Continuation flag
    pub cnt: bool,
}

impl ParsedFrameId {
    /// Parse a 29-bit CAN frame ID
    pub fn from_id(id: u32) -> Self {
        ParsedFrameId {
            ptp: (id >> 28) & 1 == 1,
            command: ((id >> 20) & 0xFF) as u8,
            destination_address: ((id >> 12) & 0xFF) as u8,
            source_address: ((id >> 4) & 0xFF) as u8,
            cnt: (id >> 3) & 1 == 1,
        }
    }

    /// Build a 29-bit CAN frame ID
    pub fn to_id(&self) -> u32 {
        let mut id: u32 = 0;
        if self.ptp {
            id |= 1 << 28;
        }
        id |= (self.command as u32) << 20;
        id |= (self.destination_address as u32) << 12;
        id |= (self.source_address as u32) << 4;
        if self.cnt {
            id |= 1 << 3;
        }
        id
    }
}
