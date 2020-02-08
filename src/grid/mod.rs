use rand::Rng;
use std::fmt;

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
}


impl Grid {
    pub fn new() -> Self {

        let  board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    

        let mut grid = Grid {
            board
        };

        grid.random_tile();
        grid.random_tile();
        grid
    }

    pub fn slide(&mut self, direction: Direction) {

        match direction {
            Direction::LEFT => {
                for i in 0..4 {
                    for j in 1..4 {
                        for k in 0..j {
                            if self.board[i][j-k] > 0 && (self.board[i][j-k-1] == self.board[i][j-k] || self.board[i][j-k-1] == 0) {
                                self.board[i][j-k-1] += self.board[i][j-k];
                                self.board[i][j-k] = 0;
                            }
                        }
                    }
                }
            },
            Direction::RIGHT => {
                for i in 0..4 {
                    for j in (0..3).rev() {
                        for k in 0..(3-j) {
                            if self.board[i][j+k] > 0 && (self.board[i][j+k+1] == self.board[i][j+k] || self.board[i][j+k+1] == 0) {
                                self.board[i][j+k+1] += self.board[i][j+k];
                                self.board[i][j+k] = 0;
                            }
                        }
                    }
                }
            },
            Direction::UP => {
                for j in 0..4 {
                    for i in 1..4 {
                        for k in 0..i {
                            if self.board[i-k][j] > 0 && (self.board[i-k-1][j] == self.board[i-k][j] || self.board[i-k-1][j] == 0) {
                                self.board[i-k-1][j] += self.board[i-k][j];
                                self.board[i-k][j] = 0;
                            }
                        }
                    }
                }
            },
            Direction::DOWN => {
                for j in 0..4 {
                    for i in (0..3).rev() {
                        for k in 0..(3-i) {
                            if self.board[i+k][j] > 0 && (self.board[i+k+1][j] == self.board[i+k][j] || self.board[i+k+1][j] == 0) {
                                self.board[i+k+1][j] += self.board[i+k][j];
                                self.board[i+k][j] = 0;
                            }
                        }
                    }
                }
            }
        }

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

        self.board[row][col] = 2;
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
}  

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}\n{} {} {} {}\n{} {} {} {}\n{} {} {} {}", 
            self.board[0][0], self.board[0][1], self.board[0][2], self.board[0][3],
            self.board[1][0], self.board[1][1], self.board[1][2], self.board[1][3],
            self.board[2][0], self.board[2][1], self.board[2][2], self.board[2][3],
            self.board[3][0], self.board[3][1], self.board[3][2], self.board[3][3]
        )
    }
}

#[test]
fn test_new() {
    let grid = Grid::new();
    let mut count_2s = 0;
    for i in 0..4 {
        for j in 0..4 {
            if grid.board[i][j] == 2 {
                count_2s += 1;
            } 
        }
    }
    assert_eq!(count_2s, 2);
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
    let mut grid = Grid::new();
    grid.board = board;

    assert_eq!("4 4 0 0\n0 0 0 0\n0 0 0 0\n0 0 0 2", grid.to_string());
}

#[test]
fn test_slide() {
    let board =  [[4, 4, 2, 0], [4, 0, 2, 0], [0, 0, 0, 0], [0, 0, 2, 2]];
    let expected_left =  [[8, 2, 0, 0], [4, 2, 0, 0], [0, 0, 0, 0], [4, 0, 0, 0]];
    let expected_right =  [[0, 0, 8, 2], [0, 0, 4, 2], [0, 0, 0, 0], [0, 0, 0, 4]];
    let expected_up =  [[8, 4, 4, 2], [0, 0, 2, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    let expected_down =  [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, 0], [8, 4, 4, 2]];

    let mut grid_left = Grid::new();
    let mut grid_right = Grid::new();
    let mut grid_up = Grid::new();
    let mut grid_down = Grid::new();
    grid_left.board = board;
    grid_right.board = board;
    grid_up.board = board;
    grid_down.board = board;
    grid_left.slide(Direction::LEFT);
    grid_right.slide(Direction::RIGHT);
    grid_up.slide(Direction::UP);
    grid_down.slide(Direction::DOWN);

    assert_eq!(grid_left.board, expected_left);
    assert_eq!(grid_right.board, expected_right);
    assert_eq!(grid_up.board, expected_up);
    assert_eq!(grid_down.board, expected_down);


}