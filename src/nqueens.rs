use std::{marker::PhantomData, thread::Thread};

use crate::{booleans::{Bool, False, Not, Or, True}, functions::{AnyTrue, Apply, Function, Map}, lists::List, numbers::{Number, PeanoAbsDiff, PeanoEqual, N0, N1, N3, N4, N8}};

struct Queen<X: Number, Y: Number>(PhantomData<X>, PhantomData<Y>);

trait Threatens<QueenA, QueenB> {
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

struct Threatens1<ThisQueen>(PhantomData<ThisQueen>);
impl<ThisQueen, OtherQueen> Function<OtherQueen> for Threatens1<ThisQueen>
where
    (ThisQueen, OtherQueen): Threatens<ThisQueen, OtherQueen>,
{
    type Apply = <(ThisQueen, OtherQueen) as Threatens<ThisQueen, OtherQueen>>::Output;
}

trait Safe<ExistingQueens, NewQueen> {
    type Output: Bool;
}
impl<ExistingQueens: List, X: Number, Y: Number> Safe<ExistingQueens, Queen<X, Y>>
    for (ExistingQueens, Queen<X, Y>)
where
    ExistingQueens: Map<Threatens1<Queen<X, Y>>>,
    <ExistingQueens as Map<Threatens1<Queen<X, Y>>>>::Output: AnyTrue,
    <<ExistingQueens as Map<Threatens1<Queen<X, Y>>>>::Output as AnyTrue>::Output: Not
{
    // It is safe to add a new queen if it is not threatened by any existing queen 
    type Output = <<<ExistingQueens as Map<Threatens1<Queen<X, Y>>>>::Output as AnyTrue>::Output as Not>::Not;
}

struct Safe1<Config>(PhantomData<Config>);
impl<Config, NewQueen> Function<NewQueen> for Safe1<Config> 
where 
    (Config, NewQueen): Safe<Config, NewQueen>
{
    type Apply = <(Config, NewQueen) as Safe<Config, NewQueen>>::Output;
}


trait NumberOps<N: Number>: Number + PeanoEqual<N> + PeanoAbsDiff<N> {}
impl<N1: Number, N2: Number + PeanoEqual<N1> + PeanoAbsDiff<N1>> NumberOps<N1> for N2 {}
