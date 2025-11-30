use chrono::Local;
use csv::Writer;
use serde::ser::{Serialize, Serializer};
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::System;

pub struct DeviceMonitor {
    system: System,
    interval: Duration,
    log_dir: PathBuf,
    rotation_interval: Duration,
}

struct SystemMetrics {
    timestamp: String,
    memory_used_mb: u64,
    memory_total_mb: u64,
    cpu_global_usage: f32,
    cpus: Vec<f32>,
}

impl Serialize for SystemMetrics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        // 4 fixed fields + N core fields
        let mut seq = serializer.serialize_seq(Some(4 + self.cpus.len()))?;

        seq.serialize_element(&self.timestamp)?;
        seq.serialize_element(&self.memory_used_mb)?;
        seq.serialize_element(&self.memory_total_mb)?;
        seq.serialize_element(&self.cpu_global_usage)?;

        // Dynamically add a field for each core
        for usage in &self.cpus {
            seq.serialize_element(usage)?;
        }

        seq.end()
    }
}

impl DeviceMonitor {
    pub fn new(interval: Duration, log_dir: PathBuf, rotation_interval: Duration) -> Self {
        let mut system = System::new_all();
        // First refresh to init
        system.refresh_all();
        Self {
            system,
            interval,
            log_dir,
            rotation_interval,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure log directory exists
        if !self.log_dir.exists() {
            fs::create_dir_all(&self.log_dir)?;
        }

        println!("Starting monitoring... Press Ctrl+C to stop.");
        println!("Logs will be saved to: {:?}", self.log_dir);

        let mut current_file_start = Instant::now();
        // Force creation of first file
        let mut wtr = self.create_new_writer()?;

        loop {
            // Check for rotation
            if current_file_start.elapsed() >= self.rotation_interval {
                wtr.flush()?;
                wtr = self.create_new_writer()?;
                current_file_start = Instant::now();
            }

            // Sleep first to allow CPU usage calculation (needs time delta)
            thread::sleep(self.interval);

            self.system.refresh_all();

            let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
            let memory_used = self.system.used_memory() / 1024 / 1024;
            let memory_total = self.system.total_memory() / 1024 / 1024;
            let cpu_global = self.system.global_cpu_usage();

            let cpus: Vec<f32> = self
                .system
                .cpus()
                .iter()
                .map(|cpu| cpu.cpu_usage())
                .collect();

            let metrics = SystemMetrics {
                timestamp: now,
                memory_used_mb: memory_used,
                memory_total_mb: memory_total,
                cpu_global_usage: cpu_global,
                cpus,
            };

            wtr.serialize(metrics)?;
            wtr.flush()?;
        }
    }

    fn create_new_writer(&mut self) -> Result<Writer<std::fs::File>, Box<dyn std::error::Error>> {
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let filename = format!("device_metrics_{}.csv", timestamp);
        let file_path = self.log_dir.join(filename);

        println!("Creating new log file: {:?}", file_path);

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&file_path)?;

        let mut wtr = Writer::from_writer(file);

        // Write header
        let mut headers = vec![
            "timestamp".to_string(),
            "memory_used_mb".to_string(),
            "memory_total_mb".to_string(),
            "cpu_global_usage".to_string(),
        ];
        // Add headers for each core
        // We need to refresh cpu to get the cores count correctly if not yet
        self.system.refresh_all();
        for (i, _) in self.system.cpus().iter().enumerate() {
            headers.push(format!("cpu_core_{}", i));
        }
        wtr.write_record(&headers)?;

        Ok(wtr)
    }
}
