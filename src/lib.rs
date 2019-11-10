use std::{ fs };
use std::path::Path;
use std::str;
use std::io;

pub struct CPUUsage {
    pub usage: Vec<f32>,
    pub time: Vec<f32>,
}


impl CPUUsage {

    pub fn new() -> CPUUsage {
        let usage = vec![];
        let time = vec![];
        CPUUsage {
            usage,
            time
        }
    }

    pub fn add_cpu_data(&mut self) -> io::Result<()>{
        let new_cpu_data = get_cpu_info()?;
        self.usage.push(new_cpu_data);
        let time_to_add = match self.time.last() {
             Some(x) =>  x + 1.0,
             None => 1.0
        };
        self.time.push(time_to_add);
        Ok(())
    }

    pub fn get_usage(&self) -> &Vec<f32> {
        self.usage.as_ref()
    }
    
    pub fn get_time(&self) -> &Vec<f32> {
        self.time.as_ref()
    }
}

pub fn get_cpu_info() -> Result<f32, io::Error> {
    let cpu_file_path = Path::new("/proc/stat");
    let cpu_file = fs::read_to_string(cpu_file_path)?;
    let cpu_info = match cpu_file.lines().next() {
        Some(i) => i,
        None => "File Error",
    };
    let mut cpu_vec: Vec<&str> = cpu_info
        .split(' ')
        .collect();

    let cpu_values: Vec<f32> = cpu_vec
        .drain(2..)
        .map(| x | x.parse().unwrap())
        .collect();

    let cpu_usage = calculate_cpu_usage(&cpu_values);
    Ok(cpu_usage)
}

fn calculate_cpu_usage(cpu_vec: &Vec<f32>) -> f32 {
    let result = (cpu_vec[0] + cpu_vec[2]) * 100.0 / 
        (cpu_vec[0] + cpu_vec[2] + cpu_vec[3]);
    result
}
