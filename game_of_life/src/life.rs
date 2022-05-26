use crate::board::Board;
use std::collections::HashSet;
use std::fmt;

pub struct GameOfLife {
    pub board: Board,
    pub iterations: u32,
    pub initial_points: u32,
}

impl GameOfLife {
    pub fn new(dimensions: (u32, u32), iterations: u32, initial_points: u32) -> GameOfLife {
        let (x, y) = dimensions;
        let mut board = Board::new(x, y);
        board.create_random_points(initial_points);
        GameOfLife {
            board,
            iterations,
            initial_points,
        }
    }

    pub fn run(&mut self, verbose: bool) {
        for _ in 0..self.iterations {
            let mut new_board: HashSet<(i32, i32)> = HashSet::new();

            for cell in self.board.get_cells() {
                let count = self.board.neighbours(*cell);
                if count == 3 && !self.board.is_cell_alive(*cell) {
                    new_board.insert(*cell);
                } else if (count == 2 || count == 3) && self.board.is_cell_alive(*cell) {
                    new_board.insert(*cell);
                }
            }
            self.board.update(new_board);
            if verbose {
                self.board.display();
            }
        }
    }
}

impl fmt::Debug for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let separator = "=".repeat(100);
        write!(f, "GAME OF LIFE - Conway's\n")?;
        write!(f, "{}", separator)?;
        write!(f, "\n Running for {} iterations", self.iterations)?;
        write!(f, "\n With initially {} cells alive", self.initial_points)?;
        write!(f, "\n With the following space: \n{:?}", self.board)
    }
}
