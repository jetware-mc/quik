extern crate sys_info;
use std::env;
use std::fs;
use crate::{configuration::Config, CONFIG};

#[macro_export]
macro_rules! point {
    ($($arg:tt)*) => ({
        use chrono::Utc;
        let now = Utc::now().format("%H:%M:%S");
        println!("[{}] {}", now, format_args!($($arg)*));
    })
}

pub fn startup() {
    point!("Starting up...");
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let os_type = sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string());
    let cpu_speed = sys_info::cpu_speed().unwrap_or_default();
    let max_ram = sys_info::mem_info().map(|x| x.total).unwrap_or_default();
    let disk_space = sys_info::disk_info().map(|x| x.total).unwrap_or_default();
    let current_dir = env::current_dir().unwrap_or_else(|_| fs::canonicalize(".").unwrap());

    let intro_line = format!(
        "│ OS: {}, Arch: {}, OS Type: {}, CPU Speed: {} MHz, Max RAM: {} MB, Disk Space: {} GB, Current Dir: {} │",
        os, arch, os_type, cpu_speed, max_ram / 1024, disk_space / (1024 * 1024 * 1024), current_dir.display()
    );

    let border_line = "┌─────────────────────────────────────────────────────────────────────────────────────────────────────────┐";
    let bottom_line = "└─────────────────────────────────────────────────────────────────────────────────────────────────────────┘";

    println!("{}", border_line);
    println!("{}", intro_line);
    println!("{}", bottom_line);
}
