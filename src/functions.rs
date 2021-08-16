use crate::{
    booleans::{Bool, False, True},
    lists::{Cons, List, ListConcat, Nil},
    numbers::{PeanoAbsDiff, Successor, Zero, N1},
};

pub trait Function<T> {
    type Apply;
}

pub trait Predicate<T>: Function<T> {}
impl<T, U: Function<T>> Predicate<T> for U where Self::Apply: Bool {}

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

struct Increment;
impl<I> Function<I> for Increment {
    type Apply = Successor<I>;
}

struct Decrement;
impl<I: PeanoAbsDiff<N1>> Function<I> for Decrement {
    type Apply = <I as PeanoAbsDiff<N1>>::AbsDiff;
}

struct IsZero;
impl Function<Zero> for IsZero {
    type Apply = True;
}
impl<N> Function<Successor<N>> for IsZero {
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
