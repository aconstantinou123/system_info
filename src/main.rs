mod util;

use system_info::CPUUsage;
use std::process;
use std::io;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Widget};
use tui::Terminal;

use util::event::{Event, Events};


fn main() -> Result<(), failure::Error> {
    let events = Events::new();
    let mut cpu_usage = CPUUsage::new();
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            Chart::default()
            .block(
                Block::default()
                        .title("Chart")
                        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels_style(Style::default().modifier(Modifier::ITALIC))
                        .bounds([0.0, 100.0])
                        .labels(&["0", "25", "50", "75", "100"]),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels_style(Style::default().modifier(Modifier::ITALIC))
                        .bounds([0.0, 100.0])
                        .labels(&["0", "20", "40", "60", "80", "100"]),
                )
                .datasets(&[
                    Dataset::default()
                        .name("CPU %")
                        .marker(Marker::Dot)
                        .style(Style::default().fg(Color::Cyan))
                        .data(&cpu_usage.get_usage()[..]),
                ])
                .render(&mut f, size);
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
            }
        }
    }
    Ok(())
}
