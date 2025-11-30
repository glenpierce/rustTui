use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Instant};
use ratatui::{
    prelude::*,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    style::{Color, Style, Modifier},
};
use tachyonfx::{fx, Duration, EffectManager, Interpolation};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut effects: EffectManager<()> = EffectManager::default();
    let mut msg_effects: EffectManager<()> = EffectManager::default();

    // Vaporwave palette (RGB)
    let bg_dark = Color::Rgb(8, 0, 20);         // very dark background
    let neon_cyan = Color::Rgb(102, 255, 255);  // bright cyan
    let neon_pink = Color::Rgb(255, 102, 204);  // hot pink
    let vapor_purple = Color::Rgb(153, 102, 255); // soft purple

    // Global subtle purple -> cyan fade
    let fx_initial = fx::fade_to(vapor_purple, neon_cyan, (200, Interpolation::SineIn));
    effects.add_effect(fx_initial);

    let mut input = String::new();
    let mut messages: Vec<String> = Vec::new();
    let mut last_frame = Instant::now();

    loop {
        let elapsed = last_frame.elapsed();
        last_frame = Instant::now();

        terminal.draw(|frame| {
            let area = frame.area();

            // Layout: header, input, messages
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [Constraint::Length(3), Constraint::Length(3), Constraint::Min(3)].as_ref(),
                )
                .split(area);

            // Header with vaporwave styling
            let header = Paragraph::new("Vaporwave Terminal")
                .style(Style::default().fg(neon_pink).bg(bg_dark).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            frame.render_widget(header, chunks[0]);

            // Input box
            let input_widget = Paragraph::new(input.as_str()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Input")
                    .style(Style::default().fg(neon_cyan).bg(bg_dark)),
            );
            frame.render_widget(input_widget, chunks[1]);

            // Messages area
            let msgs_text = if messages.is_empty() {
                "No messages yet".to_string()
            } else {
                messages.join("\n")
            };
            let messages_widget = Paragraph::new(msgs_text).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Messages")
                    .style(Style::default().fg(neon_pink).bg(bg_dark)),
            ).alignment(Alignment::Left);
            frame.render_widget(messages_widget, chunks[2]);

            // Apply global effects to whole screen
            effects.process_effects(elapsed.into(), frame.buffer_mut(), area);

            // Apply message-only effects restricted to the messages area
            msg_effects.process_effects(elapsed.into(), frame.buffer_mut(), chunks[2]);
        })?;

        // Event handling
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
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

                            // Add a short coalesce + neon fade effect for the messages area
                            let coalesce_fx = fx::coalesce((200, Interpolation::SineIn));
                            msg_effects.add_effect(coalesce_fx);

                            let msg_flash = fx::fade_to(neon_pink, vapor_purple, (350, Interpolation::SineIn));
                            msg_effects.add_effect(msg_flash);
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
