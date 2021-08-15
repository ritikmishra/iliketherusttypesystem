use crate::{booleans::{Bool, False, True}, lists::{Cons, First, List, ListConcat, Nil}, numbers::{N1, N2, N7, PeanoAbsDiff, Range, Successor, Zero}};

pub trait Function<T> {
    type Apply;
}

pub trait Predicate<T>: Function<T> {}
impl<T, U: Function<T>> Predicate<T> for U where Self::Apply: Bool {}

pub trait Map<F> {
    type Output;
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

pub trait MapCat<F> {
    type Output;
}
impl<F> MapCat<F> for Nil {
    type Output = Nil;
}
impl<F, FirstList, RestList: MapCat<F>> MapCat<F> for Cons<FirstList, RestList>
where
    FirstList: Map<F>,
    <FirstList as Map<F>>::Output: List + ListConcat,
    <RestList as MapCat<F>>::Output: ListConcat,
{
    type Output =
        <<FirstList as Map<F>>::Output as ListConcat>::ConcatWith<<RestList as MapCat<F>>::Output>;
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

