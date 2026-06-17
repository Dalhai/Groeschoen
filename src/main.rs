mod finance;
mod import;

use anyhow::Result;

use finance::Account;
use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, style::Stylize, text::{Line, Text}, widgets::{Block, Paragraph}
};

/// Top-level application state.
struct App {
    /// The account currently being worked on.
    account: Account,
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            account: Account {
                name: "Checking".to_string(),
                balance: 0.0,
                transactions: import::actual::load_sample_transactions()?,
            },
        })
    }
}

fn main() -> Result<()> {
    let app = App::new()?;

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &app);

    ratatui::restore();
    result
}

/// Main event loop: draw, then wait for a key. `q` or `Esc` quits.
fn run(terminal: &mut DefaultTerminal, app: &App) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, app))?;

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
fn draw(frame: &mut Frame, app: &App) {
    // Split the screen into a title bar and a body
    let [title_area, body_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(frame.area());

    // Split the body into a left sidebar and a main area
    let [sidebar_area, main_area] =
        Layout::horizontal([Constraint::Length(28), Constraint::Min(0)]).areas(body_area);

    let title = Paragraph::new("Working Poor".bold())
        .centered()
        .block(Block::bordered());
    frame.render_widget(title, title_area);

    let account = &app.account;

    let sidebar =
        Paragraph::new(account.name.as_str()).block(Block::bordered().title("Accounts".bold()));
    frame.render_widget(sidebar, sidebar_area);

    let lines = account
        .transactions
        .iter()
        .map(|t| format!("{}: {} ({})", t.date, t.amount, t.payee.0))
        .map(|l| Line::from(l))
        .collect::<Vec<Line>>();

    let workspace = Paragraph::new(Text::from(lines)).block(Block::bordered().title("Workspace".bold()));
    frame.render_widget(workspace, main_area);
}
