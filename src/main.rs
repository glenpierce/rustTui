use crossterm::event::{self, Event, KeyCode};
use std::{collections::HashMap, env, fs, io, time::Instant};
use ratatui::{
    prelude::*,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    style::{Color, Style, Modifier},
};
use tachyonfx::{fx, Duration, EffectManager, Interpolation};

// Updated function: returns (top_commands, debug_messages)
fn read_history_commands() -> (Vec<(String, usize)>, Vec<String>) {
    let mut debug: Vec<String> = Vec::new();

    // Try common history files; prefer zsh then bash
    let mut candidates = Vec::new();
    if let Ok(home) = env::var("HOME") {
        candidates.push(format!("{}/.zsh_history", home));
        candidates.push(format!("{}/.bash_history", home));
    } else {
        debug.push("ENV: HOME not set".to_string());
    }

    // debug.push(format!("Candidate history files: {:?}", candidates));

    let mut counts: HashMap<String, usize> = HashMap::new();

    for path in &candidates {
        // debug.push(format!("Attempting to read {}", path));
        match fs::read(path) {
            Ok(bytes) => {
                debug.push(format!("Read {} bytes from {}", bytes.len(), path));
                // Convert bytes to text, replacing invalid UTF-8 sequences
                let content_cow = String::from_utf8_lossy(&bytes);
                // debug.push(format!(
                //     "After lossy UTF-8 conversion: {} chars, {} lines",
                //     content_cow.len(),
                //     content_cow.lines().count()
                // ));

                // show a few sample lines for diagnosis
                // for (i, line) in content_cow.lines().take(6).enumerate() {
                //     debug.push(format!("sample line {}: {}", i + 1, line));
                // }

                for line in content_cow.lines() {
                    // zsh history lines can look like: ": 1600000000:0;git status"
                    // bash lines are plain commands.
                    let cmd_part = if let Some(idx) = line.find(';') {
                        // after the last ';' if multiple, use substring after it
                        let after = &line[idx + 1..];
                        after.trim()
                    } else {
                        line.trim()
                    };

                    if cmd_part.is_empty() {
                        continue;
                    }

                    // Get the command name (first token). If it's "sudo", try the next token.
                    let mut iter = cmd_part.split_whitespace();
                    let first = iter.next().unwrap_or("");
                    let name = if first == "sudo" {
                        iter.next().unwrap_or(first)
                    } else {
                        first
                    }
                    .to_string();

                    if !name.is_empty() {
                        *counts.entry(name).or_default() += 1;
                    }
                }

                // debug.push(format!("Commands parsed so far: {}", counts.len()));
                // if we successfully read one history file, don't fallback further
                if !counts.is_empty() {
                    // debug.push(format!("Stopping after successful parse of {}", path));
                    break;
                } else {
                    // debug.push(format!("No commands found in {}, continuing to next candidate", path));
                }
            }
            Err(e) => {
                let msg = format!("Failed to read {}: {}", path, e);
                debug.push(msg.clone());
                eprintln!("{}", msg);
            }
        }
    }

    if counts.is_empty() {
        debug.push("No history commands parsed from any candidate file.".to_string());
        eprintln!("No history commands parsed from any candidate file.");
    } else {
        // debug.push(format!("Total unique commands parsed: {}", counts.len()));
    }

    // Collect and sort unique command names and append them to debug output,
    // one command per line for easy inspection.
    if !counts.is_empty() {
        let mut names: Vec<String> = counts.keys().cloned().collect();
        names.sort();
        debug.push(format!("Command names ({}):", names.len()));
        for name in names {
            debug.push(format!("  {}", name));
        }
    }

    let mut pairs: Vec<(String, usize)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    pairs.truncate(20);
    (pairs, debug)
}

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

    // Read history and append top 20 commands to messages
    let (mut top_cmds, history_debug) = read_history_commands();

    // if !history_debug.is_empty() {
    //     for line in history_debug {
    //         messages.push(format!("  {}", line));
    //     }
    //     messages.push(String::new());
    // }

    if !top_cmds.is_empty() {
        // Ensure descending sort by count and limit to 20 entries
        top_cmds.sort_by(|a, b| b.1.cmp(&a.1));
        if top_cmds.len() > 20 {
            top_cmds.truncate(20);
        }

        messages.push("Top commands:".to_string());
        for (i, (name, count)) in top_cmds.into_iter().enumerate() {
            messages.push(format!("  {:>2}. {:<25} {}", i + 1, name, count));
        }
        messages.push(String::new());
    }

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
