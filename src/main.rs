mod util;

use system_info::{CPUUsage, MemInfo, ProcessInfo};
use std::process;
use std::io;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::style::{Color, Modifier, Style};
use tui::layout::{Constraint, Layout};
use tui::widgets::{Axis, Row, Table,  Block, Borders, Chart, Dataset, Marker, Widget};
use tui::Terminal;
use std::path::Path;
use std::{thread, time};

use util::event::{Event, Events};


fn main() -> Result<(), failure::Error> {
    let mut process_info = ProcessInfo::new()?;
    let proc_path = Path::new("/proc/");
    process_info.update(&proc_path)?; 
    let second = time::Duration::from_millis(1000); 
    thread::sleep(second);
    process_info.update(&proc_path)?;
    let events = Events::new();
    let mut cpu_usage = CPUUsage::new();
    let mut mem_info = MemInfo::new();
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    loop {
        terminal.draw(|mut f| {
        let chunks = Layout::default()
            .constraints([Constraint::Length(20), Constraint::Min(0)].as_ref())
            .split(f.size());
        let size = f.size();
        Chart::default()
            .block(
              Block::default()
                      .title("CPU/Memory")
                      .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD))
                      .borders(Borders::ALL),
              )
              .x_axis(
                  Axis::default()
                      .title("Seconds")
                      .style(Style::default().fg(Color::Gray))
                      .labels_style(Style::default().modifier(Modifier::ITALIC))
                      .bounds([0.0, 300.0])
                      .labels(&["0", "50", "100", "150", "200", "250", "300"]),
              )
              .y_axis(
                  Axis::default()
                      .title("%")
                      .style(Style::default().fg(Color::Gray))
                      .labels_style(Style::default().modifier(Modifier::ITALIC))
                      .bounds([0.0, 100.0])
                      .labels(&["0", "20", "40", "60", "80", "100"]),
              )
              .datasets(&[ 
                  Dataset::default()
                      .name(&format!("CPU {:.2}%", cpu_usage.get_current_cpu()))
                      .marker(Marker::Dot)
                      .style(Style::default().fg(Color::Cyan))
                      .data(&cpu_usage.get_usage()[..]),
                  Dataset::default()
                      .name(&format!("Memory {:.2}%", mem_info.get_current_mem()))
                      .marker(Marker::Dot)
                      .style(Style::default().fg(Color::Magenta))
                      .data(&mem_info.get_usage()[..]),

              ])
              .render(&mut f, chunks[0]);

            let selected_style = Style::default().fg(Color::Yellow).modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);
            let header = ["PID", "Process Name","State", "UTime", "STime", "Total Time", "RSS", "Memory %",  "CPU %"];
            let rows = process_info.get_processes().iter().enumerate().map(|(i, process)| {
                let process_vec: Vec<String> = vec![
                    process.pid.to_string(),
                    process.process_name.clone(),
                    process.state.clone(),
                    process.utime.to_string(),
                    process.stime.to_string(),
                    process.total_time.to_string(),
                    process.rss.to_string(),
                    format!("{:.2}", process.mem_percent),
                    format!("{:.2}", process.cpu_percent),
                    ];
                Row::StyledData(process_vec.into_iter(), normal_style) 
                // if i == app.selected {
                //} else {
                // Row::StyledData(item.into_iter(), normal_style)
                // }
            });
            // println!("{:?}", rows);
            let rects = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .margin(5)
                .split(f.size());
            Table::new(header.into_iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("Processes"))
                .widths(&[
                    20,
                    20,
                    20,
                    20,
                    20,
                    20,
                    20,
                    20,
                    20,
                ])
                .render(&mut f, chunks[1]); 
        })?;
      match events.next()? {
          Event::Input(input) => {
              if input == Key::Char('q') {
                  println!("quit");
                  break;
              }

          }
          Event::Tick => {
              if let Err(e) = cpu_usage.add_cpu_data() {
                  eprintln!("Application error: {}", e);
                  process::exit(1);
              }
              if let Err(e) = mem_info.add_mem_data() {
                  eprintln!("Application error: {}", e);
                  process::exit(1);
              }
              if let Err(e) = process_info.update(&proc_path) {
                  eprintln!("Application error: {}", e);
                  process::exit(1);
              }   
          }

      }
    }
    Ok(())
}
