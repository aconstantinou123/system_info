use system_info::CPUUsage;
use std::process;
use std::{thread, time};

fn main() {
    let mut cpu_usage = CPUUsage::new();
    let second = time::Duration::from_millis(1000);
    loop {
        if let Err(e) = cpu_usage.add_cpu_data() {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
        println!("usage: {:?}", cpu_usage.get_usage());
        println!("time: {:?}", cpu_usage.get_time());
        thread::sleep(second);
    }
}
