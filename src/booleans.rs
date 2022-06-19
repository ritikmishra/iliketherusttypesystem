use std::marker::PhantomData;

use crate::{numbers::{Number, Zero, Successor}, lists::StrRepr};

pub struct True;
pub struct False;

pub trait Bool {
    const VALUE: bool;
    type EquivalentInt: Number;
}
impl Bool for True {
    const VALUE: bool = true;
    type EquivalentInt = Successor<Zero>;
}
impl Bool for False {
    const VALUE: bool = false;
    type EquivalentInt = Zero;
}
impl StrRepr for True {
    fn str_repr() -> String {
        Self::VALUE.to_string()
    }
}
impl StrRepr for False {
    fn str_repr() -> String {
        Self::VALUE.to_string()
    }
}

pub trait Not {
    type Not: Bool;
}
impl Not for True {
    type Not = False;
}
impl Not for False {
    type Not = True;
}

pub trait Or<Other: Bool> {
    type Or: Bool;
}
/// True or anything is True
impl<B: Bool> Or<B> for True {
    type Or = True;
}
impl Or<True> for False {
    type Or = True;
}
impl Or<False> for False {
    type Or = False;
}

pub trait And<Other: Bool> {
    type And: Bool;
}
impl And<True> for True {
    type And = True;
}
impl And<False> for True {
    type And = False;
}
impl<B: Bool> And<B> for False {
    type And = False;
}

#[macro_export]
macro_rules! logic {
    (and $x:ty, $y:ty) => {
        <$x as $crate::booleans::And<$y>>::And
    };
    (or $x:ty, $y:ty) => {
        <$x as $crate::booleans::Or<$y>>::Or
    };
}

pub struct If<Cond, V1, V2>(PhantomData<Cond>, PhantomData<V1>, PhantomData<V2>);
pub trait IfOutput { type Output; }
impl<V1, V2> IfOutput for If<True, V1, V2> {
    type Output = V1;
}
impl<V1, V2> IfOutput for If<False, V1, V2> {
    type Output = V2;
}


#[macro_export]
macro_rules! typeif {
    ($cond:ty, $on_true:ty, $on_false:ty) => {
        <$crate::booleans::If<$cond, $on_true, $on_false> as $crate::booleans::IfOutput>::Output
    };
}

/// convert a regular bool into a funny type system bool
pub struct BoolWrapper<const VAL: bool>;
pub trait LiftBool {
    type Value: Bool;
}
impl LiftBool for BoolWrapper<true> {
    type Value = True;
}
impl LiftBool for BoolWrapper<false> {
    type Value = False;
}
