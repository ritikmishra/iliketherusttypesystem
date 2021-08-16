use std::marker::PhantomData;

use crate::{
    booleans::{Bool, False, True},
    functions::Function,
    lists::{Cons, List, Nil},
};

#[derive(Default)]
pub struct Zero;
#[derive(Default)]
pub struct Successor<N>(PhantomData<N>);

pub trait Number {
    const VALUE: usize;
}
impl Number for Zero {
    const VALUE: usize = 0;
}
impl<N: Number> Number for Successor<N> {
    const VALUE: usize = N::VALUE + 1;
}

pub type N0 = Zero;
pub type N1 = Successor<N0>;
pub type N2 = Successor<N1>;
pub type N3 = Successor<N2>;
pub type N4 = Successor<N3>;
pub type N5 = Successor<N4>;
pub type N6 = Successor<N5>;
pub type N7 = Successor<N6>;
pub type N8 = Successor<N7>;

pub trait PeanoEqual<OtherNumber: Number> {
    type Equal: Bool;
}
/// 0 == 0
impl PeanoEqual<Zero> for Zero {
    type Equal = True;
}
/// 0 != n + 1 for all n >= 0
impl<N: Number> PeanoEqual<Successor<N>> for Zero {
    type Equal = False;
}
/// n + 1 != 0 for all n >= 0
impl<N: Number> PeanoEqual<Zero> for Successor<N> {
    type Equal = False;
}
/// n1 + 1 == n2 + 1 if and only if n1 == n2
impl<N: Number, M: Number + PeanoEqual<N>> PeanoEqual<Successor<M>> for Successor<N> {
    type Equal = <M as PeanoEqual<N>>::Equal;
}

pub trait PeanoLT<OtherNumber: Number> {
    type LT: Bool;
}
/// 0 < 0 is false
impl PeanoLT<Zero> for Zero {
    type LT = False;
}
/// 0 < n + 1 is true for any n >= 0
/// Zero::PeanoLT<Successor<N>>
impl<N: Number> PeanoLT<Successor<N>> for Zero {
    type LT = True;
}
/// n + 1 < 0 is false for any n >= 0
impl<N: Number> PeanoLT<Zero> for Successor<N> {
    type LT = False;
}
/// n + 1 < m + 1 if and only if n < m
impl<M: Number, N: Number + PeanoLT<M>> PeanoLT<Successor<M>> for Successor<N> {
    // <Successor<N> as PeanoLT<Successor<M>> = <N as PeanoLT<M>>
    type LT = <N as PeanoLT<M>>::LT;
}

pub trait PeanoAbsDiff<OtherNumber: Number> {
    type AbsDiff: Number;
}
/// |0 - 0| = 0
impl PeanoAbsDiff<Zero> for Zero {
    type AbsDiff = Zero;
}
/// |0 - (n + 1)| = n + 1
impl<N: Number> PeanoAbsDiff<Successor<N>> for Zero {
    type AbsDiff = Successor<N>;
}
/// |n + 1 - 0| = n + 1
impl<N: Number> PeanoAbsDiff<Zero> for Successor<N> {
    type AbsDiff = Successor<N>;
}
/// |(n + 1) - (m + 1)| = |n - m|
impl<N: Number, M: Number + PeanoAbsDiff<N>> PeanoAbsDiff<Successor<M>> for Successor<N> {
    type AbsDiff = <M as PeanoAbsDiff<N>>::AbsDiff;
}

pub trait Range {
    type Range: List;
}
impl Range for Zero {
    type Range = Nil;
}
/// Returns list that counts down from N-1. The list has N elements in it;
impl<N: Number + Range> Range for Successor<N> {
    type Range = Cons<N, <N as Range>::Range>;
}

pub struct RangeFn;
impl<N: Range> Function<N> for RangeFn {
    type Apply = N::Range;
}
