use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
    DefaultTerminal, Frame,
};

#[derive(Default, Debug, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Exiting,
}

#[derive(Default)]
pub struct App {
    state: AppState,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), anyhow::Error> {
        while self.state != AppState::Exiting {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<(), anyhow::Error> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self.handle_key_press(key_event.code),
            _ => (),
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Esc => self.state = AppState::Exiting,
            KeyCode::Char('q') => self.state = AppState::Exiting,
            _ => (),
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title("App").borders(Borders::ALL);
        block.render(area, buf);
    }
}
