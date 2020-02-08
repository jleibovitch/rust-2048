

pub mod grid;
pub mod display;

use display::Display;

fn main() {
    let mut game_grid = grid::Grid::new();
    println!("{:?}", game_grid);

    let game_display = display::cli_display::CLIDisplay::new();
    game_display.render(&game_grid);

    loop {
        let exit_flag = game_display.input(&mut game_grid);
        if exit_flag {
            break
        }
        game_display.render(&game_grid);
    }
    
}
