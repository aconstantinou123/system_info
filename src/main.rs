use system_info;
use std::process;

fn main() {
    if let Err(e) = system_info::get_cpu_info() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
