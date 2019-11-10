use std::{ fs };
use std::path::Path;
use std::str;
use std::io;

pub struct CPUUsage {
    pub usage: Vec<(f64, f64)>,
}


impl CPUUsage {

    pub fn new() -> CPUUsage {
        let usage = vec![];
        CPUUsage {
            usage,
        }
    }

    pub fn add_cpu_data(&mut self) -> io::Result<()>{
        let new_cpu_data = get_cpu_info()?;
        let time_to_add = match self.usage.last() {
             Some(x) =>  x.1 + 1.0,
             None => 1.0
        };
        self.usage.push((time_to_add, new_cpu_data));
        Ok(())
    }

    pub fn get_usage(&self) -> &Vec<(f64, f64)> {
        self.usage.as_ref()
    }
}

pub fn get_cpu_info() -> Result<f64, io::Error> {
    let cpu_file_path = Path::new("/proc/stat");
    let cpu_file = fs::read_to_string(cpu_file_path)?;
    let cpu_info = match cpu_file.lines().next() {
        Some(i) => i,
        None => "File Error",
    };
    let mut cpu_vec: Vec<&str> = cpu_info
        .split(' ')
        .collect();

    let cpu_values: Vec<f64> = cpu_vec
        .drain(2..)
        .map(| x | x.parse().unwrap())
        .collect();

    let cpu_usage = calculate_cpu_usage(&cpu_values);
    Ok(cpu_usage)
}

fn calculate_cpu_usage(cpu_vec: &Vec<f64>) -> f64 {
    let result = (cpu_vec[0] + cpu_vec[2]) * 100.0 / 
        (cpu_vec[0] + cpu_vec[2] + cpu_vec[3]);
    result
}
