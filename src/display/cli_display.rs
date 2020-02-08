use std::io::{Write, stdout, stdin};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use super::Display;
use super::super::grid;


pub struct CLIDisplay;

impl Display for CLIDisplay {

    fn new() -> Self {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide)
            .unwrap();
        stdout.flush().unwrap();
        CLIDisplay
    }

    fn input(&self, state: &mut grid::Grid) -> bool {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
       
        for c in stdin.keys() {
            // write!(stdout,
            //        "{}{}",
            //        termion::cursor::Goto(1, 1),
            //        termion::clear::CurrentLine)
            //         .unwrap();
        
            match c.unwrap() {

                Key::Char('q') => {
                    return true;
                },
                // Key::Char(c) => println!("{}", c),
                // Key::Alt(c) => println!("^{}", c),
                // Key::Ctrl(c) => println!("*{}", c),
                // Key::Esc => println!("ESC"),
                Key::Left => {
                    // println!(">");
                    state.slide(grid::Direction::LEFT);
                    break;
                },
                Key::Right => {
                    state.slide(grid::Direction::RIGHT);
                    break;
                },
                Key::Up =>{
                    state.slide(grid::Direction::UP);
                    break;
                },
                Key::Down => {
                    state.slide(grid::Direction::DOWN);
                    break;
                },

                // Key::Backspace => println!("×"),
                _ => {}
            }
           
        }
        state.random_tile();
        false
    }

    fn render(&self, state: &grid::Grid) {
        let mut stdout = stdout();
        write!(stdout,
                "{}{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::All,
                state)
                .unwrap();   
    }

}

impl Drop for CLIDisplay {
    fn drop(&mut self) {
        // let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", termion::cursor::Show).unwrap();
    }
}
