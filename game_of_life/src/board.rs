use rand::Rng;
use std::collections::HashSet;
use std::fmt;

pub struct Board {
    _board: HashSet<(u32, u32)>,
    pub cols: u32,
    pub rows: u32,
}

impl Board {
    /// Creates a new board
    pub fn new(c: u32, r: u32) -> Board {
        Board {
            cols: c,
            rows: r,
            _board: HashSet::new(),
        }
    }
    /// Checks whether the Cell in the position (x, y) is alive
    pub fn is_alive(&self, x: u32, y: u32) -> bool {
        assert!(x < (self.cols + 1));
        assert!(y < (self.rows + 1));
        self._board.contains(&(x, y))
    }

    pub fn is_cell_alive(&self, cell: (u32, u32)) -> bool {
        self._board.contains(&cell)
    }

    // Updates the inner space with the new distribution of cells
    pub fn update(&mut self, space: HashSet<(u32, u32)>) {
        self._board = space
    }
    // Returns the neighbours of a given point
    pub fn neighbours(&self, point: (u32, u32)) -> Vec<(u32, u32)> {
        let (x, y) = point;
        vec![(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
    }

    pub fn get_cells(&self) -> &HashSet<(u32, u32)> {
        &self._board
    }
    // Creates a random number of alive cells inside the space
    pub fn create_random_points(&mut self, n: u32) {
        assert!(self.cols != 0);
        assert!(self.rows != 0);
        let mut rng = rand::thread_rng();
        for i in 0..n {
            let x = rng.gen_range(1..self.cols + 1);
            let y = rng.gen_range(1..self.rows + 1);
            self._board.insert((x, y));
        }
    }
    // Returns the number of alive cells in the space
    pub fn alive_cells(&self) -> usize {
        self._board.len()
    }

    pub fn display(&self) {
        println!("{}", "=".repeat(self.cols as usize));
        for i in 1..self.cols + 1 {
            for j in 1..self.rows + 1 {
                match self._board.contains(&(i, j)) {
                    true => print!("@"),
                    false => print!("#"),
                }
            }
            println!("");
        }
        println!("{}\n", "=".repeat(self.cols as usize));
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The number of alive cells in the space is: {}",
            self._board.len(),
        )?;
        write!(f, "\n{:?}", self._board)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn create_board(x: u32, y: u32) -> Board {
        Board::new(x, y)
    }

    #[test]
    fn create_empty_board() {
        let board = create_board(0, 0);
        assert_eq!(board.cols, 0);
        assert_eq!(board.rows, 0);
    }
    #[test]
    fn create_10_by_10_board() {
        let board = create_board(10, 10);
        assert_eq!(board.cols, 10);
        assert_eq!(board.rows, 10);
    }

    #[test]
    fn check_point_in_board() {
        let board = create_board(10, 10);
        assert_eq!(board.is_alive(0, 0), false);
    }

    #[test]
    fn check_update_space() {
        let mut board = create_board(10, 10);
        assert_eq!(board.is_alive(0, 0), false);
        let mut new_board = HashSet::new();
        new_board.insert((0, 0));
        board.update(new_board);
        assert_eq!(board.is_alive(0, 0), true);
    }

    #[test]
    fn test_create_random_board() {
        let mut board = create_board(100, 100);
        board.create_random_points(10);
        assert_eq!(board.alive_cells(), 10);
    }
}
