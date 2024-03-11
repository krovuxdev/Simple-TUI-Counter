use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use std::io::{self, stdout};
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            unsafe {
                match key.code {
                    KeyCode::Left => X.prev(),
                    KeyCode::Right => X.next(),
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    _ => {
                        return Ok(false);
                    }
                };
            }
        }
    }
    Ok(false)
}
struct Iter {
    num: i16,
}
impl Iter {
    fn next(&mut self) -> i16 {
        self.num += 1;
        self.num.into()
    }
    fn prev(&mut self) -> i16 {
        self.num -= 1;
        self.num.into()
    }
}
static mut X: Iter = Iter { num: 0 };
fn ui(frame: &mut Frame) {
    let paragraph = Paragraph::new(
        format! {" (press left)<---  {}  --->(press right)",unsafe {X.num}}
            .green()
            .bold(),
    );
    frame.render_widget(
        paragraph.centered().block(
            Block::new()
                .title(
                    Title::from("Count".light_red())
                        .alignment(Alignment::Center)
                        .position(Position::Top),
                )
                .borders(Borders::all())
                .reset()
                .title_bottom("Exit press <q>".red())
                .yellow()
                .padding(Padding::new(0, 0, 6, 1)),
        ),
        frame.size(),
    );
}
