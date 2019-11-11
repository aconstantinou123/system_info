use std::{ fs };
use std::path::Path;
use std::str;
use std::io;

pub struct CPUUsage {
    usage: Vec<(f64, f64)>,
    prev_time: f64,
    prev_usage: f64,
}


impl CPUUsage {

    pub fn new() -> CPUUsage {
        let usage = vec![];
        let prev_time = 0.0;
        let prev_usage = 0.0;
        CPUUsage {
            usage,
            prev_time,
            prev_usage,
        }
    }

    pub fn add_cpu_data(&mut self) -> io::Result<()>{
        if self.usage.len() >= 300 {
            self.usage.clear();
        }
        let new_cpu_data = self.get_cpu_info()?;
        let time_to_add = match self.usage.last() {
             Some(x) =>  x.0 + 1.0,
             None => 1.0
        };
        self.usage.push((time_to_add, new_cpu_data));
        Ok(())
    }

    pub fn get_usage(&self) -> &Vec<(f64, f64)> {
        self.usage.as_ref()
    }

    pub fn get_cpu_info(&mut self) -> Result<f64, io::Error> {
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

        let cpu_usage = self.calculate_current_cpu_usage(&cpu_values);
        Ok(cpu_usage)
    }

    fn calculate_current_cpu_usage(&mut self, cpu_vec: &Vec<f64>) -> f64 {
        let current_usage = cpu_vec[0] + cpu_vec[2];
        let current_time = cpu_vec[0] + cpu_vec[2] + cpu_vec[3];
        if self.prev_time == 0.0 && self.prev_usage == 0.0 {
            self.prev_usage = current_usage;
            self.prev_time = current_time;
        }
        let current_cpu = (current_usage - self.prev_usage) * 100.0 / 
            (current_time - self.prev_time);
        self.prev_usage = current_usage;
        self.prev_time = current_time;
        current_cpu
    }
}


