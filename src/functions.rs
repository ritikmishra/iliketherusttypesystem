use crate::{
    booleans::{Bool, False, True},
    lists::{Cons, List, ListConcat, Nil},
    numbers::{PeanoAbsDiff, Successor, Zero, N1, Number, GEQZero, Negative, GTZero},
};

pub trait Function<T> {
    type Apply;
}

pub trait Predicate<T>: Function<T> {
    type BoolApply: Bool;
}
impl<T, U: Function<T>> Predicate<T> for U where <Self as Function<T>>::Apply: Bool {
    type BoolApply = <Self as Function<T>>::Apply;
}

pub trait Map<F> {
    type Output: List;
}
impl<F> Map<F> for Nil {
    type Output = Nil;
}
impl<F: Function<X>, X, XS> Map<F> for Cons<X, XS>
where
    XS: Map<F>,
{
    type Output = Cons<F::Apply, <XS as Map<F>>::Output>;
}

pub trait Filter<F> {
    type Output;
}
impl<F> Filter<F> for Nil {
    type Output = Nil;
}
impl<F: Predicate<X>, X, XS: Filter<F>> Filter<F> for Cons<X, XS>
where
    <F as Function<X>>::Apply: Bool,
    <XS as Filter<F>>::Output: PrependIf<F::Apply, X>,
{
    type Output = <<XS as Filter<F>>::Output as PrependIf<F::Apply, X>>::Output;
}

/// Given a list of lists, map each list using the function, and concat the results together
pub trait FlatMap<F> {
    type Output;
}
impl<F> FlatMap<F> for Nil {
    type Output = Nil;
}
impl<FirstItem, F: Function<FirstItem>, RestItems: FlatMap<F>> FlatMap<F>
    for Cons<FirstItem, RestItems>
where
    <F as Function<FirstItem>>::Apply: List + ListConcat,
    <RestItems as FlatMap<F>>::Output: ListConcat,
{
    type Output = <<F as Function<FirstItem>>::Apply as ListConcat>::ConcatWith<
        <RestItems as FlatMap<F>>::Output,
    >;
}

pub trait PrependIf<Predicate: Bool, Item> {
    type Output;
}
impl<Item, L: List> PrependIf<True, Item> for L {
    type Output = Cons<Item, Self>;
}
impl<Item, L: List> PrependIf<False, Item> for L {
    type Output = Self;
}

pub struct Increment;
impl<I: Number + GEQZero> Function<I> for Increment {
    type Apply = Successor<I>;
}
impl<I: Number + GTZero> Function<Negative<Successor<I>>> for Increment {
    type Apply = Negative<I>;
}
/// -1 + 1 = 0
impl Function<Negative<Successor<Zero>>> for Increment {
    type Apply = Zero;
}

struct Decrement;
impl<I: PeanoAbsDiff<N1>> Function<I> for Decrement {
    type Apply = <I as PeanoAbsDiff<N1>>::AbsDiff;
}

struct IsZero;
impl Function<Zero> for IsZero {
    type Apply = True;
}
impl<N: GEQZero> Function<Successor<N>> for IsZero {
    type Apply = False;
}
impl<N: GTZero> Function<Negative<N>> for IsZero {
    type Apply = False;
}

pub trait AnyTrue {
    type Output: Bool;
}
impl AnyTrue for Nil {
    type Output = False;
}
impl<L: List> AnyTrue for Cons<True, L> {
    type Output = True;
}
impl<L: List + AnyTrue> AnyTrue for Cons<False, L> {
    type Output = <L as AnyTrue>::Output;
}

#[macro_export]
macro_rules! func_call {
    ($name:ty[$param:ty]) => {
        <$name as $crate::functions::Function<$param>>::Apply
    };
    ($name:ty[$($param:ty),+]) => {
        <$name as $crate::functions::Function<($($param,)+)>>::Apply
    };
}

#[macro_export]
macro_rules! pred_call {
    ($name:ty[$param:ty]) => {
        <$name as $crate::functions::Predicate<$param>>::BoolApply
    };
    ($name:ty[$($param:ty),+]) => {
        <$name as $crate::functions::Predicate<($($param,)+)>>::BoolApply
    };
}