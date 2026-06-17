mod finance;
mod import;

use anyhow::Result;

use finance::Account;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Cell, Paragraph, Row, Table, TableState},
};

/// Top-level application state.
struct App {
    /// The account currently being worked on.
    account: Account,

    /// Selection and scroll state for the transaction table.
    table_state: TableState,
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            account: Account {
                name: "Checking".to_string(),
                balance: 0.0,
                transactions: import::actual::load_sample_transactions()?,
            },
            table_state: TableState::default().with_selected(Some(0)),
        })
    }
}

fn main() -> Result<()> {
    let mut app = App::new()?;

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &mut app);

    ratatui::restore();
    result
}

/// Main event loop: draw, then handle a key. `q`/`Esc` quits; arrows move the
/// transaction selection.
fn run(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down => app.table_state.select_next(),
                    KeyCode::Up => app.table_state.select_previous(),
                    _ => {}
                }
            }
        }
    }
}

/// Render the title bar, the left sidebar and the main working area.
fn draw(frame: &mut Frame, app: &mut App) {
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

    let header = Row::new(vec!["Date", "Payee", "Notes", "Category", "Amount"]).bold();

    let rows = account
        .transactions
        .iter()
        .map(|t| {
            Row::new(vec![
                Cell::from(t.date.to_string()),
                Cell::from(t.payee.0.clone()),
                Cell::from(t.notes.clone()),
                Cell::from(t.category.0.clone()),
                Cell::from(Text::from(format!("{:.2}", t.amount)).right_aligned()),
            ])
        })
        .collect::<Vec<Row>>();

    // Date and Amount are fixed; the three text columns share the rest
    // proportionally so Notes always gets a slice.
    let widths = [
        Constraint::Length(10),
        Constraint::Fill(2),
        Constraint::Fill(3),
        Constraint::Fill(2),
        Constraint::Length(12),
    ];

    let workspace = Table::new(rows, widths)
        .header(header)
        .block(Block::bordered().title("Workspace".bold()))
        .row_highlight_style(Style::new().reversed())
        .highlight_symbol("▶ ");
    frame.render_stateful_widget(workspace, main_area, &mut app.table_state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use finance::{Category, Payee, Transaction};
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn sample_app() -> App {
        let txns = vec![
            Transaction {
                amount: 975.0,
                payee: Payee("Starting Balance".into()),
                notes: String::new(),
                category: Category("Starting Balances".into()),
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            },
            Transaction {
                amount: -280.0,
                payee: Payee("Shop".into()),
                notes: "weekend".into(),
                category: Category("Shopping".into()),
                date: NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
            },
        ];
        App {
            account: Account { name: "Checking".into(), balance: 0.0, transactions: txns },
            table_state: TableState::default().with_selected(Some(0)),
        }
    }

    #[test]
    fn renders_table_with_header_and_selection() {
        let mut app = sample_app();
        let mut terminal = Terminal::new(TestBackend::new(90, 12)).unwrap();
        terminal.draw(|f| draw(f, &mut app)).unwrap();

        let text: String =
            terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

        for needle in ["Date", "Payee", "Notes", "Category", "Amount", "975.00", "-280.00", "▶"] {
            assert!(text.contains(needle), "expected `{needle}` in render:\n{text}");
        }
    }
}
