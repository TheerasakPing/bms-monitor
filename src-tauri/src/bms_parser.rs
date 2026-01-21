//! BMS Protocol Parser
//! Parses CAN frames according to Ecube BMS-PCS Communication Protocol V1.20

use crate::bms_types::*;

/// Parse Command 0x80 - Charge/Discharge Limits
pub fn parse_charge_discharge_limits(data: &[u8]) -> Option<ChargeDischargeLimits> {
    if data.len() < 8 {
        return None;
    }

    Some(ChargeDischargeLimits {
        charge_voltage_limit: u16::from_le_bytes([data[0], data[1]]) as f32 * 0.1,
        charge_current_limit: u16::from_le_bytes([data[2], data[3]]) as f32 * 0.1,
        discharge_voltage_limit: u16::from_le_bytes([data[4], data[5]]) as f32 * 0.1,
        discharge_current_limit: u16::from_le_bytes([data[6], data[7]]) as f32 * 0.1,
    })
}

/// Parse Command 0x81 - SOC/SOH
pub fn parse_soc_soh(data: &[u8]) -> Option<SocSohData> {
    if data.len() < 6 {
        return None;
    }

    Some(SocSohData {
        soc: u16::from_le_bytes([data[0], data[1]]),
        soh: u16::from_le_bytes([data[2], data[3]]),
        backup_time_minutes: u16::from_le_bytes([data[4], data[5]]),
    })
}

/// Parse Command 0x82 - Voltage/Current
pub fn parse_voltage_current(data: &[u8]) -> Option<VoltageCurrentData> {
    if data.len() < 4 {
        return None;
    }

    let voltage = u16::from_le_bytes([data[0], data[1]]) as f32 * 0.1;
    let current = i16::from_le_bytes([data[2], data[3]]) as f32 * 0.1;
    let power = voltage * current.abs() / 1000.0; // Convert to kW

    Some(VoltageCurrentData {
        voltage,
        current,
        power,
    })
}

/// Parse Command 0x83 - Cell Voltage
pub fn parse_cell_voltage(data: &[u8]) -> Option<CellVoltageData> {
    if data.len() < 8 {
        return None;
    }

    let max_voltage = u16::from_le_bytes([data[0], data[1]]) as f32 * 0.001;
    let min_voltage = u16::from_le_bytes([data[4], data[5]]) as f32 * 0.001;

    Some(CellVoltageData {
        max_voltage,
        max_voltage_pack_no: data[2],
        max_voltage_cell_no: data[3],
        min_voltage,
        min_voltage_pack_no: data[6],
        min_voltage_cell_no: data[7],
        voltage_delta: max_voltage - min_voltage,
    })
}

/// Parse Command 0x84 - Temperature
pub fn parse_temperature(data: &[u8]) -> Option<TemperatureData> {
    if data.len() < 8 {
        return None;
    }

    let max_temperature = i16::from_le_bytes([data[0], data[1]]) as f32 * 0.1;
    let min_temperature = i16::from_le_bytes([data[4], data[5]]) as f32 * 0.1;

    Some(TemperatureData {
        max_temperature,
        max_temp_pack_no: data[2],
        max_temp_sensor_no: data[3],
        min_temperature,
        min_temp_pack_no: data[6],
        min_temp_sensor_no: data[7],
        temp_delta: max_temperature - min_temperature,
    })
}

/// Parse Command 0x85 - Operation Status
pub fn parse_operation_status(data: &[u8]) -> Option<OperationStatusData> {
    if data.len() < 4 {
        return None;
    }

    let prohibition_flags = data[3];

    Some(OperationStatusData {
        system_status: SystemStatus::from(data[0]),
        work_status: WorkStatus::from(data[1]),
        operation_status: OperationStatusCode::from(data[2]),
        discharge_prohibited: (prohibition_flags & 0x01) != 0,
        charge_prohibited: (prohibition_flags & 0x02) != 0,
        discharge_prohibited_hard: (prohibition_flags & 0x04) != 0,
    })
}

/// Parse Command 0x86 - Accumulated Times
pub fn parse_accumulated_times(data: &[u8]) -> Option<AccumulatedTimesData> {
    if data.len() < 4 {
        return None;
    }

    Some(AccumulatedTimesData {
        charge_times: u16::from_le_bytes([data[0], data[1]]),
        discharge_times: u16::from_le_bytes([data[2], data[3]]),
    })
}

