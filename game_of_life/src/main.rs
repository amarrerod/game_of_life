mod board;
mod life;
use life::GameOfLife;

fn main() {
    let dimensions = (100, 100);
    let n_points = 10000;
    let iterations = 10;

    let mut game_of_life = GameOfLife::new(dimensions, iterations, n_points);
    let initial_cells = game_of_life.board.alive_cells();
    game_of_life.board.display();
    game_of_life.run(true);
    println!(
        "Initial alive cells were: {} and final alive cells are: {}",
        initial_cells,
        game_of_life.board.alive_cells()
    );
}
