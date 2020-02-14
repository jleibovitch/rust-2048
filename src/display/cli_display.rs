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

        let mut direction: Option<grid::Direction>;
       
        for c in stdin.keys() {
        
            match c.unwrap() {

                Key::Char('q') => {
                    return true;
                },
             
                Key::Left => {
                    
                    direction = if state.move_left {Some(grid::Direction::LEFT)} else {None};
                },
                Key::Right => {
                    direction = if state.move_right {Some(grid::Direction::RIGHT)} else {None};
                },
                Key::Up =>{
                    direction = if state.move_up {Some(grid::Direction::UP)} else {None};
                },
                Key::Down => {
                    direction = if state.move_down { Some(grid::Direction::DOWN) } else {None};
                },
                _ => {
                    direction = None;
                }
            }
           
            if direction.is_some() {
                state.update_board(state.slide(direction.unwrap()));
                state.random_tile();
                state.update_moves();
                break;
            }
        }       

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
