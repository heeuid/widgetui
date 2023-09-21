use std::{cell::RefMut, error::Error};

use ratatui::{
    prelude::{Constraint, Direction, Layout},
    widgets::Paragraph,
};
use tui_helper::*;

struct TestChunk;

pub fn chunk_generator(frame: &mut Frame, mut chunks: RefMut<Chunks>) -> WidgetResult {
    let chunk = layout! {
        frame.size(),
        constraint!(%50),
        constraint!(#1) => {constraint!(#3), constraint!(%100), constraint!(#3)},
        constraint!(%50)
    }[1][1];

    chunks.register_chunk::<TestChunk>(chunk);

    Ok(())
}

pub fn render(
    frame: &mut Frame,
    chunks: RefMut<Chunks>,
    mut events: RefMut<Events>,
) -> WidgetResult {
    let chunk = chunks.get_chunk::<TestChunk>()?;

    frame.render_widget(Paragraph::new("Hello, world!"), chunk);

    if events.key(crossterm::event::KeyCode::Char('q')) {
        events.register_exit();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    App::new(100)?
        .with_widget(chunk_generator)
        .with_widget(render)
        .run()
}