mod app;
mod context;
mod ui;
use app::run;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}
