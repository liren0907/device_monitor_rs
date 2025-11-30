use device_monitor_rs::DeviceMonitor;
use std::path::PathBuf;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = PathBuf::from("logs");
    let interval = Duration::from_secs(1);
    let rotation_interval = Duration::from_secs(10); // Rotate every 10 seconds for testing

    let mut monitor = DeviceMonitor::new(interval, log_dir, rotation_interval);
    monitor.start()?;

    Ok(())
}
