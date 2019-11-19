use std::{ fs };
use std::path::Path;
use std::str;
use std::io;
use regex::Regex;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: i32,
    pub process_name: String,
    pub state: String,
    pub utime: f64,
    pub stime: f64,
    pub total_time: f64,
    pub mem_percent: f64,
    pub cpu_percent: f64,
}

impl Process {

    pub fn new(pid: i32, state: String,  process_name: String, utime: f64, stime: f64) -> Process {
        let total_time = utime + stime;
        let mem_percent = 0.0;
        let cpu_percent = 0.0;
        Process {
            pid,
            process_name,
            state,
            utime,
            stime,
            total_time,
            mem_percent,
            cpu_percent,
        }
    }

    pub fn set_cpu_percent(&mut self, percent: f64) {
        self.cpu_percent = percent;
    }
}

#[derive(Debug)]
pub struct ProcessInfo {
    processes: Vec<Process>, 
    cpu_time_diff: f64,
    current_cpu_time: f64,
}


impl ProcessInfo {
    
    pub fn new() -> ProcessInfo {
        let processes = vec![];
        let cpu_time_diff = 0.0;
        let current_cpu_time = 0.0;
        ProcessInfo {
            processes,
            cpu_time_diff,
            current_cpu_time,
        }
    }

    pub fn get_processes(&self) -> &Vec<Process> {
        self.processes.as_ref()
    }

    pub fn sort_by_cpu(&mut self) {
        self.processes
            .sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap());
    }

    pub fn update(&mut self, proc_path: &Path) -> Result<(), io::Error>{
        self.update_cpu_diff()?;
        self.read_dirs(&proc_path)?;
        self.sort_by_cpu();
        Ok(())
    }

    pub fn read_dirs(&mut self, proc_path: &Path) -> Result<(), io::Error> {
        // println!("{:?}", proc_path);
        let mut dirs = vec![];
        let mut path;
        let digits_only = Regex::new("^[0-9]*$").unwrap();
        for entry in fs::read_dir(proc_path)? {
            let entry = entry?;
            path = entry.path();
            if path.is_dir() && digits_only.is_match(path.file_name().unwrap().to_str().unwrap()) {
                dirs.push(path);
            }
            else if path.is_file() && path.file_name().unwrap() == "stat" 
                && proc_path.to_str().unwrap() != "/proc/" {
                let contents = match fs::read_to_string(&path) {
                    Ok(f) => f,
                    Err(e) => {
                        e.to_string()
                    },
                };
                self.get_mem_data(&contents);
            }
        }
        for dir in dirs.iter() {
            self.read_dirs(&dir);
        }
        Ok(())
    }

    pub fn get_mem_data(&mut self, contents: &str) -> Result<(), io::Error> {
        let contents_array: Vec<&str> = contents
            .split_whitespace()
            .collect();
        let pid: i32 = contents_array[0].parse().unwrap();
        let process_name: String = contents_array[2].replace("(", "").replace(")", "");
        let state: String = contents_array[1].replace("(", "").replace(")", "");
        let utime: f64 = contents_array[13].parse().unwrap();
        let stime: f64 = contents_array[14].parse().unwrap();
        let total_time = utime + stime;
        let mem_data = Process::new(
            pid,
            process_name,
            state,
            utime,
            stime,
        );
        self.add_to_processes(mem_data);
        Ok(())
    }

    pub fn update_cpu_diff(&mut self) -> Result<(), io::Error> {
        let current_cpu_time = self.get_cpu_info()?;
        self.cpu_time_diff = current_cpu_time - self.current_cpu_time;
        self.current_cpu_time = current_cpu_time;
        // println!("{}", self.cpu_time_diff);
        Ok(())
    }


    pub fn add_to_processes(&mut self, mut process: Process) -> Result<(), io::Error> {
        let found_process = self.processes.iter()
            .find(|p| p.pid == process.pid);
        match found_process {
            Some(p) => {
                let utime_percent = 100.0 * (process.utime - p.utime) / self.cpu_time_diff;
                let stime_percent = 100.0 * (process.stime - p.stime) / self.cpu_time_diff;
                let percent = 100.0 * (process.total_time - p.total_time) / self.cpu_time_diff;
                process.set_cpu_percent(percent);
                let filtered_processes: Vec<Process> = self.processes.iter().cloned()
                    .filter(| x| x.pid != process.pid)
                    .collect();
                self.processes = filtered_processes;
                // println!("{:?}", process);
                self.processes.push(process);
            },
            None => self.processes.push(process),
        };
        Ok(())
    }

    pub fn get_cpu_info(&mut self) -> Result<f64, io::Error>{
        let mut cpu_time = 0.0;
        let cpu_vec = create_cpu_vector()?;
        for val in cpu_vec.iter() {
            cpu_time += val;
        } 
        Ok(cpu_time)
    }
}

pub fn create_cpu_vector() -> Result<Vec<f64>, io::Error> {
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
    Ok(cpu_values)
}
