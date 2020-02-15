

pub mod grid;
pub mod display;

use display::Display;

fn main() {
    let mut game_grid = grid::Grid::new();

    let game_display = display::cli_display::CLIDisplay::new();
    game_display.render(&game_grid);

    loop {
        let exit_flag = game_display.input(&mut game_grid);
        if exit_flag {
            break
        }
        game_display.render(&game_grid);

        let (game_over, win) = game_grid.game_over();
        if game_over {

            if win {
                println!("\n\nYou won!");
            } else {
                println!("\n\nYou lose..");
            }

            break;
        }

       
    }
    
}
