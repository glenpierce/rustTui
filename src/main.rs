use crossterm::event::{self, Event};
use std::{io, time::Instant};

use ratatui::{prelude::*, widgets::Paragraph};
use tachyonfx::{fx, EffectManager, Interpolation};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut effects: EffectManager<()> = EffectManager::default();

    // Add a simple fade-in effect
    let fx = fx::fade_to(Color::Cyan, Color::Gray, (1_000, Interpolation::SineIn));
    effects.add_effect(fx);

    let mut last_frame = Instant::now();
    loop {
        let elapsed = last_frame.elapsed();
        last_frame = Instant::now();

        terminal.draw(|frame| {
            let screen_area = frame.area();

            // Render your content
            let text = Paragraph::new("Hello, TachyonFX!").alignment(Alignment::Center);
            frame.render_widget(text, screen_area);

            // Apply effects
            effects.process_effects(elapsed.into(), frame.buffer_mut(), screen_area);
        })?;

        // Exit on any key press
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(_) = event::read()? {
                break;
            }
        }
    }

    ratatui::restore();
    Ok(())
}
