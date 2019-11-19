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
    current_cpu: f64,
}


impl CPUUsage {

    pub fn new() -> CPUUsage {
        let usage = vec![];
        let prev_time = 0.0;
        let prev_usage = 0.0;
        let current_cpu = 0.0;
        CPUUsage {
            usage,
            prev_time,
            prev_usage,
            current_cpu,
        }
    }

    pub fn get_current_cpu(&self) -> f64 {
        self.current_cpu
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
        self.current_cpu = current_cpu;
        current_cpu
    }
}

#[derive(Debug)]
pub struct MemInfo {
    usage: Vec<(f64, f64)>,
    current_mem: f64,
}

impl MemInfo {

    pub fn new() -> MemInfo {
        let usage = vec![];
        let current_mem = 0.0;
        MemInfo {
            usage,
            current_mem,
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

    pub fn get_current_mem(&self) -> f64 {
        self.current_mem
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
        let mem_total = extract_kb_info(mem_total_line);
        let mem_free = extract_kb_info(mem_free_line);
        let percentage_used = mem_free / mem_total * 100.0;
        self.current_mem = percentage_used;
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
    pub rss: f64,
}

impl Process {

    pub fn new(pid: i32, state: String,  process_name: String, utime: f64, stime: f64, rss: f64) -> Process {
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
            rss,
        }
    }

    pub fn set_cpu_percent(&mut self, percent: f64) {
        self.cpu_percent = percent;
    }

    pub fn set_utime(&mut self, utime: f64) {
        self.utime = utime;
    }

    pub fn set_stime(&mut self, stime: f64) {
        self.stime = stime;
    }

    pub fn set_total_time(&mut self, total_time: f64) {
        self.total_time = total_time;
    }

    pub fn set_mem_percent(&mut self, mem_percent: f64) {
        self.mem_percent = mem_percent;
    }

    pub fn set_rss(&mut self, rss: f64) {
        self.rss = rss;
    }

}

#[derive(Debug)]
pub struct ProcessInfo {
    processes: Vec<Process>, 
    cpu_time_diff: f64,
    current_cpu_time: f64,
    total_mem: f64,
}


impl ProcessInfo {
    
    pub fn new() -> Result<ProcessInfo, io::Error> {
        let processes = vec![];
        let cpu_time_diff = 0.0;
        let current_cpu_time = 0.0;
        let total_mem = get_total_mem_info()?;
       Ok(ProcessInfo {
            processes,
            cpu_time_diff,
            current_cpu_time,
            total_mem,
        })
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
                self.get_cpu_data(&contents);
            }
            else if path.is_file() && path.file_name().unwrap() == "status" 
                && proc_path.to_str().unwrap() != "/proc/" {
                let contents = match fs::read_to_string(&path) {
                    Ok(f) => f,
                    Err(e) => {
                        e.to_string()
                    },
                };
                self.get_proccess_mem(&contents);
            }

        }
        for dir in dirs.iter() {
            self.read_dirs(&dir);
        }
        Ok(())
    }

    pub fn get_proccess_mem(&mut self, contents: &str) -> Result<(), io::Error> {
        let mut rss = 0.0;
        let mem_total_line: Vec<&str> = contents.lines()
            .filter(|line| line.contains("VmRSS"))
            .collect();
        if !mem_total_line.is_empty(){
            let mem_total_vec: Vec<&str> = mem_total_line[0]
                .split_whitespace()
                .collect();
            rss = mem_total_vec[1].parse().unwrap();
        }
                
        let pid_vec = get_line_from_file(&contents, "Pid");
        let state_vec = get_line_from_file(&contents, "State");
        let name_vec = get_line_from_file(&contents, "Name");
        let pid: i32 = pid_vec[1].parse().unwrap();
        let process_name = String::from(name_vec[0]);
        let state = String::from(state_vec[0]);
        let mem_percent = rss / self.total_mem * 100.0;
        let utime = 0.0;
        let stime = 0.0;
        let total_time = 0.0;
        let cpu_percent = 0.0;
        let mut new_process = Process::new(
            pid,
            process_name,
            state,
            utime,
            stime,
            rss,
        );
        new_process.set_mem_percent(mem_percent);
        self.add_mem_info_to_processes(new_process);
        Ok(())
    }

