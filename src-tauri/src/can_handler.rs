//! CAN Bus Communication Module
//! Supports USB-CAN adapters via serial port and SocketCAN on Linux

use crate::bms_types::*;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CanError {
    #[error("Serial port error: {0}")]
    SerialError(String),
    #[error("CAN frame parse error: {0}")]
    ParseError(String),
    #[error("Connection timeout")]
    Timeout,
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for CanError {
    fn from(err: std::io::Error) -> Self {
        CanError::IoError(err.to_string())
    }
}

/// CAN Adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanConfig {
    /// Adapter type
    pub adapter_type: AdapterType,
    /// Serial port path (for USB-CAN adapters)
    pub serial_port: Option<String>,
    /// Serial baud rate
    pub serial_baud_rate: u32,
    /// CAN bus baud rate (125K for BMS)
    pub can_baud_rate: u32,
    /// SocketCAN interface name (for Linux)
    pub socket_can_interface: Option<String>,
    /// BMS address
    pub bms_address: u8,
    /// Host address (PCS)
    pub host_address: u8,
}

impl Default for CanConfig {
    fn default() -> Self {
        CanConfig {
            adapter_type: AdapterType::UsbCan,
            serial_port: None,
            serial_baud_rate: 115200,
            can_baud_rate: CAN_BAUD_RATE,
            socket_can_interface: None,
            bms_address: 0x01,
            host_address: 0x80,
        }
    }
}

/// Adapter type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AdapterType {
    /// I+ Series USB-CAN adapter
    UsbCan,
    /// I+BT Bluetooth CAN adapter
    BluetoothCan,
    /// SocketCAN (Linux only)
    #[cfg(target_os = "linux")]
    SocketCan,
    /// Simulation mode (for testing)
    Simulation,
}

/// Parse I+ Series frame format
/// Frame format: 0xAA + type(1) + id(4) + len(1) + data(0-8) + checksum(1)
fn parse_iplus_frame(buffer: &[u8]) -> Option<CanFrame> {
    if buffer.len() < 8 {
        return None;
    }

    // Check header
    if buffer[0] != 0xAA {
        return None;
    }

    let frame_type = buffer[1];
    if frame_type != 0x01 {
        // Extended frame
        return None;
    }

    let id = u32::from_le_bytes([buffer[2], buffer[3], buffer[4], buffer[5]]);
    let len = buffer[6] as usize;

    if buffer.len() < 7 + len + 1 {
        return None;
    }

    let data = buffer[7..7 + len].to_vec();

    // Verify checksum
    let mut checksum: u8 = 0;
    for i in 0..7 + len {
        checksum = checksum.wrapping_add(buffer[i]);
    }
    if checksum != buffer[7 + len] {
        return None;
    }

    Some(CanFrame {
        id,
        data,
        timestamp: chrono::Utc::now().timestamp_millis(),
    })
}

/// Build I+ Series frame format
fn build_iplus_frame(frame: &CanFrame) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(16);

    buffer.push(0xAA); // Header
    buffer.push(0x01); // Extended frame type
    buffer.extend_from_slice(&frame.id.to_le_bytes());
    buffer.push(frame.data.len() as u8);
    buffer.extend_from_slice(&frame.data);

    // Calculate checksum
    let checksum: u8 = buffer.iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
    buffer.push(checksum);

    buffer
}

/// Simulation handler for testing without hardware
pub struct SimulationHandler {
    connected: bool,
    frame_counter: u32,
}

impl SimulationHandler {
    pub fn new() -> Self {
        SimulationHandler {
            connected: false,
            frame_counter: 0,
        }
    }

