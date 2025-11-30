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
    let mut messages: Vec<String> = Vec::new();
    let mut last_frame = Instant::now();

    loop {
        let elapsed = last_frame.elapsed();
        last_frame = Instant::now();

        terminal.draw(|frame| {
            let area = frame.area();

            // Split the terminal vertically: header, input, and messages area
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [Constraint::Length(3), Constraint::Length(3), Constraint::Min(3)].as_ref(),
                )
                .split(area);

            // Header (top)
            let header = Paragraph::new("Hello, TachyonFX!").alignment(Alignment::Center);
            frame.render_widget(header, chunks[0]);

            // Input box (middle)
            let input_widget = Paragraph::new(input.as_str())
                .block(Block::default().borders(Borders::ALL).title("Input"));
            frame.render_widget(input_widget, chunks[1]);

            // Messages (bottom) show submitted inputs
            let msgs_text = if messages.is_empty() {
                "No messages yet".to_string()
            } else {
                messages.join("\n")
            };
            let messages_widget = Paragraph::new(msgs_text)
                .block(Block::default().borders(Borders::ALL).title("Messages"))
                .alignment(Alignment::Left);
            frame.render_widget(messages_widget, chunks[2]);

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
                        if !input.is_empty() {
                            messages.push(input.clone());
                            input.clear();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}
