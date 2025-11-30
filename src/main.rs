use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Instant};

use ratatui::{
    prelude::*,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use tachyonfx::{fx, EffectManager, Interpolation};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut effects: EffectManager<()> = EffectManager::default();

    // Add a simple fade-in effect
    let fx = fx::fade_to(Color::Cyan, Color::Gray, (1_000, Interpolation::SineIn));
    effects.add_effect(fx);

    let mut input = String::new();
    let mut last_frame = Instant::now();

    loop {
        let elapsed = last_frame.elapsed();
        last_frame = Instant::now();

        terminal.draw(|frame| {
            let area = frame.area();

            // Split the terminal vertically: main area and an input box at the bottom
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(area);

            // Main content (top)
            let text = Paragraph::new("Hello, TachyonFX!").alignment(Alignment::Center);
            frame.render_widget(text, chunks[0]);

            // Input box (bottom) with a border and current input text
            let input_widget = Paragraph::new(input.as_str())
                .block(Block::default().borders(Borders::ALL).title("Input"));
            frame.render_widget(input_widget, chunks[1]);

            // Apply effects to the whole screen buffer
            effects.process_effects(elapsed.into(), frame.buffer_mut(), area);
        })?;

        // Poll for events and handle keys. Only exit on Escape.
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        // Respect printable characters only
                        if !c.is_control() {
                            input.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        // Optionally handle submit; currently ignored
                    }
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}
