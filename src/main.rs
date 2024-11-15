extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{terminal_size, color, style};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut string = String::from("");

    //write!(
    //    stdout,
    //    "{}{}q to exit. Type stuff, use alt, and so on.{}",
    //    termion::clear::All,
    //    termion::cursor::Goto(1, 1),
    //    termion::cursor::Hide
    //)
    //.unwrap();
    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();

    for k in stdin.keys() {
        // write!(
        //     stdout,
        //     "{}{}",
        //     termion::cursor::Goto(1, 1),
        //     termion::clear::CurrentLine
        // )
        // .unwrap();

        match k.as_ref().unwrap() {
            Key::Char('q') => break,
            Key::Char('s') => println!("Size is {:?}", terminal_size().unwrap()),
            Key::Char(c) => {string.push(*c)},
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => {let _ = string.pop();},
            _ => {
                println!("{:?}", k)
            }
        }
        //let hi = String::from("hi");
        write!(
            stdout,
            "{}{}REd{}",
            termion::cursor::Goto(1, terminal_size().unwrap().1),
            color::Fg(color::Red),
            color::Fg(color::White)
        ).unwrap();
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine,
            string
        ).unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
}
