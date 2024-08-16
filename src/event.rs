use crate::quit;
use crate::redraw;
use crate::run;
use crate::Data;
use std::io::Write;
use termion::cursor::{SteadyBar, SteadyBlock};
use termion::event::Key;
use termion::input::TermRead;

pub fn event_loop(data: &mut Data) {
    for evt in data.stdin.lock().keys() {
        redraw(data);
        let k = evt.unwrap();
        match data.mode {
            'c' => match k {
                Key::Esc => {
                    data.mode = 'n';
                    data.command = String::new();
                    data.command_x = 0;
                    write!(data.stdout, "{}", SteadyBlock).unwrap();
                }
                Key::Char(c) => {
                    if c == '\n' {
                        if run(data) {
                            quit(&mut data.stdout);
                            break;
                        } else {
                            data.command = String::new();
                            data.command_x = 0;
                            write!(data.stdout, "{}", SteadyBlock).unwrap();
                        }
                    } else {
                        let command1 = String::from(&data.command[0..data.command_x]);
                        let command2 = String::from(&data.command[data.command_x..]);
                        data.command = command1 + &c.to_string() + &command2;
                        data.command_x += 1;
                    }
                }
                Key::Backspace => {
                    if data.command.len() > 0 {
                        let command1 = String::from(&data.command[0..data.command_x]);
                        let command2 = String::from(&data.command[data.command_x..]);
                        data.command = String::from(&command1[0..command1.len() - 1]) + &command2;
                        data.command_x -= 1;
                    } else {
                        data.mode = 'n';
                        write!(data.stdout, "{}", SteadyBlock).unwrap();
                    }
                }
                _ => {
                    println!("{:?}", k);
                    break;
                }
            },
            'n' => {
                match k {
                    Key::Char('k') => {
                        data.move_up();
                    }
                    Key::Char('j') => {
                        data.move_down();
                    }
                    Key::Char('h') => {
                        data.move_left();
                    }
                    Key::Char('l') => {
                        data.move_right();
                    }
                    Key::Char('$') => {
                        data.x = data.content[data.y as usize - 1 + data.top].len() as u16;
                        data.x_max = true;
                    }
                    Key::Char(':') => {
                        data.mode = 'c';
                        write!(data.stdout, "{}", SteadyBar).unwrap();
                    }
                    Key::Char('i') => {
                        data.mode = 'i';
                        write!(data.stdout, "{}", SteadyBar).unwrap();
                    }
                    Key::Char('a') => {
                        data.mode = 'i';
                        data.x += 1;
                        write!(data.stdout, "{}", SteadyBar).unwrap();
                    }
                    Key::Char('o') => {
                        data.mode = 'i';
                        let mut new_content: Vec<String> = Vec::new();
                        for i in 0..data.content.len() {
                            new_content.push(data.content[i].clone());
                            if i as u16 + 1 == data.y + data.top as u16 {
                                new_content.push(String::new());
                            }
                        }
                        data.content = new_content;
                        if data.y == data.height - 3 {
                            data.top += 1;
                        } else {
                            data.y += 1;
                        }
                        write!(data.stdout, "{}", SteadyBar).unwrap();
                    }
                    Key::Char('d') => {
                        if data.content.len() == 1 {
                            data.content = Vec::new();
                            data.content.push(String::new());
                        } else {
                            let mut new_content: Vec<String> = Vec::new();
                            for i in 0..data.content.len() {
                                if i as u16 + 1 != data.y + data.top as u16 {
                                    new_content.push(data.content[i].clone());
                                }
                            }
                            if data.y as usize + data.top == data.content.len() {
                                data.y = (data.content.len() - data.top - 1) as u16;
                            }
                            if data.y < 1 {
                                data.y = 1;
                                data.top -= 1;
                            }
                            data.content = new_content;
                        }
                    }
                    Key::Char('G') => {
                        if data.height - 2 < data.content.len() as u16 {
                            data.y = data.height - 2;
                            data.top = data.content.len() - data.y as usize;
                        } else {
                            data.y = (data.content.len() - data.top) as u16;
                        }
                    }
                    _ => {}
                }
                if data.x == 0 {
                    data.x = 1;
                }
            }
            'i' => match k {
                Key::Esc => {
                    data.mode = 'n';
                    if data.x > 1 {
                        data.x -= 1;
                    }
                    write!(data.stdout, "{}", SteadyBlock).unwrap();
                }
                Key::Up => {
                    data.move_up();
                }
                Key::Down => {
                    data.move_down();
                }
                Key::Left => {
                    data.move_left();
                }
                Key::Right => {
                    data.move_right();
                }
                Key::Backspace => {
                    let mut line = data.content[data.y as usize - 1 + data.top].clone();
                    if line.len() > 0 {
                        let line1 = String::from(&line[0..(data.x as usize - 1)]);
                        let line2 = String::from(&line[(data.x as usize - 1)..]);
                        if line1.len() > 0 {
                            line = String::from(&line1[0..line1.len() - 1]) + &line2;
                            data.content[data.y as usize - 1 + data.top] = line;
                            data.x -= 1;
                        } else if data.y + data.top as u16 > 1 {
                            data.content[data.y as usize - 2 + data.top] += &line2;
                            data.content.remove(data.y as usize - 1 + data.top);
                            data.y -= 1;
                            if data.y < 1 {
                                data.y = 1;
                                data.top -= 1;
                            }
                            data.x = data.content[data.y as usize - 1 + data.top].len() as u16 + 1;
                        }
                    } else if data.y + data.top as u16 > 1 {
                        data.content.remove(data.y as usize - 1 + data.top);
                        data.y -= 1;
                        if data.y < 1 {
                            data.y = 1;
                            data.top -= 1;
                        }
                        data.x = data.content[data.y as usize - 1 + data.top].len() as u16 + 1;
                    }
                }
                Key::Char(c) => {
                    let mut line = data.content[data.y as usize - 1 + data.top].clone();
                    if line.len() == 0 {
                        line = c.to_string();
                        data.content[data.y as usize - 1 + data.top] = line;
                        data.x += 1;
                    } else {
                        let line1 = String::from(&line[0..(data.x as usize - 1)]);
                        let line2 = String::from(&line[(data.x as usize - 1)..]);
                        if c == '\n' {
                            data.content[data.y as usize - 1 + data.top] = line1;
                            let mut new_content: Vec<String> = Vec::new();
                            for i in 0..data.content.len() {
                                new_content.push(data.content[i].clone());
                                if i as u16 + 1 == data.y + data.top as u16 {
                                    new_content.push(line2.clone());
                                }
                            }
                            data.content = new_content;
                            data.x = 1;
                            if data.y == data.height - 3 {
                                data.top += 1;
                            } else {
                                data.y += 1;
                            }
                        } else {
                            line = line1 + &c.to_string() + &line2;
                            data.content[data.y as usize - 1 + data.top] = line;
                            data.x += 1;
                        }
                    }
                }
                _ => {}
            },
            'e' => match k {
                Key::Char('\n') => {}
                Key::Char(':') => {
                    data.mode = 'c';
                    write!(data.stdout, "{}", SteadyBar).unwrap();
                }
                Key::Up | Key::Char('k') => {
                    data.move_up();
                }
                Key::Down | Key::Char('j') => {
                    data.move_down();
                }
                _ => {}
            },
            _ => {}
        }
        redraw(data);
        data.stdout.flush().unwrap();
    }
}
