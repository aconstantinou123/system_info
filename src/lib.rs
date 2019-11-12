use std::{ fs };
use std::path::Path;
use std::str;
use std::io;
use regex::Regex;

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

    pub fn clear_usage(&mut self) {
        if self.usage.len() >= 300 {
            self.usage.clear();
        } 
    }

    pub fn add_cpu_data(&mut self) -> io::Result<()>{
        self.clear_usage();
        let new_cpu_data = self.get_cpu_info()?;
        let time_to_add = match self.usage.last() {
             Some(x) =>  x.0 + 1.0,
             None => 1.0
        };
        self.usage.push((time_to_add, new_cpu_data));
        // println!("{:?}", self.usage);
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

pub struct MemInfo {
    usage: Vec<(f64, f64)>,
}

impl MemInfo {

    pub fn new() -> MemInfo {
        let usage = vec![];
        MemInfo {
            usage
        }
    }

    pub fn clear_usage(&mut self) {
        if self.usage.len() >= 300 {
            self.usage.clear();
        } 
    }

    pub fn get_usage(&self) -> &Vec<(f64, f64)> {
        self.usage.as_ref()
    }

    pub fn extract_kb_info(&self, line: Vec<&str>) -> f64 {
        let line_vec: Vec<&str> = line[0]
            .split_whitespace()
            .collect();
        let kb: f64 = line_vec[1]
            .parse()
            .unwrap();
        kb
    }

    pub fn get_mem_info(&mut self) -> Result<f64, io::Error> {
        let mem_file_path = Path::new("/proc/meminfo");
        let mem_file = fs::read_to_string(mem_file_path)?;
        let mem_total_line: Vec<&str> = mem_file.lines()
            .filter(|line| line.contains("MemTotal"))
            .collect();
       let mem_free_line: Vec<&str> = mem_file.lines()
            .filter(|line| line.contains("MemFree"))
            .collect();
        let mem_total = self.extract_kb_info(mem_total_line);
        let mem_free = self.extract_kb_info(mem_free_line);
        let percentage_used = mem_free / mem_total * 100.0;
        Ok(percentage_used)

    }

    pub fn add_mem_data(&mut self) -> io::Result<()>{
        self.clear_usage();
        let new_mem_data = self.get_mem_info()?;
        let time_to_add = match self.usage.last() {
             Some(x) =>  x.0 + 1.0,
             None => 1.0
        };
        self.usage.push((time_to_add, new_mem_data));
        // println!("{:?}", self.usage);
        Ok(())
    }



}

pub struct ProcessInfo {
    process_cpu_usage: Vec<(String, f64)> 

}

impl ProcessInfo {
    
    pub fn new() -> ProcessInfo {
        let process_cpu_usage = vec![];
        ProcessInfo {
            process_cpu_usage
        }
    }

    pub fn read_dirs(&self, proc_path: &Path) -> Result<(), io::Error> {
        println!("{:?}", proc_path);
        let mut dirs = vec![];
        let mut path;
        let digits_only = Regex::new("^[0-9]*$").unwrap();
        for entry in fs::read_dir(proc_path)? {
            let entry = entry?;
            path = entry.path();
            if path.is_dir() && digits_only.is_match(path.file_name().unwrap().to_str().unwrap()) {
                dirs.push(path);
            }
            else if path.is_file() && path.file_name().unwrap() == "stat" {
                // println!("{:?}", path.file_name().unwrap());
                let contents = match fs::read_to_string(&path) {
                    Ok(f) => f,
                    Err(e) => {
                        e.to_string()
                    },
                };
                println!("{}", contents);
            }
        }
        for dir in dirs.iter() {
            self.read_dirs(&dir);
        }
        Ok(())
    }
}
