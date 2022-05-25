mod board;
mod life;
use life::GameOfLife;

fn main() {
    let dimensions = (10, 10);
    let n_points = 100;
    let iterations = 2000;

    let mut game_of_life = GameOfLife::new(dimensions, iterations, n_points);
    game_of_life.run();
}
