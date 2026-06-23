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
        let mut transactions = import::actual::load_sample_transactions()?;
        // Newest transactions first.
        transactions.sort_by_key(|t| std::cmp::Reverse(t.date));

        Ok(Self {
            account: Account {
                name: "Checking".to_string(),
                transactions,
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

    let title = Paragraph::new("Gröschön".bold())
        .centered()
        .block(Block::bordered());
    frame.render_widget(title, title_area);

    let account = &app.account;

    let accounts_block = Block::bordered().title("Accounts".bold());
    let accounts_inner = accounts_block.inner(sidebar_area);
    frame.render_widget(accounts_block, sidebar_area);

    frame.render_widget(Paragraph::new(account.name.as_str()), accounts_inner);
    frame.render_widget(
        Paragraph::new(format!("{:.2}", account.balance())).right_aligned(),
        accounts_inner,
    );

    let header = Row::new(vec!["Date", "Payee", "Notes", "Category", "Amount"]).bold();

    let rows = account
        .transactions
        .iter()
        .map(|t| {
            let amount = Cell::from(Text::from(format!("{:.2}", t.amount)).right_aligned());
            let amount = if t.is_deposit() { amount.green() } else { amount };

            Row::new(vec![
                Cell::from(t.date.to_string()),
                Cell::from(t.payee.0.clone()),
                Cell::from(t.notes.clone()),
                Cell::from(t.category.0.clone()),
                amount,
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
