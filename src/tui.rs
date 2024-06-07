use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::*,
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

#[derive(Default)]
pub enum AppScreen {
    #[default]
    Tab1,
    Tab2,
}

#[derive(Default, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
}

pub struct App {
    app_state: AppState,
    screen: AppScreen,
    terminal_handle: Tui,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(App {
            app_state: AppState::default(),
            screen: AppScreen::default(),
            terminal_handle: terminal,
        })
    }

    pub fn init_terminal(&self) -> io::Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(())
    }

    pub fn restore_terminal(&self) -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn draw(&mut self) -> io::Result<()> {
        let _ = self.terminal_handle.draw(|frame| {
            frame.render_widget(
                LineGauge::default()
                    .block(Block::bordered().title("Progress"))
                    .gauge_style(
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .line_set(symbols::line::THICK)
                    .ratio(0.4),
                Rect {
                    x: 10,
                    y: 10,
                    width: 100,
                    height: 1,
                },
            );
        });
        Ok(())
    }

    pub fn handle_input(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press
                    && key_event.code == KeyCode::Char('q') =>
            {
                self.app_state = AppState::Quitting;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        while self.app_state != AppState::Quitting {
            self.draw()?;
            self.handle_input()?;
        }
        Ok(())
    }
}
