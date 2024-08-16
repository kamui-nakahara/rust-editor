mod command;
mod data;
mod event;
mod keyfunctions;
mod screen;
mod settings;

use command::run;
use data::Data;
use event::event_loop;
use screen::redraw;
use settings::Settings;
use std::io::{Stdout, Write};
use termion::clear::All;
use termion::color::{Bg, Reset, Rgb};
use termion::cursor::{Goto, Show, SteadyBlock};
use termion::raw::RawTerminal;

fn quit(stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}{}{}{}", Show, Bg(Reset), Goto(1, 1), All).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = Data::new();
    let mut args = std::env::args();
    if args.len() > 1 {
        data.load(args.nth(1).unwrap()).unwrap();
    } else {
        data.load(String::new()).unwrap();
    }
    write!(data.stdout, "{}{}", Bg(Rgb(20, 20, 20)), SteadyBlock).unwrap();
    redraw(&mut data);
    data.stdout.flush().unwrap();
    event_loop(&mut data);
    Ok(())
}
