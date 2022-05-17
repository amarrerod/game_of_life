use std::collections::HashSet;

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
        assert!(x < self.cols);
        assert!(y < self.rows);
        self._board.contains(&(x, y))
    }

    pub fn update(&mut self, space: HashSet<(u32, u32)>) {
        self._board = space
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
}
