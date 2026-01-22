//! iTEKON USBCAN-2I Handler
//! Uses VCI (Vehicle CAN Interface) API compatible with ZLG/GCgd/iTEKON adapters
//!
//! This module requires the ControlCAN.dll or ECanVci64.dll to be present.

use crate::bms_types::*;
use std::time::Duration;

#[cfg(target_os = "windows")]
use libloading::{Library, Symbol};

/// VCI device types
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum VciDeviceType {
    /// USBCAN-I (single channel)
    UsbCan1 = 3,
    /// USBCAN-II (dual channel)
    UsbCan2 = 4,
    /// USBCAN-2I (dual channel, newer)
    UsbCan2I = 21,
}

/// CAN frame structure for VCI API
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct VciCanObj {
    pub id: u32,
    pub time_stamp: u32,
    pub time_flag: u8,
    pub send_type: u8,
    pub remote_flag: u8,
    pub extern_flag: u8,
    pub data_len: u8,
    pub data: [u8; 8],
    pub reserved: [u8; 3],
}

/// CAN init configuration
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VciInitConfig {
    pub acc_code: u32,
    pub acc_mask: u32,
    pub reserved: u32,
    pub filter: u8,
    pub timing0: u8,
    pub timing1: u8,
    pub mode: u8,
}

impl Default for VciInitConfig {
    fn default() -> Self {
        VciInitConfig {
            acc_code: 0x00000000,
            acc_mask: 0xFFFFFFFF, // Accept all
            reserved: 0,
            filter: 1, // Single filter
            // 125Kbps timing for 8MHz crystal
            timing0: 0x03,
            timing1: 0x1C,
            mode: 0, // Normal mode
        }
    }
}

/// VCI board info
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VciBoardInfo {
    pub hw_version: u16,
    pub fw_version: u16,
    pub dr_version: u16,
    pub in_version: u16,
    pub irq_num: u16,
    pub can_num: u8,
    pub str_serial_num: [u8; 20],
    pub str_hw_type: [u8; 40],
    pub reserved: [u16; 4],
}

impl Default for VciBoardInfo {
    fn default() -> Self {
        VciBoardInfo {
            hw_version: 0,
            fw_version: 0,
            dr_version: 0,
            in_version: 0,
            irq_num: 0,
            can_num: 0,
            str_serial_num: [0; 20],
            str_hw_type: [0; 40],
            reserved: [0; 4],
        }
    }
}

#[cfg(target_os = "windows")]
type VciOpenDevice = unsafe extern "stdcall" fn(u32, u32, u32) -> u32;
#[cfg(target_os = "windows")]
type VciCloseDevice = unsafe extern "stdcall" fn(u32, u32) -> u32;
#[cfg(target_os = "windows")]
type VciInitCan = unsafe extern "stdcall" fn(u32, u32, u32, *const VciInitConfig) -> u32;
#[cfg(target_os = "windows")]
type VciStartCan = unsafe extern "stdcall" fn(u32, u32, u32) -> u32;
#[cfg(target_os = "windows")]
type VciResetCan = unsafe extern "stdcall" fn(u32, u32, u32) -> u32;
#[cfg(target_os = "windows")]
type VciTransmit = unsafe extern "stdcall" fn(u32, u32, u32, *const VciCanObj, u32) -> u32;
#[cfg(target_os = "windows")]
type VciReceive = unsafe extern "stdcall" fn(u32, u32, u32, *mut VciCanObj, u32, i32) -> u32;
#[cfg(target_os = "windows")]
type VciGetReceiveNum = unsafe extern "stdcall" fn(u32, u32, u32) -> u32;
#[cfg(target_os = "windows")]
type VciReadBoardInfo = unsafe extern "stdcall" fn(u32, u32, *mut VciBoardInfo) -> u32;

/// iTEKON USBCAN Handler
#[cfg(target_os = "windows")]
pub struct ItekonHandler {
    library: Option<Library>,
    device_type: u32,
    device_index: u32,
    can_channel: u32,
    connected: bool,
}

#[cfg(target_os = "windows")]
impl ItekonHandler {
    pub fn new() -> Self {
        ItekonHandler {
            library: None,
            device_type: VciDeviceType::UsbCan2I as u32,
            device_index: 0,
            can_channel: 0,
            connected: false,
        }
    }

