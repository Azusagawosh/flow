use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}};
use crossterm::event::{read, Event, KeyCode};
use crate::notes;
use std::io::{stdout, Write};

enum Mode {
    Writing,
    Command,
}

pub fn on_start(note_name: String) {
    if enable_raw_mode().is_ok() {
        println!("Raw mode enabled — редактируем: {}", note_name);
    } else {
        println!("Failed to enable raw mode");
        return;
    }

    let mut mode = Mode::Writing;
    let mut content = String::new();
    let mut cmd_buffer = String::new();

    loop {
        if let Ok(Event::Key(key_event)) = read() {
            match mode {
                Mode::Writing => {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            content.push(c);
                            print!("{}", c);
                            stdout().flush().unwrap();
                        }
                        KeyCode::Enter => {
                            content.push_str("\n");
                            print!("\r\n");
                            stdout().flush().unwrap();
                        }
                        KeyCode::Esc => {
                            mode = Mode::Command;
                            println!("\r\n--- РЕЖИМ КОМАНД (q/qs/пусто) ---");
                            stdout().flush().unwrap();
                        }
                        KeyCode::Backspace => {
                            content.pop();
                            print!("\x08 \x08");
                            stdout().flush().unwrap();
                        }
                        _ => {}
                    }
                }
                Mode::Command => {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            cmd_buffer.push(c);
                            print!("{}", c);
                            stdout().flush().unwrap();
                        }
                        KeyCode::Backspace => {
                            cmd_buffer.pop();
                            print!("\x08 \x08");
                            stdout().flush().unwrap();
                        }
                        KeyCode::Enter => {
                            println!("\r\n");
                            match cmd_buffer.as_str() {
                                "q" => {
                                    println!("Выход без сохранения");
                                    break;
                                }
                                "qs" => {
                                    notes::save_note(note_name.clone(), content.clone());
                                    break;
                                }
                                "" => {
                                    mode = Mode::Writing;
                                    println!("\r\n--- ВОЗВРАТ К ПИСЬМУ ---");
                                }
                                _ => {
                                    println!("\r\nНеизвестная команда: {}", cmd_buffer);
                                }
                            }
                            cmd_buffer.clear();
                        }
                        KeyCode::Esc => {
                            mode = Mode::Writing;
                            println!("\r\n--- ВОЗВРАТ К ПИСЬМУ (Esc) ---");
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if disable_raw_mode().is_ok() {
        println!("Raw mode disabled");
    } else {
        println!("Failed to disable raw mode");
    }
}