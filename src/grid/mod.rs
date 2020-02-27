use rand::Rng;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

type Board = [[u32; 4]; 4];

#[derive(Debug)]
pub struct Grid {    
    board: Board,
    pub score: u32,
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
}


impl Grid {
    pub fn new() -> Self {

        let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

        let mut grid = Grid {
            board,
            score: 0,
            move_left: true,
            move_right: true,
            move_up: true,
            move_down: true,
        };

        grid.random_tile();
        grid.random_tile();

        grid.update_moves();
        grid
    }

    #[allow(dead_code)]
    fn new_from_board(board: Board) -> Self {
        let mut g = Grid {
            board,
            score: 0,
            move_left: true,
            move_right: true,
            move_up: true,
            move_down: true,
        };
        g.update_moves();
        g
    }

    pub fn slide(&self, direction: Direction) -> (Board, u32) {

        let mut board = self.board;
        let mut score = 0;

        match direction {
            Direction::LEFT => {
                for i in 0..4 {
                    let mut collisions = [false, false, false, false];
                    for j in 1..4 {
                        for k in 0..j {
                            if board[i][j-k] > 0 && 
                                (board[i][j-k-1] == board[i][j-k] && !collisions[j-k-1] && !collisions[j-k] || board[i][j-k-1] == 0){
                                if board[i][j-k-1] == board[i][j-k] {
                                    collisions[j-k-1] = true;
                                    score += board[i][j-k-1] * 2;
                                }
                                board[i][j-k-1] += board[i][j-k];
                                board[i][j-k] = 0;
                            }
                        }
                    }
                }
            },
            Direction::RIGHT => {
                for i in 0..4 {
                    let mut collisions = [false, false, false, false];
                    for j in (0..3).rev() {
                        for k in 0..(3-j) {
                            if board[i][j+k] > 0 && (board[i][j+k+1] == board[i][j+k] && !collisions[j+k+1] && !collisions[j+k]|| board[i][j+k+1] == 0) {
                                if board[i][j+k+1] == board[i][j+k] {
                                    collisions[j+k+1] = true;
                                    score += board[i][j+k+1] * 2;
                                }
                                board[i][j+k+1] += board[i][j+k];
                                board[i][j+k] = 0;
                            }
                        }
                    }
                }
            },
            Direction::UP => {
                for j in 0..4 {
                    let mut collisions = [false, false, false, false];
                    for i in 1..4 {
                        for k in 0..i {
                            if board[i-k][j] > 0 && (board[i-k-1][j] == board[i-k][j] && !collisions[i-k-1] && !collisions[i-k]|| board[i-k-1][j] == 0) {
                                if board[i-k-1][j] == board[i-k][j] {
                                    collisions[i-k-1] = true;
                                    score += board[i-k-1][j] * 2;
                                }
                                board[i-k-1][j] += board[i-k][j];
                                board[i-k][j] = 0;
                            }
                        }
                    }
                }
            },
            Direction::DOWN => {
                for j in 0..4 {
                    let mut collisions = [false, false, false, false];
                    for i in (0..3).rev() {
                        for k in 0..(3-i) {
                            if board[i+k][j] > 0 && (board[i+k+1][j] == board[i+k][j] && !collisions[i+k+1] && !collisions[i+k] || board[i+k+1][j] == 0) {
                                if board[i+k+1][j] == board[i+k][j] {
                                    collisions[i+k+1] = true;
                                    score += board[i+k+1][j] * 2;
                                }
                                board[i+k+1][j] += board[i+k][j];
                                board[i+k][j] = 0;
                            }
                        }
                    }
                }
            }
        }
        (board, score)
    }

    pub fn random_tile(&mut self) {

        // unable to add more, so don't add
        if self.count_filled() == 16 {
            return
        }

        let mut rng = rand::thread_rng();
        let (mut row, mut col) = (rng.gen_range(0, 4), rng.gen_range(0,4));
        while self.board[row][col] != 0 {
           row = rng.gen_range(0,4);
           col = rng.gen_range(0,4);
        }

        
        self.board[row][col] = if rng.gen_range(0,6) >= 2 { 2 } else { 4 };
    }

    fn count_filled(&self) -> u32 {
        let mut count = 0;
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] > 0 {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn update_moves(&mut self) {

        let directions = [Direction::LEFT, Direction::RIGHT, Direction::UP, Direction::DOWN];
        for &direction in directions.iter() {

            let (updated, _) = self.slide(direction);
             
            match direction {
                Direction::LEFT => self.move_left = !(updated == self.board),
                Direction::RIGHT => self.move_right = !(updated == self.board),
                Direction::UP => self.move_up = !(updated == self.board),
                Direction::DOWN => self.move_down = !(updated == self.board),
            }
            

        }

    }

    pub fn update_board(&mut self, board: Board) {
        self.board = board;
    }

    pub fn game_over(&self) -> (bool, bool) {

        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 2048 {
                    return (true, true);
                }
            }
        }

        if self.move_down || self.move_up || self.move_left || self.move_right {
            (false, false)
        } else {
            (true, false)
        }

    }

    pub fn reset(&mut self) {
        let new_grid = Grid::new();

        self.board = new_grid.board;
        self.update_moves();
    }
}  

