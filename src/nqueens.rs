use std::marker::PhantomData;

use crate::{booleans::{Bool, False, Not, Or, True}, functions::{AnyTrue, Filter, Function, Map, FlatMap}, lists::{Cons, List, Nil, StrRepr}, numbers::{N0, N1, N2, N3, N4, N5, N8, Number, PeanoAbsDiff, PeanoEqual, PeanoLT, Range, Successor, Zero}};

#[derive(Default)]
pub struct Queen<X: Number, Y: Number>(PhantomData<X>, PhantomData<Y>);

impl<X: Number, Y: Number> StrRepr for Queen<X, Y> {
    fn str_repr() -> String {
        format!("Queen({}, {})", X::str_repr(), Y::str_repr())
    }
}

pub trait Threatens<QueenA, QueenB> {
    type Output: Bool;
}
impl<AX, AY, BX, BY> Threatens<Queen<AX, AY>, Queen<BX, BY>> for (Queen<AX, AY>, Queen<BX, BY>)
where
    AX: NumberOps<BX>,
    AY: NumberOps<BY>,
    BX: Number,
    BY: Number,
    // If ax = bx or ay = by, then queens can threaten each other
    <AX as PeanoEqual<BX>>::Equal: Or<<AY as PeanoEqual<BY>>::Equal>,
    // If |ax - bx| = |ay - by|, then queens can threaten each other diagonally
    <AX as PeanoAbsDiff<BX>>::AbsDiff: PeanoEqual<<AY as PeanoAbsDiff<BY>>::AbsDiff>,
    <<AX as PeanoAbsDiff<BX>>::AbsDiff as PeanoEqual<<AY as PeanoAbsDiff<BY>>::AbsDiff>>::Equal:
        Or<<<AX as PeanoEqual<BX>>::Equal as Or<<AY as PeanoEqual<BY>>::Equal>>::Or>,
{
    type Output = <<<AX as PeanoAbsDiff<BX>>::AbsDiff as PeanoEqual<
        <AY as PeanoAbsDiff<BY>>::AbsDiff,
    >>::Equal as Or<<<AX as PeanoEqual<BX>>::Equal as Or<<AY as PeanoEqual<BY>>::Equal>>::Or>>::Or;
}

fn threatens_type_test() {
    let _: <(Queen<N0, N0>, Queen<N0, N8>) as Threatens<_, _>>::Output = True;
    let _: <(Queen<N0, N0>, Queen<N4, N0>) as Threatens<_, _>>::Output = True;
    let _: <(Queen<N0, N0>, Queen<N4, N4>) as Threatens<_, _>>::Output = True;
    let _: <(Queen<N4, N4>, Queen<N0, N0>) as Threatens<_, _>>::Output = True;
    let _: <(Queen<N4, N3>, Queen<N0, N0>) as Threatens<_, _>>::Output = False;
    let _: <(Queen<N4, N3>, Queen<N1, N0>) as Threatens<_, _>>::Output = True;
    let _: <(Queen<N1, N0>, Queen<N4, N3>) as Threatens<_, _>>::Output = True;
}

pub struct Threatens1<ThisQueen>(PhantomData<ThisQueen>);
impl<ThisQueen, OtherQueen> Function<OtherQueen> for Threatens1<ThisQueen>
where
    (ThisQueen, OtherQueen): Threatens<ThisQueen, OtherQueen>,
{
    type Apply = <(ThisQueen, OtherQueen) as Threatens<ThisQueen, OtherQueen>>::Output;
}

pub trait Safe<ExistingQueens, NewQueen> {
    type Output: Bool;
}
impl<ExistingQueens: List, X: Number, Y: Number> Safe<ExistingQueens, Queen<X, Y>>
    for (ExistingQueens, Queen<X, Y>)
where
    ExistingQueens: Map<Threatens1<Queen<X, Y>>>,
    <ExistingQueens as Map<Threatens1<Queen<X, Y>>>>::Output: AnyTrue,
    <<ExistingQueens as Map<Threatens1<Queen<X, Y>>>>::Output as AnyTrue>::Output: Not,
{
    // It is safe to add a new queen if it is not threatened by any existing queen
    type Output =
        <<<ExistingQueens as Map<Threatens1<Queen<X, Y>>>>::Output as AnyTrue>::Output as Not>::Not;
}

/// Partial application of `Safe`
pub struct Safe1<Config>(PhantomData<Config>);
impl<Config, NewQueen> Function<NewQueen> for Safe1<Config>
where
    (Config, NewQueen): Safe<Config, NewQueen>,
{
    type Apply = <(Config, NewQueen) as Safe<Config, NewQueen>>::Output;
}

pub struct Queen1<X: Number>(PhantomData<X>);
impl<X: Number, Y: Number> Function<Y> for Queen1<X> {
    type Apply = Queen<X, Y>;
}

/// Return a list of queens with given x pos and y in [0, NumQueens)
pub trait QueensInRow<NumQueens: Number, XPos: Number> {
    type Output: List;
}
impl<NumQueens, XPos> QueensInRow<NumQueens, XPos> for (NumQueens, XPos)
where
    NumQueens: Number + Range,
    XPos: Number,
    <NumQueens as Range>::Range: Map<Queen1<XPos>>,
{
    type Output = <<NumQueens as Range>::Range as Map<Queen1<XPos>>>::Output;
}