    pub fn connect(&mut self) -> Result<(), CanError> {
        self.connected = true;
        log::info!("Simulation mode connected");
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), CanError> {
        self.connected = false;
        log::info!("Simulation mode disconnected");
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn send_frame(&mut self, _frame: &CanFrame) -> Result<(), CanError> {
        // Simulation mode ignores sent frames
        Ok(())
    }

    pub fn receive_frame(&mut self, _timeout: Duration) -> Result<Option<CanFrame>, CanError> {
        if !self.connected {
            return Ok(None);
        }

        // Simulation mode: return test frame immediately without sleep
        std::thread::sleep(Duration::from_millis(10)); // Minimal delay
        Ok(Some(self.generate_test_frame()))
    }

    fn generate_test_frame(&mut self) -> CanFrame {
        self.frame_counter += 1;

        // Cycle through different commands
        let command = match self.frame_counter % 10 {
            0 => 0x80u8, // Limits
            1 => 0x81,   // SOC/SOH
            2 => 0x82,   // Voltage/Current
            3 => 0x83,   // Cell Voltage
            4 => 0x84,   // Temperature
            5 => 0x85,   // Operation Status
            6 => 0x86,   // Accumulated Times
            7 => 0x87,   // Accumulated Power
            8 => 0x8F,   // Version
            _ => 0xC0,   // Alarm
        };

        let frame_id = ParsedFrameId {
            ptp: true,
            command,
            destination_address: 0x80, // PCS
            source_address: 0x01,      // BMS
            cnt: false,
        };

        let data = match command {
            0x80 => vec![0x90, 0x21, 0xE8, 0x03, 0x40, 0x1A, 0xE8, 0x03], // 859.2V, 100A, 672V, 100A
            0x81 => vec![0x50, 0x00, 0x64, 0x00, 0x3C, 0x00, 0x00, 0x00], // SOC 80%, SOH 100%, 60min
            0x82 => vec![0xB9, 0x1F, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00], // 812.1V, 5.6A discharge
            0x83 => vec![0x42, 0x0D, 0x01, 0x05, 0x38, 0x0D, 0x02, 0x08], // Max 3.394V, Min 3.384V
            0x84 => vec![0x0E, 0x01, 0x01, 0x03, 0xF8, 0x00, 0x02, 0x05], // Max 27°C, Min 24.8°C
            0x85 => vec![0x04, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00], // Discharging, Boot, Normal
            0x86 => vec![0x64, 0x00, 0x62, 0x00, 0x00, 0x00, 0x00, 0x00], // 100 charge, 98 discharge
            0x87 => vec![0xE0, 0x9F, 0x02, 0x00, 0xDE, 0xC9, 0x02, 0x00], // 17200 kWh, 18275 kWh
            0x8F => vec![0x56, 0x32, 0x2E, 0x31, 0x39, 0x53, 0x00, 0x00], // V2.19S
            0xC0 => vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // No alarms
            _ => vec![0; 8],
        };

        CanFrame {
            id: frame_id.to_id(),
            data,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}

impl Default for SimulationHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// CAN Manager for handling communication
pub struct CanManager {
    simulation_handler: Option<SimulationHandler>,
    serial_port: Option<Box<dyn serialport::SerialPort + Send>>,
    config: CanConfig,
    bms_data: Arc<Mutex<BmsData>>,
    running: Arc<Mutex<bool>>,
    connected: bool,
}

impl CanManager {
    pub fn new_with_mutex(config: CanConfig, bms_data: Arc<Mutex<BmsData>>) -> Self {
        CanManager {
            simulation_handler: if config.adapter_type == AdapterType::Simulation {
                Some(SimulationHandler::new())
            } else {
                None
            },
            serial_port: None,
            config,
            bms_data,
            running: Arc::new(Mutex::new(false)),
            connected: false,
        }
    }

    pub fn connect(&mut self) -> Result<(), CanError> {
        match self.config.adapter_type {
            AdapterType::Simulation => {
                if let Some(ref mut handler) = self.simulation_handler {
                    handler.connect()?;
                    self.connected = true;
                }
            }
            AdapterType::UsbCan | AdapterType::BluetoothCan => {
                let port_name = self
                    .config
                    .serial_port
                    .as_ref()
                    .ok_or_else(|| CanError::DeviceNotFound("No serial port specified".to_string()))?;

                let port = serialport::new(port_name, self.config.serial_baud_rate)
                    .timeout(Duration::from_millis(1000))
                    .open()
                    .map_err(|e| CanError::SerialError(e.to_string()))?;

                self.serial_port = Some(port);
                self.connected = true;
                log::info!("Connected to USB-CAN adapter on {}", port_name);
            }
            #[cfg(target_os = "linux")]
            AdapterType::SocketCan => {
                // TODO: Implement SocketCAN
                self.simulation_handler = Some(SimulationHandler::new());
                if let Some(ref mut handler) = self.simulation_handler {
                    handler.connect()?;
                    self.connected = true;
                }
            }
        }
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), CanError> {
        *self.running.lock() = false;

        if let Some(ref mut handler) = self.simulation_handler {
            handler.disconnect()?;
        }

        self.serial_port = None;
        self.connected = false;
        log::info!("Disconnected");
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn get_bms_data(&self) -> BmsData {
        self.bms_data.lock().clone()
    }

    fn send_frame(&mut self, frame: &CanFrame) -> Result<(), CanError> {
        match self.config.adapter_type {
            AdapterType::Simulation => {
                if let Some(ref mut handler) = self.simulation_handler {
                    handler.send_frame(frame)?;
                }
            }
            _ => {
                if let Some(ref mut port) = self.serial_port {
                    let data = build_iplus_frame(frame);
                    port.write_all(&data)
                        .map_err(|e| CanError::SerialError(e.to_string()))?;
                }
            }
        }
        Ok(())
    }

    fn receive_frame(&mut self, timeout: Duration) -> Result<Option<CanFrame>, CanError> {
        match self.config.adapter_type {
            AdapterType::Simulation => {
                if let Some(ref mut handler) = self.simulation_handler {
                    return handler.receive_frame(timeout);
                }
            }
            _ => {
                if let Some(ref mut port) = self.serial_port {
                    port.set_timeout(timeout)
                        .map_err(|e| CanError::SerialError(e.to_string()))?;

                    let mut buffer = [0u8; 32];
                    match port.read(&mut buffer) {
                        Ok(n) if n > 0 => return Ok(parse_iplus_frame(&buffer[..n])),
                        Ok(_) => return Ok(None),
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => return Ok(None),
                        Err(e) => return Err(CanError::SerialError(e.to_string())),
                    }
                }
            }
        }
        Ok(None)
    }

    /// Query all BMS data
    pub fn query_all_data(&mut self) -> Result<(), CanError> {
        use crate::bms_parser::build_query_frame;

        let commands = [
            BmsCommand::ChargeDischargeLimits,
            BmsCommand::SocSoh,
            BmsCommand::VoltageCurrent,
            BmsCommand::CellVoltage,
            BmsCommand::Temperature,
            BmsCommand::OperationStatus,
            BmsCommand::AccumulatedTimes,
            BmsCommand::AccumulatedPower,
            BmsCommand::SoftwareVersion,
            BmsCommand::AlarmStatus,
        ];

        let is_simulation = self.config.adapter_type == AdapterType::Simulation;
        let send_delay = if is_simulation {
            Duration::from_millis(5)
        } else {
            Duration::from_millis(30) // Reduced from 50ms
        };

        for cmd in commands {
            let frame = build_query_frame(cmd, self.config.host_address, self.config.bms_address);
            self.send_frame(&frame)?;
            std::thread::sleep(send_delay);
        }

        // Receive responses with appropriate timeout
        let receive_timeout = if is_simulation {
            Duration::from_millis(10)
        } else {
            Duration::from_millis(50) // Reduced from 100ms
        };

        // Update timestamp
        {
            let mut data = self.bms_data.lock();
            data.timestamp = chrono::Utc::now().timestamp_millis();
            data.connected = true;
        }

        for _ in 0..10 {
            if let Ok(Some(frame)) = self.receive_frame(receive_timeout) {
                let mut data = self.bms_data.lock();
                crate::bms_parser::parse_can_frame(&frame, &mut data);
            }
        }

        Ok(())
    }

    /// Start continuous data reception
    pub fn start_receiving(&mut self) -> Result<(), CanError> {
        *self.running.lock() = true;

        loop {
            if !*self.running.lock() {
                break;
            }

            if let Ok(Some(frame)) = self.receive_frame(Duration::from_millis(100)) {
                let mut data = self.bms_data.lock();
                crate::bms_parser::parse_can_frame(&frame, &mut data);
            }
        }

        Ok(())
    }

    /// Get available serial ports
    pub fn list_serial_ports() -> Vec<String> {
        serialport::available_ports()
            .unwrap_or_default()
            .into_iter()
            .map(|p| p.port_name)
            .collect()
    }
}