    pub fn add_mem_info_to_processes(&mut self, mut process: Process) -> Result<(), io::Error> {
        let found_process = self.processes.iter()
            .find(|p| p.pid == process.pid);
        match found_process {
            Some(p) => {
                process.set_stime(p.stime);
                process.set_utime(p.utime);
                process.set_total_time(p.total_time);
                process.set_cpu_percent(p.cpu_percent);
                let filtered_processes: Vec<Process> = self.processes.iter().cloned()
                    .filter(| x| x.pid != process.pid)
                    .collect();
                self.processes = filtered_processes;
                self.processes.push(process);
            },
            None => self.processes.push(process),
        };
        Ok(())
    }

    pub fn get_cpu_data(&mut self, contents: &str) -> Result<(), io::Error> {
        let contents_array: Vec<&str> = contents
            .split_whitespace()
            .collect();
        let pid: i32 = contents_array[0].parse().unwrap();
        let process_name: String = contents_array[2].replace("(", "").replace(")", "");
        let state: String = contents_array[1].replace("(", "").replace(")", "");
        let utime: f64 = contents_array[13].parse().unwrap();
        let stime: f64 = contents_array[14].parse().unwrap();
        let rss = 0.0;
        let total_time = utime + stime;
        let mem_data = Process::new(
            pid,
            process_name,
            state,
            utime,
            stime,
            rss,
        );
        self.add_cpu_info_to_processes(mem_data);
        Ok(())
    }

    pub fn update_cpu_diff(&mut self) -> Result<(), io::Error> {
        let current_cpu_time = self.get_cpu_info()?;
        self.cpu_time_diff = current_cpu_time - self.current_cpu_time;
        self.current_cpu_time = current_cpu_time;
        Ok(())
    }


    pub fn add_cpu_info_to_processes(&mut self, mut process: Process) -> Result<(), io::Error> {
        let found_process = self.processes.iter()
            .find(|p| p.pid == process.pid);
        match found_process {
            Some(p) => {
                let utime_percent = 100.0 * (process.utime - p.utime) / self.cpu_time_diff;
                let stime_percent = 100.0 * (process.stime - p.stime) / self.cpu_time_diff;
                let percent = 100.0 * (process.total_time - p.total_time) / self.cpu_time_diff;
                process.set_cpu_percent(percent);
                process.set_rss(p.rss);
                process.set_mem_percent(p.mem_percent);
                let filtered_processes: Vec<Process> = self.processes.iter().cloned()
                    .filter(| x| x.pid != process.pid)
                    .collect();
                self.processes = filtered_processes;
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

pub fn extract_kb_info(line: Vec<&str>) -> f64 {
        let line_vec: Vec<&str> = line[0]
            .split_whitespace()
            .collect();
        let kb: f64 = line_vec[1]
            .parse()
            .unwrap();
        kb    
}

pub fn get_total_mem_info() -> Result<f64, io::Error> {
        let mem_file_path = Path::new("/proc/meminfo");
        let mem_file = fs::read_to_string(mem_file_path)?;
        let mem_total_line: Vec<&str> = mem_file.lines()
            .filter(|line| line.contains("MemTotal"))
            .collect();
        let total_mem = extract_kb_info(mem_total_line);
        Ok(total_mem)
 }

pub fn get_line_from_file<'a>(file: &'a str, pattern: &str) -> Vec<&'a str> {
    let found_line: Vec<&str> = file.lines()
            .filter(|line| line.contains(pattern))
            .collect();
    let found_vec: Vec<&str> = found_line[0]
            .split_whitespace()
            .collect();
    found_vec
}