    pub fn set_device_type(&mut self, device_type: VciDeviceType) {
        self.device_type = device_type as u32;
    }

    pub fn set_channel(&mut self, channel: u32) {
        self.can_channel = channel;
    }

    /// Load the DLL and connect to the device
    pub fn connect(&mut self) -> Result<(), String> {
        // Build list of paths to try
        let mut dll_paths: Vec<std::path::PathBuf> = Vec::new();

        // First, try the bundled resources directory (where Tauri places it)
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Windows: resources folder next to exe
                dll_paths.push(exe_dir.join("resources").join("ControlCAN.dll"));
                // Also try directly next to exe
                dll_paths.push(exe_dir.join("ControlCAN.dll"));
            }
        }

        // Try current working directory
        dll_paths.push(std::path::PathBuf::from("ControlCAN.dll"));
        dll_paths.push(std::path::PathBuf::from("resources/ControlCAN.dll"));

        // Alternative DLL names (system paths)
        let system_dlls = [
            "ControlCAN.dll",
            "ECanVci64.dll",
            "ECANVCI.dll",
            "USBCAN.dll",
        ];

        let mut lib = None;

        // Try bundled paths first
        for path in &dll_paths {
            if path.exists() {
                match unsafe { Library::new(path) } {
                    Ok(l) => {
                        log::info!("Loaded CAN library from: {:?}", path);
                        lib = Some(l);
                        break;
                    }
                    Err(e) => {
                        log::debug!("Failed to load {:?}: {}", path, e);
                    }
                }
            }
        }

        // Fall back to system paths
        if lib.is_none() {
            for name in &system_dlls {
                match unsafe { Library::new(name) } {
                    Ok(l) => {
                        log::info!("Loaded CAN library: {}", name);
                        lib = Some(l);
                        break;
                    }
                    Err(e) => {
                        log::debug!("Failed to load {}: {}", name, e);
                    }
                }
            }
        }

        let library = lib.ok_or_else(|| {
            "Failed to load CAN DLL. Please install the iTEKON driver and ensure ControlCAN.dll is in PATH".to_string()
        })?;

        // Open device
        let open_device: Symbol<VciOpenDevice> = unsafe {
            library
                .get(b"VCI_OpenDevice")
                .map_err(|e| format!("VCI_OpenDevice not found: {}", e))?
        };

        let result = unsafe { open_device(self.device_type, self.device_index, 0) };
        if result != 1 {
            return Err(format!(
                "VCI_OpenDevice failed. Device type: {}, Index: {}. Error code: {}",
                self.device_type, self.device_index, result
            ));
        }

        // Initialize CAN
        let init_can: Symbol<VciInitCan> = unsafe {
            library
                .get(b"VCI_InitCAN")
                .map_err(|e| format!("VCI_InitCAN not found: {}", e))?
        };

        let config = VciInitConfig::default();
        let result = unsafe {
            init_can(
                self.device_type,
                self.device_index,
                self.can_channel,
                &config,
            )
        };
        if result != 1 {
            return Err(format!("VCI_InitCAN failed. Error code: {}", result));
        }

        // Start CAN
        let start_can: Symbol<VciStartCan> = unsafe {
            library
                .get(b"VCI_StartCAN")
                .map_err(|e| format!("VCI_StartCAN not found: {}", e))?
        };

        let result = unsafe { start_can(self.device_type, self.device_index, self.can_channel) };
        if result != 1 {
            return Err(format!("VCI_StartCAN failed. Error code: {}", result));
        }

        self.library = Some(library);
        self.connected = true;
        log::info!("iTEKON USBCAN connected successfully");
        Ok(())
    }

    /// Disconnect from the device
    pub fn disconnect(&mut self) -> Result<(), String> {
        if let Some(ref library) = self.library {
            let close_device: Symbol<VciCloseDevice> = unsafe {
                library
                    .get(b"VCI_CloseDevice")
                    .map_err(|e| format!("VCI_CloseDevice not found: {}", e))?
            };

            unsafe { close_device(self.device_type, self.device_index) };
        }

        self.library = None;
        self.connected = false;
        log::info!("iTEKON USBCAN disconnected");
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Send a CAN frame
    pub fn send_frame(&self, frame: &CanFrame) -> Result<(), String> {
        let library = self
            .library
            .as_ref()
            .ok_or_else(|| "Not connected".to_string())?;

        let transmit: Symbol<VciTransmit> = unsafe {
            library
                .get(b"VCI_Transmit")
                .map_err(|e| format!("VCI_Transmit not found: {}", e))?
        };

        let mut can_obj = VciCanObj::default();
        can_obj.id = frame.id;
        can_obj.extern_flag = 1; // Extended frame (29-bit)
        can_obj.data_len = frame.data.len() as u8;
        for (i, &byte) in frame.data.iter().enumerate() {
            if i < 8 {
                can_obj.data[i] = byte;
            }
        }

        let result = unsafe {
            transmit(
                self.device_type,
                self.device_index,
                self.can_channel,
                &can_obj,
                1,
            )
        };

        if result != 1 {
            return Err(format!("VCI_Transmit failed. Error code: {}", result));
        }

        Ok(())
    }

    /// Receive CAN frames
    pub fn receive_frame(&self, timeout: Duration) -> Result<Option<CanFrame>, String> {
        let library = self
            .library
            .as_ref()
            .ok_or_else(|| "Not connected".to_string())?;

        // Check if data available
        let get_receive_num: Symbol<VciGetReceiveNum> = unsafe {
            library
                .get(b"VCI_GetReceiveNum")
                .map_err(|e| format!("VCI_GetReceiveNum not found: {}", e))?
        };

        let count =
            unsafe { get_receive_num(self.device_type, self.device_index, self.can_channel) };

        if count == 0 {
            // Wait a bit and try again
            std::thread::sleep(timeout);
            let count =
                unsafe { get_receive_num(self.device_type, self.device_index, self.can_channel) };
            if count == 0 {
                return Ok(None);
            }
        }

        // Receive frame
        let receive: Symbol<VciReceive> = unsafe {
            library
                .get(b"VCI_Receive")
                .map_err(|e| format!("VCI_Receive not found: {}", e))?
        };

        let mut can_obj = VciCanObj::default();
        let wait_time = timeout.as_millis() as i32;

        let result = unsafe {
            receive(
                self.device_type,
                self.device_index,
                self.can_channel,
                &mut can_obj,
                1,
                wait_time,
            )
        };

        if result == 0 {
            return Ok(None);
        }

        let data = can_obj.data[..can_obj.data_len as usize].to_vec();

        Ok(Some(CanFrame {
            id: can_obj.id,
            data,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }))
    }

    /// Get device info
    pub fn get_board_info(&self) -> Result<VciBoardInfo, String> {
        let library = self
            .library
            .as_ref()
            .ok_or_else(|| "Not connected".to_string())?;

        let read_board_info: Symbol<VciReadBoardInfo> = unsafe {
            library
                .get(b"VCI_ReadBoardInfo")
                .map_err(|e| format!("VCI_ReadBoardInfo not found: {}", e))?
        };

        let mut info = VciBoardInfo::default();
        let result =
            unsafe { read_board_info(self.device_type, self.device_index, &mut info) };

        if result != 1 {
            return Err(format!("VCI_ReadBoardInfo failed. Error code: {}", result));
        }

        Ok(info)
    }
}

#[cfg(target_os = "windows")]
impl Default for ItekonHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "windows")]
impl Drop for ItekonHandler {
    fn drop(&mut self) {
        if self.connected {
            let _ = self.disconnect();
        }
    }
}

// Stub for non-Windows platforms
#[cfg(not(target_os = "windows"))]
pub struct ItekonHandler;

#[cfg(not(target_os = "windows"))]
impl ItekonHandler {
    pub fn new() -> Self {
        ItekonHandler
    }

    pub fn connect(&mut self) -> Result<(), String> {
        Err("iTEKON USBCAN is only supported on Windows".to_string())
    }

    pub fn disconnect(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        false
    }

    pub fn send_frame(&self, _frame: &CanFrame) -> Result<(), String> {
        Err("iTEKON USBCAN is only supported on Windows".to_string())
    }

    pub fn receive_frame(&self, _timeout: Duration) -> Result<Option<CanFrame>, String> {
        Err("iTEKON USBCAN is only supported on Windows".to_string())
    }
}

#[cfg(not(target_os = "windows"))]
impl Default for ItekonHandler {
    fn default() -> Self {
        Self::new()
    }
}
