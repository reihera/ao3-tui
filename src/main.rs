use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    style::{Style, Stylize},
    text::Line,
    layout::{Constraint, Layout, Direction},
    widgets::{Block, Paragraph, List, Tabs}

};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
}

impl App {

	

    /// Construct a new instance of [`App`].
    pub fn new() -> Self { 
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| {
            self.render(frame);
			})?;	
            self.handle_crossterm_events()?;
			
		}
 		Ok(())
 	}



    
	//render
    fn render(&mut self, frame: &mut Frame) {
		use Constraint::{Length, Min};
		let layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![
				Constraint::Percentage(25),
				Constraint::Percentage(100),
			])
			.split(frame.area());
   

		frame.render_widget(
			Tabs::new(vec!["Reader", "Changelog"])
				.select(2)
				.block(Block::bordered().title("fuck off"))
				.highlight_style(Style::default().red()),

			layout[1]
		);
    	frame.render_widget(
    		List::new(["placeholder 1", "placeholder 2"])
    			.block(Block::bordered().title("sidebar")),
    		layout[0]
    		)
    	}
    
 
    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    // [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
