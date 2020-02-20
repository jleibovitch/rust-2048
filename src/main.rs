

pub mod grid;
pub mod display;

use display::Display;

fn main() {

    let mut game_display = display::cli_display::CLIDisplay::new();
    
    // render the first frame
    game_display.render();

    while !game_display.get_exit_flag() {
        game_display.input();
        game_display.render(); 
    }
    
}
