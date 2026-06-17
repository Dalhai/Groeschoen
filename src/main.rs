mod account;

use anyhow::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Paragraph},
};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

/// Main event loop: draw, then wait for a key. `q` or `Esc` quits.
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
            {
                return Ok(());
            }
        }
    }
}

/// Render the title bar, the left sidebar and the main working area.
fn draw(frame: &mut Frame) {
    let [title_area, body_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(frame.area());

    let [sidebar_area, main_area] =
        Layout::horizontal([Constraint::Length(28), Constraint::Min(0)]).areas(body_area);

    let title = Paragraph::new("Working Poor".bold())
        .centered()
        .block(Block::bordered());
    frame.render_widget(title, title_area);

    let sidebar = Block::bordered().title("Sidebar");
    frame.render_widget(sidebar, sidebar_area);

    let main = Paragraph::new("Press q to quit").block(Block::bordered().title("Workspace"));
    frame.render_widget(main, main_area);
}
