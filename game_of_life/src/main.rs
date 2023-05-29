mod args;
mod board;
mod life;

use clap::Parser;
use life::GameOfLife;

fn main() {
    let args = args::Args::parse();

    let dimensions = (args.x, args.y);
    let iterations = args.i;
    let n_points = args.p;
    let verbosity = args.v;

    let mut game_of_life = GameOfLife::new(dimensions, iterations, n_points);
    let initial_cells = game_of_life.board.alive_cells();
    game_of_life.board.display();
    game_of_life.run(verbosity);
    println!(
        "Initial alive cells were: {} and final alive cells are: {}",
        initial_cells,
        game_of_life.board.alive_cells()
    );
}
