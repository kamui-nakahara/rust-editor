use crate::Data;
use std::fs::canonicalize;
use std::io::Write;
use std::path::{Path, PathBuf};
use termion::clear::All;
use termion::color::{Fg, Reset, Rgb};
use termion::cursor::Goto;

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

fn draw_file(data: &mut Data) {
    let length = data.content.len().to_string().len();
    for i in 0..min(data.content.len() - data.top, data.height as usize - 2) {
        if data.settings.number {
            let line = i + data.top + 1;
            if data.y - 1 == i as u16 {
                println!(
                    "{}{}{}{}{} {}",
                    Goto(1, i as u16 + 1),
                    " ".repeat(length - line.to_string().len()),
                    Fg(Rgb(250, 200, 0)),
                    line,
                    Fg(Reset),
                    data.content[i + data.top]
                );
            } else {
                println!(
                    "{}{}{}{}{} {}",
                    Goto(1, i as u16 + 1),
                    " ".repeat(length - line.to_string().len()),
                    Fg(Rgb(127, 127, 127)),
                    line,
                    Fg(Reset),
                    data.content[i + data.top]
                );
            }
        } else {
            println!("{}{}", Goto(1, i as u16 + 1), data.content[i + data.top]);
        }
    }
}

fn draw_dir(data: &mut Data) {
    let division = "=".repeat(data.width as usize);
    println!("{}{}{}", Goto(1, 1), Fg(Rgb(127, 127, 127)), division);
    let path = canonicalize(Path::new(&data.path))
        .unwrap()
        .to_string_lossy()
        .into_owned();
    println!("{}{}", Goto(1, 2), path);
    println!("{}{}{}", Goto(1, 3), division, Fg(Reset));
    for i in 0..min(data.listdir.len() - data.top, data.height as usize - 2 - 3) {
        let file = data.listdir[i + data.top].clone();
        let file_path = PathBuf::from(data.path.clone()).join(file.clone());
        let mut fg = Fg(Rgb(255, 255, 255));
        let mut mark = "";
        if file_path.is_dir() {
            fg = Fg(Rgb(0, 200, 100));
            mark = "/";
        }
        if data.y as usize == i + 1 {
            println!(
                "{}{}{}{}{}{}{}",
                Goto(1, i as u16 + 1 + 3),
                fg,
                termion::style::Underline,
                file,
                mark,
                Fg(Reset),
                termion::style::Reset,
            );
        } else {
            println!(
                "{}{}{}{}{}",
                Goto(1, i as u16 + 1 + 3),
                fg,
                file,
                mark,
                Fg(Reset),
            );
        }
    }
}

pub fn redraw(data: &mut Data) {
    write!(data.stdout, "{}", All).unwrap();
    if data.is_dir {
        draw_dir(data);
    } else {
        draw_file(data);
    }
    if data.path.is_empty() {
        write!(
            data.stdout,
            "{}{}[Untitled]{}",
            Goto(1, data.height - 1),
            Fg(Rgb(243, 152, 0)),
            Fg(Reset)
        )
        .unwrap();
    } else {
        let path = canonicalize(data.path.clone())
            .unwrap()
            .display()
            .to_string();
        write!(
            data.stdout,
            "{}{}{}{}",
            Goto(1, data.height - 1),
            Fg(Rgb(243, 152, 0)),
            path,
            Fg(Reset)
        )
        .unwrap();
    }
    match data.mode {
        'c' => {
            write!(
                data.stdout,
                "{}:{}{}{}",
                Goto(1, data.height + 1),
                Goto(2, data.height + 1),
                data.command,
                Goto(2 + data.command_x as u16, data.height + 1)
            )
            .unwrap();
        }
        'n' => {
            if data.settings.number {
                write!(
                    data.stdout,
                    "{}",
                    Goto(
                        data.x + data.content.len().to_string().len() as u16 + 1,
                        data.y
                    )
                )
                .unwrap();
            } else {
                write!(data.stdout, "{}", Goto(data.x, data.y)).unwrap();
            }
        }
        'i' => {
            if data.settings.number {
                write!(
                    data.stdout,
                    "{}",
                    Goto(
                        data.x + data.content.len().to_string().len() as u16 + 1,
                        data.y
                    )
                )
                .unwrap();
            } else {
                write!(data.stdout, "{}", Goto(data.x, data.y)).unwrap();
            }
        }
        _ => {}
    }
}