/// Parse Command 0x87 - Accumulated Power
pub fn parse_accumulated_power(data: &[u8]) -> Option<AccumulatedPowerData> {
    if data.len() < 8 {
        return None;
    }

    Some(AccumulatedPowerData {
        charge_energy: u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as f32 * 0.1,
        discharge_energy: u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as f32 * 0.1,
    })
}

/// Parse Command 0x8F - Software Version
pub fn parse_software_version(data: &[u8]) -> Option<String> {
    let version: String = data
        .iter()
        .take(8)
        .filter(|&&b| b != 0)
        .map(|&b| b as char)
        .collect();

    if version.is_empty() {
        None
    } else {
        Some(version)
    }
}

/// Parse Command 0xC0 - Alarm Status
pub fn parse_alarm_status(data: &[u8]) -> Option<AlarmStatus> {
    if data.len() < 8 {
        return None;
    }

    let raw_status = u64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);

    let mut active_alarms = Vec::new();
    let mut max_severity = 0u8;

    for bit in 0..64 {
        if (raw_status >> bit) & 1 == 1 {
            active_alarms.push(bit as u8);

            // Get severity for known alarm bits
            if bit <= 40 {
                if let Some(alarm) = match bit {
                    0 => Some(AlarmBit::CellOverVoltage),
                    1 => Some(AlarmBit::CellUnderVoltage),
                    2 => Some(AlarmBit::ChargingOverTempAlarm),
                    3 => Some(AlarmBit::ChargingLowTempAlarm),
                    4 => Some(AlarmBit::DischargingOverTempPrealarm),
                    5 => Some(AlarmBit::DischargingLowTempPrealarm),
                    6 => Some(AlarmBit::DischargingOverCurrentPrealarm),
                    7 => Some(AlarmBit::ChargingOverCurrentPrealarm),
                    8 => Some(AlarmBit::TotalOverVoltagePrealarm),
                    9 => Some(AlarmBit::TotalUnderVoltageWarning),
                    14 => Some(AlarmBit::BmuCommunicationInterruption),
                    18 => Some(AlarmBit::ChargingOverTempProtection),
                    19 => Some(AlarmBit::ChargingLowTempProtection),
                    20 => Some(AlarmBit::DischargingOverTempProtection),
                    21 => Some(AlarmBit::DischargingLowTempProtection),
                    22 => Some(AlarmBit::DischargingOverCurrentProtectionL1),
                    23 => Some(AlarmBit::DischargingOverCurrentProtectionL2),
                    24 => Some(AlarmBit::ChargingOverCurrentProtectionL1),
                    25 => Some(AlarmBit::ChargingOverCurrentProtectionL2),
                    26 => Some(AlarmBit::ChargingOverCurrentProtectionL3),
                    27 => Some(AlarmBit::TotalChargingOverVoltageProtection),
                    28 => Some(AlarmBit::TotalChargingUnderVoltageProtection),
                    29 => Some(AlarmBit::ChargingDcContactorFailure),
                    30 => Some(AlarmBit::DischargingDcContactorFailure),
                    31 => Some(AlarmBit::EpoShutdown),
                    32 => Some(AlarmBit::FireProtection),
                    _ => None,
                } {
                    let severity = get_alarm_severity(alarm);
                    if severity > max_severity {
                        max_severity = severity;
                    }
                }
            }
        }
    }

    Some(AlarmStatus {
        raw_status,
        active_alarms,
        max_severity,
    })
}

/// Build a query frame for a specific command
pub fn build_query_frame(
    command: BmsCommand,
    source_address: u8,
    destination_address: u8,
) -> CanFrame {
    let frame_id = ParsedFrameId {
        ptp: true,
        command: command as u8,
        destination_address,
        source_address,
        cnt: false,
    };

    CanFrame {
        id: frame_id.to_id(),
        data: vec![0; 8], // Query frames have empty data
        timestamp: chrono::Utc::now().timestamp_millis(),
    }
}

