//! BMS Monitor - Tauri Application Entry Point

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bms_monitor_lib::commands::*;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            list_ports,
            connect,
            disconnect,
            is_connected,
            get_bms_data,
            query_all_data,
            start_receiving,
            get_alarm_descriptions,
            get_system_status_name,
            get_work_status_name,
            get_operation_status_name,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
