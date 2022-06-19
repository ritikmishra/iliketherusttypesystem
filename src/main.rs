#![feature(generic_associated_types, associated_type_bounds)]

use functions::Function;
use nqueens::NQueensSolution;
use numbers::N5;

mod booleans;
mod nqueens;
mod numbers;
mod lists;
mod functions;
mod gameoflife;

use lists::StrRepr;

fn main() {
    type Solution = <NQueensSolution as Function<N5>>::Apply;
    println!("{}", Solution::str_repr())
}
