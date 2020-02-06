use rand::Rng;

enum Direction {
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

        let mut board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        
        let mut rng = rand::thread_rng();
        
        let (pos1x, pos1y) = (rng.gen_range(0, 4), rng.gen_range(0,4));
        let (mut pos2x, mut pos2y) = (rng.gen_range(0, 4), rng.gen_range(0,4));

        while pos1x == pos2x && pos1y == pos2y {
            pos2x = rng.gen_range(0, 4);
            pos2y = rng.gen_range(0, 4);
        }
      
        board[pos1y][pos1x] = 2;
        board[pos2y][pos2x] = 2;

        Grid {
            board,
        }
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