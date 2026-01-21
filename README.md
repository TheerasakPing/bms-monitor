# BMS Battery Monitor

A cross-platform desktop application for monitoring Battery Management Systems (BMS) via CAN bus communication. Built with Tauri V2 and Svelte 5.

![License](https://img.shields.io/github/license/TheerasakPing/bms-monitor)
![Release](https://img.shields.io/github/v/release/TheerasakPing/bms-monitor)
![Build](https://img.shields.io/github/actions/workflow/status/TheerasakPing/bms-monitor/ci.yml)

## Features

- **Real-time Dashboard**
  - SOC (State of Charge) and SOH (State of Health) gauges
  - Battery voltage, current, and power display
  - Cell voltage monitoring (max/min with cell identification)
  - Temperature monitoring (max/min with sensor location)
  - Operation status panel

- **Alarm Monitoring**
  - 41 defined alarm types with severity levels (1-3)
  - Real-time alarm notifications
  - Alarm history tracking

- **Debug Log**
  - Record all CAN communication
  - Parse and display frame data
  - Filter by direction (TX/RX) and command type
  - Export to CSV or JSON

- **Multiple Connection Modes**
  - USB-CAN adapter (I+ Series compatible)
  - Simulation mode for testing

## Supported BMS Protocol

Ecube BMS-PCS Communication Protocol V1.20
- CAN 2.0B (29-bit extended identifier)
- Baud rate: 125 Kbps
- Data transmission: Low byte first

### Supported Commands

| Command | Description |
|---------|-------------|
| 0x80 | Charge/discharge voltage and current limits |
| 0x81 | SOC, SOH, backup time |
| 0x82 | Battery output voltage and current |
| 0x83 | Cell voltage max/min with PACK/cell numbers |
| 0x84 | Temperature max/min with PACK/sensor numbers |
| 0x85 | Operation status |
| 0x86 | Accumulated charge/discharge times |
| 0x87 | Accumulated charge/discharge power (kWh) |
| 0x8F | BMS software version |
| 0xC0 | Alarm status (64-bit) |
| 0xD0 | Debug status code |

## Installation

### Download

Download the latest release for your platform:

- **Windows**: `.msi` or `.exe` installer
- **macOS**: `.dmg` (Intel & Apple Silicon)
- **Linux**: `.deb`, `.rpm`, or `.AppImage`

[Download Latest Release](https://github.com/TheerasakPing/bms-monitor/releases/latest)

### Build from Source

#### Prerequisites

- [Node.js](https://nodejs.org/) 20+
- [Rust](https://www.rust-lang.org/tools/install) 1.70+
- Platform-specific dependencies:

**Ubuntu/Debian:**
```bash
sudo apt-get install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libudev-dev
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- Visual Studio Build Tools with C++ workload

#### Build Steps

```bash
# Clone repository
git clone https://github.com/TheerasakPing/bms-monitor.git
cd bms-monitor

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Usage

### Quick Start

1. Launch the application
2. Select connection type:
   - **Simulation**: Test without hardware
   - **USB-CAN**: Connect to real BMS via I+ Series adapter
3. Click **Connect**
4. View real-time BMS data on the Dashboard

### Debug Log

1. Click the **Debug Log** tab
2. Click **Start Recording**
3. CAN frames will be logged in real-time
4. Use filters to find specific commands
5. Click **Download CSV** or **Download JSON** to export

## Hardware Requirements

### USB-CAN Adapter

Tested with I+ Series USB-CAN adapters:
- I+ USB-CAN
- I+BT Bluetooth CAN (experimental)

Frame format: `0xAA + type(1) + id(4) + len(1) + data(0-8) + checksum(1)`

### Linux Serial Port Permissions

Add your user to the `dialout` group:
```bash
sudo usermod -a -G dialout $USER
# Log out and log back in
```

## Development

### Project Structure

```
bms-monitor/
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── components/     # UI components
│   │   ├── stores/         # State management
│   │   └── types/          # TypeScript types
│   └── App.svelte          # Main application
├── src-tauri/              # Rust backend
│   └── src/
│       ├── bms_types.rs    # BMS data structures
│       ├── bms_parser.rs   # Protocol parsing
│       ├── can_handler.rs  # CAN communication
│       └── commands.rs     # Tauri commands
└── .github/workflows/      # CI/CD
```

### Tech Stack

- **Frontend**: Svelte 5, TypeScript, Vite
- **Backend**: Rust, Tauri V2
- **CAN**: serialport, custom I+ protocol parser

## License

MIT License - see [LICENSE](LICENSE) file

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

## Acknowledgments

- [Tauri](https://tauri.app/) - Cross-platform app framework
- [Svelte](https://svelte.dev/) - Frontend framework
- [serialport](https://crates.io/crates/serialport) - Rust serial port library
