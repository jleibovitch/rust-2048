use std::io::{Write, stdout, stdin};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use super::{Display, State};
use super::super::grid;
use std::boxed::Box;

pub struct CLIDisplay {
    state: Option<Box<dyn State<Self>>>,
    exit: bool
}

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
        CLIDisplay{
            state: Some(Box::new(Menu{})),
            exit: false
        }
    }

    fn input(&mut self) {

        let mut state = self.state.take().unwrap();
        self.exit = state.input(self);

        if self.state.is_none() {
            self.state = Some(state);
        }
        // self.state = Some(state);
        // self.exit = self.state.unwrap().as_mu.input(self);
        // self
    }

    fn render(&self) {
        self.state.as_ref().unwrap().render();  
    }

    fn get_state(&self) -> &dyn State<Self> {
        self.state.as_ref().unwrap().as_ref()
    }

    fn set_state(&mut self, state: Option<Box<dyn State<Self>>>) {
        self.state = state;
    }

    fn get_exit_flag(&self) -> bool {
        self.exit
    }

}

impl Drop for CLIDisplay {
    fn drop(&mut self) {
        // let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", termion::cursor::Show).unwrap();
    }
}

impl<T> State<T> for grid::Grid where T: Display{
    
    fn render(&self) {
        let mut stdout = stdout();
       
        write!(stdout,
                "{}{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::All,
                self)
                .unwrap(); 

        let (game_over, win) = self.game_over();
        if game_over {

            println!("
            
{}


1. Replay
2. Exit
            ", if win {"You won!"} else {"You lose..."});
        }
        stdout.flush().unwrap();
    }

    fn input(&mut self, _display: &mut T) -> bool {
        let stdin = stdin();
        let _stdout = stdout().into_raw_mode().unwrap();

        let mut direction: Option<grid::Direction>;
        let (game_over, _) = self.game_over();
       
        for c in stdin.keys() {
        
            match c.unwrap() {

                Key::Char('q') => {
                    return true;
                },
                Key::Left => {
                    direction = if self.move_left {Some(grid::Direction::LEFT)} else {None};
                },
                Key::Right => {
                    direction = if self.move_right {Some(grid::Direction::RIGHT)} else {None};
                },
                Key::Up =>{
                    direction = if self.move_up {Some(grid::Direction::UP)} else {None};
                },
                Key::Down => {
                    direction = if self.move_down { Some(grid::Direction::DOWN) } else {None};
                },
                Key::Char('1') => {
                    if game_over {
                        self.reset();
                        break;
                    } else {
                        direction = None;
                    }
                },
                Key::Char('2') => {
                    if game_over {
                        return true;
                    } else {
                        direction = None;
                    }
                },
                _ => {
                    direction = None;
                }
            }
           
            if direction.is_some() {
                self.update_board(self.slide(direction.unwrap()));
                self.random_tile();
                self.update_moves();
                break;
            }
        }       

        false
    }
}

pub struct Menu {

}

impl<T> State<T> for Menu 
 where T: Display {
    fn render(&self) {
        
        let mut stdout = stdout();
        write!(stdout, "{}{}  
         ___   ___  _  _   ___  
        |__ \\ / _ \\| || | / _ \\ 
           ) | | | | || || (_) |
          / /| | | |__   _> _ < 
         / /_| |_| |  | || (_) |
        |____|\\___/   |_| \\___/ 
                                
                                                                            
Menu:

1. Play
2. How To Play
3. Exit",
       termion::cursor::Goto(1, 1),
     termion::clear::All).unwrap();
     stdout.flush().unwrap();

    }
    fn input(&mut self, display: &mut T) -> bool {

        let stdin = stdin();
        let _stdout = stdout().into_raw_mode().unwrap();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('1') => {
                    display.set_state(Some(Box::new(grid::Grid::new()))); 
                    break;
                },
                Key::Char('3') => return true,
                _ => break
            }
        }
       
        false
    }
}