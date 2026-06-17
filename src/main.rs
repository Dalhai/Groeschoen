mod finance;

use anyhow::{Context, Result};

use chrono::NaiveDate;
use finance::{Account, Category, Payee, Transaction};
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
                transactions: load_sample_transactions()?,
            },
        })
    }
}

/// Path to the sample transactions export used during development.
const SAMPLE_TRANSACTIONS_PATH: &str = "dev/sample_transactions_actual.csv";

/// Loads sample transactions from the development CSV export.
///
/// Expected columns:
/// `Account, Date, Payee, Notes, Category_Group, Category, Amount, Split_Amount, Cleared`.
/// Records are read as raw bytes and decoded with lossy UTF-8, since the export
/// contains some invalid byte sequences.
fn load_sample_transactions() -> Result<Vec<Transaction>> {
    let path = SAMPLE_TRANSACTIONS_PATH;
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("failed to open transactions file `{path}`"))?;

    let mut transactions = Vec::new();
    for (row, record) in reader.byte_records().enumerate() {
        let record = record.with_context(|| format!("failed to read row {row} of `{path}`"))?;

        // Decode a column to an owned String, replacing any invalid UTF-8.
        let field = |index: usize| {
            String::from_utf8_lossy(record.get(index).unwrap_or_default()).into_owned()
        };

        let date_raw = field(1);
        let date = NaiveDate::parse_from_str(date_raw.trim(), "%Y-%m-%d")
            .with_context(|| format!("row {row}: invalid date `{date_raw}`"))?;

        let amount_raw = field(6);
        let amount = match amount_raw.trim() {
            "" => 0.0,
            value => value
                .parse()
                .with_context(|| format!("row {row}: invalid amount `{value}`"))?,
        };

        transactions.push(Transaction {
            amount,
            payee: Payee(field(2)),
            notes: field(3),
            category: Category(field(5)),
            date,
        });
    }

    Ok(transactions)
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