fn color(val: u32) -> String {

    let col_val = match val {
        2 => "31",
        4 => "32",
        8 => "33",
        16 => "34",
        32 => "35",
        64 => "36",
        128 => "37",
        256 => "91",
        512 => "92",
        1024 => "93",
        2048 => "94",
        _ => return format!("{:>4}", val),
    };

    format!("\x1B[7m\x1B[{}m{:>4}\x1B[0m", col_val, val)


}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+----+----+----+----+\n\
                    |{}|{}|{}|{}|\n\
                    +----+----+----+----+\n\
                    |{}|{}|{}|{}|\n\
                    +----+----+----+----+\n\
                    |{}|{}|{}|{}|\n\
                    +----+----+----+----+\n\
                    |{}|{}|{}|{}|\n\
                    +----+----+----+----+", 
            color(self.board[0][0]), color(self.board[0][1]), color(self.board[0][2]), color(self.board[0][3]),
            color(self.board[1][0]), color(self.board[1][1]), color(self.board[1][2]), color(self.board[1][3]),
            color(self.board[2][0]), color(self.board[2][1]), color(self.board[2][2]), color(self.board[2][3]),
            color(self.board[3][0]), color(self.board[3][1]), color(self.board[3][2]), color(self.board[3][3])
        )
    }
}

#[test]
fn test_new() {
    let grid = Grid::new();
    let mut count_filled = 0;
    for i in 0..4 {
        for j in 0..4 {
            if grid.board[i][j] >= 2 {
                count_filled += 1;
            } 
        }
    }
    assert_eq!(count_filled, 2);
}

#[test]
fn test_random_tile() {
    
    let gen_cells = |random_gen: u32 | -> u32 {
        let mut grid = Grid::new();

        for _ in 0..random_gen {
            grid.random_tile();
        }

        grid.count_filled()
    };

    assert_eq!(4, gen_cells(2));
    assert_eq!(8, gen_cells(6));
    assert_eq!(12, gen_cells(10));
    assert_eq!(15, gen_cells(13));
    assert_eq!(16, gen_cells(14));
    assert_eq!(16, gen_cells(15));
}

#[test]
fn test_output() {
    let board = [[4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]];
    let grid = Grid::new_from_board(board);

    assert_eq!(format!(
                "+----+----+----+----+\n\
                |{}|{}|   0|   0|\n\
                +----+----+----+----+\n\
                |   0|   0|   0|   0|\n\
                +----+----+----+----+\n\
                |   0|   0|   0|   0|\n\
                +----+----+----+----+\n\
                |   0|   0|   0|{}|\n\
                +----+----+----+----+", 
                color(4), color(4), color(2)), grid.to_string());
}

#[test]
fn test_slide() {
    let board =  [[4, 4, 2, 0], [4, 0, 2, 0], [0, 0, 0, 0], [0, 0, 2, 2]];
    let expected_left =  [[8, 2, 0, 0], [4, 2, 0, 0], [0, 0, 0, 0], [4, 0, 0, 0]];
    let expected_right =  [[0, 0, 8, 2], [0, 0, 4, 2], [0, 0, 0, 0], [0, 0, 0, 4]];
    let expected_up =  [[8, 4, 4, 2], [0, 0, 2, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    let expected_down =  [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, 0], [8, 4, 4, 2]];

    let combine_horizontal = [[2, 2, 2, 2], [2, 2, 0, 4], [0, 0 ,4, 2], [8, 4, 4, 0]];
    let expected_horizonal = [[4, 4, 0, 0], [4, 4, 0, 0], [4, 2, 0, 0], [8, 8, 0, 0]];

    let combine_vertical = [[4, 0, 0, 0], [2, 0, 0, 0], [2, 2, 2, 2], [0, 0, 0, 4]];
    let expected_vertical = [[4, 2,2 ,2], [4, 0, 0, 4], [0, 0,0 ,0], [0, 0, 0, 0]];

    let grid_test = Grid::new_from_board(board);
    let grid_h = Grid::new_from_board(combine_horizontal);
    let grid_v = Grid::new_from_board(combine_vertical);

    assert_eq!(grid_test.slide(Direction::LEFT), (expected_left, 12));
    assert_eq!(grid_test.slide(Direction::RIGHT), (expected_right, 12));
    assert_eq!(grid_test.slide(Direction::UP), (expected_up, 12));
    assert_eq!(grid_test.slide(Direction::DOWN), (expected_down, 12));
    assert_eq!(grid_h.slide(Direction::LEFT), (expected_horizonal, 20));
    assert_eq!(grid_v.slide(Direction::UP), (expected_vertical, 4));

}

#[test]
fn test_moves_available() {
    let board = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]];

    let no_moves = Grid::new_from_board(board);
    
    assert_eq!(no_moves.move_left, false);
    assert_eq!(no_moves.move_right, false);
    assert_eq!(no_moves.move_up, false);
    assert_eq!(no_moves.move_down, false);
}

#[test]
fn test_update_board() {
    let board =  [[4, 4, 2, 0], [4, 0, 2, 0], [0, 0, 0, 0], [0, 0, 2, 2]];
    let mut grid = Grid::new();
    grid.update_board(board);
    assert_eq!(grid.board, board);
}

#[test]
fn test_game_over() {

    let grid1 = Grid::new_from_board([[4, 4, 2, 0], [4, 0, 2, 0], [0, 0, 0, 0], [0, 0, 2, 2]]);
    let grid2 = Grid::new_from_board([[4, 4, 2, 0], [4, 0, 2, 0], [0, 0, 2048, 0], [0, 0, 2, 2]]);

    let grid3 = Grid::new_from_board([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]);
    assert_eq!(grid1.game_over(), (false, false));
    assert_eq!(grid2.game_over(), (true, true));
    assert_eq!(grid3.game_over(), (true, false));


}

#[test]
fn test_reset() {
    let mut grid = Grid::new_from_board([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]);

    grid.reset();
    let mut count_filled = 0;
    for i in 0..4 {
        for j in 0..4 {
            if grid.board[i][j] >= 2 {
                count_filled += 1;
            } 
        }
    }
    assert_eq!(count_filled, 2);

}