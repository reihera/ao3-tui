use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};
fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());
    let block = Block::default()
         .title("Block")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
         .title("Block 2")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
