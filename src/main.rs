#![feature(generic_associated_types, associated_type_bounds)]

mod booleans;
mod numbers;
mod lists;
mod functions;
use lists::{Cons, Nil, ListConcat};

#[derive(Default)]
struct NC<const FOO: usize>;

fn main() {
    type OneTwoThree = Cons<NC<1>, Cons<NC<2>, Cons<NC<3>, Nil>>>;
    type FourFive = Cons<NC<4>, Cons<NC<5>, Nil>>;
    type Concated = <OneTwoThree as ListConcat>::ConcatWith<FourFive>;
    // let foo: Concated = 4;
}
