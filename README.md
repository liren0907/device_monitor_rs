# Device Monitor RS

A Rust library and tool for monitoring system performance metrics and logging them to a CSV file.

## Features

- **CPU Usage**: Logs global CPU usage and **per-core** usage.
- **Memory Usage**: Logs used and total memory in MB.
- **CSV Output**: Structured data format.
- **Log Rotation**: Automatically rotates log files based on time duration.
- **CLI**: Configurable via command-line arguments.

## Usage

### Running the CLI

You can run the CLI tool using `cargo run --bin cli`.

```bash
# Run with default settings (logs to ./logs, 1s interval, 1h rotation)
cargo run --bin cli

# Run with custom settings
cargo run --bin cli -- --log-dir ./my_logs --interval 2 --rotation 60
```

### Arguments

- `-d, --log-dir <PATH>`: Directory to store log files (default: `logs`).
- `-i, --interval <SECONDS>`: Sampling interval in seconds (default: `1`).
- `-r, --rotation <SECONDS>`: Log rotation interval in seconds (default: `3600`).

### Output Format

The output is a CSV file with the following columns:
- `timestamp`: ISO 8601 timestamp.
- `memory_used_mb`: Used memory in Megabytes.
- `memory_total_mb`: Total system memory in Megabytes.
- `cpu_global_usage`: Overall CPU usage percentage.
- `cpu_core_N`: Usage percentage for each specific CPU core.

## Library Usage

You can also use this as a library in your own Rust projects.

```rust
use device_monitor_rs::DeviceMonitor;
use std::path::PathBuf;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = PathBuf::from("logs");
    let interval = Duration::from_secs(1);
    let rotation = Duration::from_secs(3600);
    
    let mut monitor = DeviceMonitor::new(interval, log_dir, rotation);
    monitor.start()?;
    
    Ok(())
}
```
