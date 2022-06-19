#![feature(generic_associated_types, associated_type_bounds)]
#![recursion_limit = "1024"]

use numbers::{Successor, N5};

use crate::{gameoflife::{Cell, Reify, working_regular_impl::print_board, SingleGOLIter}, numbers::{N0, N1, Negative as Neg, N3, N2, N4}};

mod booleans;
mod nqueens;
mod numbers;
mod lists;
mod functions;
mod gameoflife;

type N10 = m!(add N5, N5);
type N11 = Successor<N10>;
type N12 = Successor<N11>;
type N13 = Successor<N12>;
type N14 = Successor<N13>;
type N15 = Successor<N14>;
type N16 = Successor<N15>;
type N17 = Successor<N16>;
type N18 = Successor<N17>;
type N19 = Successor<N18>;
type N20 = Successor<N19>;
type N21 = Successor<N20>;
type N22 = Successor<N21>;
type N23 = Successor<N22>;
type N24 = Successor<N23>;
type N34 = m!(add N24, N10);
type N35 = Successor<N34>;

macro_rules! do_iter {
    ($i:expr, $t:ty, $out:ident) => {
        println!("###############################");
        println!("########### ITER {} ############", $i);
        println!("###############################");

        type $out = func_call!(SingleGOLIter[$t]);
        print_board(&$out::reify());
    };
}

fn main() {
    type GliderGun = make_list!(
        Cell<N0, N0>,
        Cell<N1, N0>,
        Cell<N0, N1>,
        Cell<N1, N1>,
        Cell<N10, N0>,
        Cell<N10, Neg<N1>>,
        Cell<N10, N1>,
        Cell<N11, Neg<N2>>,
        Cell<N12, Neg<N3>>,
        Cell<N13, Neg<N3>>,
        Cell<N11, N2>,
        Cell<N12, N3>,
        Cell<N13, N3>,
        Cell<N14, N0>,
        Cell<N15, N2>,
        Cell<N15, Neg<N2>>,
        Cell<N16, N0>,
        Cell<N16, N1>,
        Cell<N16, Neg<N1>>,
        Cell<N17, N0>,
        Cell<N20, N1>,
        Cell<N20, N2>,
        Cell<N20, N3>,
        Cell<N21, N1>,
        Cell<N21, N2>,
        Cell<N21, N3>,
        Cell<N22, N0>,
        Cell<N22, N4>,
        Cell<N24, N4>,
        Cell<N24, N5>,
        Cell<N24, N0>,
        Cell<N24, Neg<N1>>,
        Cell<N34, N2>,
        Cell<N34, N3>,
        Cell<N35, N2>,
        Cell<N35, N3>,
    );

    type Glider = make_list!(
        Cell<N0, N0>,
        Cell<N1, Neg<N1>>,
        Cell<N2, Neg<N1>>,
        Cell<N2, N0>,
        Cell<N2, N1>
    );
    
    do_iter!(1, Glider, Glider1);
    do_iter!(2, Glider1, Glider2);
    do_iter!(3, Glider2, Glider3);
    do_iter!(4, Glider3, Glider4);
    do_iter!(5, Glider4, Glider5);
}