/// Parse a CAN frame and update BMS data
pub fn parse_can_frame(frame: &CanFrame, bms_data: &mut BmsData) {
    let parsed_id = ParsedFrameId::from_id(frame.id);

    if let Ok(command) = BmsCommand::try_from(parsed_id.command) {
        match command {
            BmsCommand::ChargeDischargeLimits => {
                if let Some(limits) = parse_charge_discharge_limits(&frame.data) {
                    bms_data.limits = Some(limits);
                }
            }
            BmsCommand::SocSoh => {
                if let Some(soc_soh) = parse_soc_soh(&frame.data) {
                    bms_data.soc_soh = Some(soc_soh);
                }
            }
            BmsCommand::VoltageCurrent => {
                if let Some(vc) = parse_voltage_current(&frame.data) {
                    bms_data.voltage_current = Some(vc);
                }
            }
            BmsCommand::CellVoltage => {
                if let Some(cv) = parse_cell_voltage(&frame.data) {
                    bms_data.cell_voltage = Some(cv);
                }
            }
            BmsCommand::Temperature => {
                if let Some(temp) = parse_temperature(&frame.data) {
                    bms_data.temperature = Some(temp);
                }
            }
            BmsCommand::OperationStatus => {
                if let Some(status) = parse_operation_status(&frame.data) {
                    bms_data.operation_status = Some(status);
                }
            }
            BmsCommand::AccumulatedTimes => {
                if let Some(times) = parse_accumulated_times(&frame.data) {
                    bms_data.accumulated_times = Some(times);
                }
            }
            BmsCommand::AccumulatedPower => {
                if let Some(power) = parse_accumulated_power(&frame.data) {
                    bms_data.accumulated_power = Some(power);
                }
            }
            BmsCommand::SoftwareVersion => {
                if let Some(version) = parse_software_version(&frame.data) {
                    bms_data.software_version = Some(version);
                }
            }
            BmsCommand::AlarmStatus => {
                if let Some(alarm) = parse_alarm_status(&frame.data) {
                    bms_data.alarm_status = Some(alarm);
                }
            }
            _ => {}
        }

        bms_data.timestamp = chrono::Utc::now().timestamp_millis();
        bms_data.connected = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_charge_discharge_limits() {
        // Example from protocol: 90 21 E8 03 40 1A E8 03
        // Charging voltage limit: 859.2V
        // Charging current limit: 100A
        // Discharge voltage limit: 672V
        // Discharge current limit: 100A
        let data = [0x90, 0x21, 0xE8, 0x03, 0x40, 0x1A, 0xE8, 0x03];
        let result = parse_charge_discharge_limits(&data).unwrap();

        assert!((result.charge_voltage_limit - 859.2).abs() < 0.1);
        assert!((result.charge_current_limit - 100.0).abs() < 0.1);
        assert!((result.discharge_voltage_limit - 672.0).abs() < 0.1);
        assert!((result.discharge_current_limit - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_parse_soc_soh() {
        // Example: SOC: 34%, SOH: 100%, Backup time: 30 minutes
        let data = [0x22, 0x00, 0x64, 0x00, 0x1E, 0x00, 0x00, 0x00];
        let result = parse_soc_soh(&data).unwrap();

        assert_eq!(result.soc, 34);
        assert_eq!(result.soh, 100);
        assert_eq!(result.backup_time_minutes, 30);
    }

    #[test]
    fn test_parse_voltage_current() {
        // Example: 812.1V, -120A (charging)
        let data = [0xB9, 0x1F, 0x50, 0xFB, 0x00, 0x00, 0x00, 0x00];
        let result = parse_voltage_current(&data).unwrap();

        assert!((result.voltage - 812.1).abs() < 0.1);
        assert!((result.current - (-120.0)).abs() < 0.1);
    }

    #[test]
    fn test_parse_cell_voltage() {
        // Example: Max 3.394V (PACK 8, Cell 5), Min 3.372V (PACK 11, Cell 2)
        let data = [0x42, 0x0D, 0x08, 0x05, 0x2C, 0x0D, 0x0B, 0x02];
        let result = parse_cell_voltage(&data).unwrap();

        assert!((result.max_voltage - 3.394).abs() < 0.001);
        assert_eq!(result.max_voltage_pack_no, 8);
        assert_eq!(result.max_voltage_cell_no, 5);
        assert!((result.min_voltage - 3.372).abs() < 0.001);
        assert_eq!(result.min_voltage_pack_no, 11);
        assert_eq!(result.min_voltage_cell_no, 2);
    }

    #[test]
    fn test_parse_frame_id() {
        // Example: Frame header: 18080010
        let id = 0x18080010u32;
        let parsed = ParsedFrameId::from_id(id);

        assert!(parsed.ptp);
        assert_eq!(parsed.command, 0x80);
        assert_eq!(parsed.destination_address, 0x80);
        assert_eq!(parsed.source_address, 0x01);
        assert!(!parsed.cnt);
    }
}
