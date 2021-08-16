#![feature(generic_associated_types, associated_type_bounds)]

use functions::Function;
use nqueens::NQueensSolution;
use numbers::N5;

mod booleans;
mod nqueens;
mod numbers;
mod lists;
mod functions;

fn main() {
    let x: <NQueensSolution as Function<N5>>::Apply = Default::default();
}
