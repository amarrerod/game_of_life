use rand::prelude::*;
use std::collections::HashSet;
use std::fmt;

pub struct Board {
    _board: HashSet<(i32, i32)>,
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

    pub fn is_cell_alive(&self, cell: (i32, i32)) -> bool {
        self._board.contains(&cell)
    }

    // Updates the inner space with the new distribution of cells
    pub fn update(&mut self, space: HashSet<(i32, i32)>) {
        self._board = space
    }
    // Returns the neighbours of a given point
    pub fn neighbours(&self, point: (i32, i32)) -> usize {
        let (x, y) = point;

        let n = vec![(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)];
        let count = n.iter().filter(|x| self._board.contains(x)).count();
        count
    }

    pub fn get_cells(&self) -> &HashSet<(i32, i32)> {
        &self._board
    }
    // Creates a random number of alive cells inside the space
    pub fn create_random_points(&mut self, n: u32) {
        assert!(self.cols != 0);
        assert!(self.rows != 0);
        assert!(n <= self.rows * self.cols);
        let mut rng = rand::thread_rng();

        (0..self.cols as i32)
            .flat_map(|i| (0..self.rows as i32).map(move |j| (i, j)))
            .into_iter()
            .choose_multiple(&mut rng, n as usize)
            .into_iter()
            .for_each(|(i, j)| {
                self._board.insert((i, j));
            });
    }
    // Returns the number of alive cells in the space
    pub fn alive_cells(&self) -> usize {
        self._board.len()
    }

    pub fn get_character(&self, i: u32, j: u32) -> &str {
        if j == self.rows {
            return "\n";
        }
        match self._board.contains(&(i as i32, j as i32)) {
            true => "*",
            false => "#",
        }
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "=".repeat(self.cols as usize))?;

        (1..=self.cols)
            .flat_map(|i| (1..=self.rows).map(move |j| (i, j)))
            .for_each(|(i, j)| write!(f, "{}", self.get_character(i, j)).unwrap());

        writeln!(f, "{}\n", "=".repeat(self.cols as usize))
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
        assert_eq!(board.is_cell_alive((0, 0)), false);
    }

    #[test]
    fn check_update_space() {
        let mut board = create_board(10, 10);
        assert_eq!(board.is_cell_alive((0, 0)), false);
        let mut new_board = HashSet::new();
        new_board.insert((0, 0));
        board.update(new_board);
        assert_eq!(board.is_cell_alive((0, 0)), true);
    }

    #[test]
    fn test_create_random_board() {
        let mut board = create_board(100, 100);
        board.create_random_points(10);
        assert_eq!(board.alive_cells(), 10);
    }
}
