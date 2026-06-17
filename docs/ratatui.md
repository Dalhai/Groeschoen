# Ratatui reference

Working Poor uses [ratatui](https://ratatui.rs/) for its terminal UI. Keep these
handy when working on anything UI-related:

- API docs: https://docs.rs/ratatui/latest/ratatui/
- Installation / setup: https://ratatui.rs/installation/
- "Hello Ratatui" tutorial: https://ratatui.rs/tutorials/hello-ratatui/

## Notes for this project (ratatui 0.30)

- `ratatui::init()` sets up the terminal (raw mode, alternate screen) and returns
  a `DefaultTerminal`. `ratatui::restore()` tears it back down. Always pair them so
  the terminal is restored even on error. (0.30 also offers `ratatui::run(app)` as a
  convenience that wraps both, but it expects a `std::io::Result` closure — we use
  the explicit init/restore pair so we can return `anyhow::Result` from `main`.)
- Draw with `terminal.draw(|frame| ...)`. Build layout with `Layout::vertical` /
  `Layout::horizontal` and `.areas(rect)`, render widgets with
  `frame.render_widget(widget, area)`.
- crossterm is re-exported under `ratatui::crossterm`, so no separate dependency or
  version matching is needed.
