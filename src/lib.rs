use std::{ fs };
use std::path::Path;
use std::str;
use std::io;


pub fn get_cpu_info() -> io::Result<()> {
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

    println!("{:?}", cpu_values);
    let cpu_usage = calculate_cpu_usage(&cpu_values);
    println!("cpu {}%", cpu_usage);
    Ok(())
}

fn calculate_cpu_usage(cpu_vec: &Vec<f32>) -> f32 {
    let result = (cpu_vec[0] + cpu_vec[2]) * 100.0 / 
        (cpu_vec[0] + cpu_vec[2] + cpu_vec[3]);
    result
}