/// List -> Item -> Cons<List, Item>
pub struct Prepend<Items: List>(PhantomData<Items>);
impl<I, L: List> Function<I> for Prepend<L> {
    type Apply = Cons<I, L>;
}

fn test_prepend() {
    // let _: <Prepend<Cons<N0, Cons<N0, Nil>>> as Function<N0>>::Apply = 0;
}

pub trait AddQueen<N: Number, X: Number, ExistingQueens: List> {
    type Output;
}
impl<N: Number, X: Number, ExistingQueens: List> AddQueen<N, X, ExistingQueens> for ExistingQueens
where
    (N, X): QueensInRow<N, X>,
    <(N, X) as QueensInRow<N, X>>::Output: Filter<Safe1<ExistingQueens>>,
    <<(N, X) as QueensInRow<N, X>>::Output as Filter<Safe1<ExistingQueens>>>::Output:
        Map<Prepend<ExistingQueens>>,
{
    type Output =
        <<<(N, X) as QueensInRow<N, X>>::Output as Filter<Safe1<ExistingQueens>>>::Output as Map<
            Prepend<ExistingQueens>,
        >>::Output;
}

fn test_add_queen() {
    // let _: <Nil as AddQueen<N3, N0, _>>::Output = 3;
}

pub struct AddQueen2<N: Number, X: Number>(PhantomData<N>, PhantomData<X>);
impl<N: Number, X: Number, ExistingQueens: List + AddQueen<N, X, ExistingQueens>>
    Function<ExistingQueens> for AddQueen2<N, X>
{
    type Apply = <ExistingQueens as AddQueen<N, X, ExistingQueens>>::Output;
}

fn test_add_queen2() {
    // let _: <AddQueen2<N3, N0> as Function<Nil>>::Apply = 3;
}

pub trait AddQueenToAll<N: Number, X: Number, Configs: List> {
    type Output;
}
impl<N: Number, X: Number, Configs: List> AddQueenToAll<N, X, Configs> for Configs
where
    Configs: FlatMap<AddQueen2<N, X>>,
{
    // For each configuration, get valid configurations with a queen on row X
    // And concatenate together the list of list of configurations, resulting in a
    // list of configurations (list of list of queens)
    type Output = <Configs as FlatMap<AddQueen2<N, X>>>::Output;
}

struct CheckIsList;
impl<L: List> Function<L> for CheckIsList {
    type Apply = L;
}

fn test_add_queen_to_all() {
    // List of List of Queens
    type ListOfConfigurations = <AddQueen2<N4, N0> as Function<Nil>>::Apply;
    // ListOfConfigurations is indeed a list of lists
    // let _: <ListOfConfigurations as Map<CheckIsList>>::Output = 3;
    // let _: ListOfConfigurations = 3;
    // let _: <ListOfConfigurations as AddQueenToAll<N4, N1, _>>::Output ;
}

pub trait AddQueensIf<Predicate, N, X, Configs> {
    type Output;
}

impl<N, X, Configs> AddQueensIf<False, N, X, Configs> for Configs {
    type Output = Configs;
}
impl<N: Number, X: Number, Configs: List> AddQueensIf<True, N, X, Configs> for Configs
where
    Configs: AddQueenToAll<N, X, Configs>,
    <Configs as AddQueenToAll<N, X, Configs>>::Output:
        AddQueens<N, Successor<X>, <Configs as AddQueenToAll<N, X, Configs>>::Output>,
{
    type Output = <<Configs as AddQueenToAll<N, X, Configs>>::Output as AddQueens<
        N,
        Successor<X>,
        <Configs as AddQueenToAll<N, X, Configs>>::Output,
    >>::Output;
}

/// Given an N x N board, and a list of possible configurations
/// Get configurations where queen on rows [X, N) have been placed down
pub trait AddQueens<N, X, Configs> {
    type Output;
}
impl<N: Number, X, Configs> AddQueens<N, X, Configs> for Configs
where
    X: PeanoLT<N>,
    Configs: AddQueensIf<<X as PeanoLT<N>>::LT, N, X, Configs>,
{
    type Output = <Configs as AddQueensIf<<X as PeanoLT<N>>::LT, N, X, Configs>>::Output;
}

pub struct NQueensSolution;
impl<N: Number> Function<N> for NQueensSolution
where
    Zero: PeanoLT<N>,
    Cons<Nil, Nil>: AddQueensIf<<Zero as PeanoLT<N>>::LT, N, Zero, Cons<Nil, Nil>>,
{
    type Apply = <Cons<Nil, Nil> as AddQueens<N, Zero, Cons<Nil, Nil>>>::Output;
}

pub trait NumberOps<N: Number>: Number + PeanoEqual<N> + PeanoAbsDiff<N> {}
impl<N1: Number, N2: Number + PeanoEqual<N1> + PeanoAbsDiff<N1>> NumberOps<N1> for N2 {}
