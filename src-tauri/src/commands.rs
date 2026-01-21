//! Tauri Commands for BMS Monitor

use crate::bms_types::*;
use crate::can_handler::{AdapterType, CanConfig, CanManager};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

/// Application state
pub struct AppState {
    pub bms_data: Arc<Mutex<BmsData>>,
    pub can_manager: Arc<Mutex<Option<CanManager>>>,
    pub config: Arc<Mutex<CanConfig>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            bms_data: Arc::new(Mutex::new(BmsData::default())),
            can_manager: Arc::new(Mutex::new(None)),
            config: Arc::new(Mutex::new(CanConfig::default())),
        }
    }
}

/// Connection configuration from frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub adapter_type: String,
    pub serial_port: Option<String>,
    pub serial_baud_rate: Option<u32>,
    pub bms_address: Option<u8>,
}

/// Command result type
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> CommandResult<T> {
    pub fn ok(data: T) -> Self {
        CommandResult {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        CommandResult {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Get list of available serial ports
#[tauri::command]
pub fn list_ports() -> CommandResult<Vec<String>> {
    let ports = CanManager::list_serial_ports();
    CommandResult::ok(ports)
}

/// Connect to BMS via CAN adapter
#[tauri::command]
pub fn connect(config: ConnectionConfig, state: State<'_, AppState>) -> CommandResult<bool> {
    let adapter_type = match config.adapter_type.as_str() {
        "usb" => AdapterType::UsbCan,
        "bluetooth" => AdapterType::BluetoothCan,
        "simulation" => AdapterType::Simulation,
        _ => AdapterType::UsbCan,
    };

    let can_config = CanConfig {
        adapter_type,
        serial_port: config.serial_port,
        serial_baud_rate: config.serial_baud_rate.unwrap_or(115200),
        can_baud_rate: CAN_BAUD_RATE,
        socket_can_interface: None,
        bms_address: config.bms_address.unwrap_or(0x01),
        host_address: 0x80,
    };

    let bms_data = state.inner().bms_data.clone();
    let mut manager = CanManager::new_with_mutex(can_config.clone(), bms_data);

    match manager.connect() {
        Ok(_) => {
            *state.inner().can_manager.lock() = Some(manager);
            *state.inner().config.lock() = can_config;
            CommandResult::ok(true)
        }
        Err(e) => CommandResult::err(format!("Connection failed: {}", e)),
    }
}

/// Disconnect from BMS
#[tauri::command]
pub fn disconnect(state: State<'_, AppState>) -> CommandResult<bool> {
    if let Some(mut manager) = state.inner().can_manager.lock().take() {
        match manager.disconnect() {
            Ok(_) => CommandResult::ok(true),
            Err(e) => CommandResult::err(format!("Disconnect failed: {}", e)),
        }
    } else {
        CommandResult::ok(true)
    }
}

/// Check connection status
#[tauri::command]
pub fn is_connected(state: State<'_, AppState>) -> bool {
    state
        .inner()
        .can_manager
        .lock()
        .as_ref()
        .is_some_and(|m| m.is_connected())
}

/// Get current BMS data
#[tauri::command]
pub fn get_bms_data(state: State<'_, AppState>) -> BmsData {
    state.inner().bms_data.lock().clone()
}

/// Query all BMS data (async to prevent blocking UI)
#[tauri::command]
pub async fn query_all_data(state: State<'_, AppState>) -> Result<CommandResult<bool>, ()> {
    let can_manager = state.inner().can_manager.clone();
    let bms_data = state.inner().bms_data.clone();
    let config = state.inner().config.lock().clone();

    // Run blocking operations in a separate thread
    let result = tauri::async_runtime::spawn_blocking(move || {
        let mut guard = can_manager.lock();
        if let Some(ref mut manager) = *guard {
            manager.query_all_data()
        } else {
            // If no manager, create temporary one for simulation
            drop(guard); // Release lock before creating new manager
            let mut temp_manager = CanManager::new_with_mutex(config, bms_data);
            if temp_manager.connect().is_ok() {
                let result = temp_manager.query_all_data();
                // Store the manager for future use
                *can_manager.lock() = Some(temp_manager);
                result
            } else {
                Err(crate::can_handler::CanError::DeviceNotFound(
                    "Not connected".to_string(),
                ))
            }
        }
    })
    .await;

    match result {
        Ok(Ok(_)) => Ok(CommandResult::ok(true)),
        Ok(Err(e)) => Ok(CommandResult::err(format!("Query failed: {}", e))),
        Err(e) => Ok(CommandResult::err(format!("Task failed: {}", e))),
    }
}

/// Start continuous data reception (async)
#[tauri::command]
pub async fn start_receiving(state: State<'_, AppState>) -> Result<CommandResult<bool>, ()> {
    let bms_data = state.inner().bms_data.clone();
    let config = state.inner().config.lock().clone();

    tokio::spawn(async move {
        let mut manager = CanManager::new_with_mutex(config, bms_data);
        if manager.connect().is_ok() {
            let _ = manager.start_receiving();
        }
    });

    Ok(CommandResult::ok(true))
}

/// Get alarm descriptions
#[tauri::command]
pub fn get_alarm_descriptions() -> Vec<(u8, String, u8)> {
    vec![
        (0, "Cell over voltage".to_string(), 3),
        (1, "Cell under voltage".to_string(), 3),
        (2, "Charging over temperature alarm".to_string(), 2),
        (3, "Charging low temperature alarm".to_string(), 2),
        (4, "Discharging over temperature pre-alarm".to_string(), 2),
        (5, "Discharging low temperature pre-alarm".to_string(), 2),
        (6, "Discharging over current pre-alarm".to_string(), 2),
        (7, "Charging over current pre-alarm".to_string(), 2),
        (8, "Total over voltage pre-alarm".to_string(), 2),
        (9, "Total under voltage warning".to_string(), 2),
        (10, "Circuit breaker disconnected".to_string(), 1),
        (11, "Balanced charging failed".to_string(), 1),
        (12, "Positive battery pack voltage imbalance".to_string(), 1),
        (13, "Negative battery pack voltage imbalance".to_string(), 1),
        (14, "BMU communication interruption".to_string(), 3),
        (15, "Water flooding detection alarm".to_string(), 1),
        (16, "Water flooding detection and protection".to_string(), 1),
        (18, "Charging over temperature protection".to_string(), 3),
        (19, "Charging low temperature protection".to_string(), 3),
        (20, "Discharging over temperature protection".to_string(), 3),
        (21, "Discharging low temperature protection".to_string(), 3),
        (
            22,
            "Discharging over current protection level 1".to_string(),
            3,
        ),
        (
            23,
            "Discharging over current protection level 2".to_string(),
            3,
        ),
        (
            24,
            "Charging over current protection level 1".to_string(),
            3,
        ),
        (
            25,
            "Charging over current protection level 2".to_string(),
            3,
        ),
        (
            26,
            "Charging over current protection level 3".to_string(),
            3,
        ),
        (27, "Total charging over voltage protection".to_string(), 3),
        (28, "Total charging under voltage protection".to_string(), 3),
        (29, "Charging DC contactor failure".to_string(), 3),
        (30, "Discharging DC contactor failure".to_string(), 3),
        (31, "EPO shut down".to_string(), 3),
        (32, "Fire protection".to_string(), 3),
        (33, "Parallel communication abnormality".to_string(), 1),
        (34, "Parallel address conflict".to_string(), 1),
        (35, "Insulation monitoring alarm".to_string(), 1),
        (36, "Hydrogen protection".to_string(), 1),
        (37, "Battery pack fan malfunction".to_string(), 1),
        (38, "Battery pack fuse temperature too high".to_string(), 1),
        (39, "CAN Hall communication interruption".to_string(), 1),
        (40, "CAN Hall data failure".to_string(), 1),
    ]
}

/// Get system status description
#[tauri::command]
pub fn get_system_status_name(status: u8) -> String {
    match status {
        0 => "Power On".to_string(),
        1 => "Start".to_string(),
        2 => "Alone".to_string(),
        3 => "Charging".to_string(),
        4 => "Discharging".to_string(),
        5 => "Wait to Charge".to_string(),
        6 => "Wait to Discharge".to_string(),
        7 => "Lock".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Get work status description
#[tauri::command]
pub fn get_work_status_name(status: u8) -> String {
    match status {
        0 => "Empty".to_string(),
        1 => "Boot".to_string(),
        2 => "Shut Down".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Get operation status description
#[tauri::command]
pub fn get_operation_status_name(status: u8) -> String {
    match status {
        0 => "Empty".to_string(),
        1 => "Normal".to_string(),
        2 => "Alarm".to_string(),
        3 => "Fault".to_string(),
        _ => "Unknown".to_string(),
    }
}
