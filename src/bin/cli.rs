use clap::Parser;
use device_monitor_rs::DeviceMonitor;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to store log files
    #[arg(short = 'd', long, default_value = "logs")]
    log_dir: PathBuf,

    /// Sampling interval in seconds
    #[arg(short = 'i', long, default_value_t = 1)]
    interval: u64,

    /// Log rotation interval in seconds
    #[arg(short = 'r', long, default_value_t = 3600)]
    rotation: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Starting Device Monitor CLI");
    println!("Log Directory: {:?}", args.log_dir);
    println!("Interval: {}s", args.interval);
    println!("Rotation: {}s", args.rotation);

    let mut monitor = DeviceMonitor::new(
        Duration::from_secs(args.interval),
        args.log_dir,
        Duration::from_secs(args.rotation),
    );
    monitor.start()?;

    Ok(())
}
