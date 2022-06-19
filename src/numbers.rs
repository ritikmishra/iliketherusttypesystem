use std::marker::PhantomData;

use crate::{
    booleans::{Bool, False, True},
    functions::Function,
    lists::{Cons, List, Nil, StrRepr},
};

/// implemented for N >= 0
pub trait GEQZero {}
/// implemented for N > 0
pub trait GTZero: GEQZero {}


#[derive(Default)]
pub struct Zero;
impl GEQZero for Zero {}

#[derive(Default)]
pub struct Successor<N: GEQZero>(PhantomData<N>);
impl<N: GEQZero> GEQZero for Successor<N> {}
impl<N: GEQZero> GTZero for Successor<N> {}

#[derive(Default)]
pub struct Negative<N: GTZero>(PhantomData<N>);

pub trait Number {
    const VALUE: isize;
}
impl Number for Zero {
    const VALUE: isize = 0;
}
impl<N: Number + GEQZero> Number for Successor<N> {
    const VALUE: isize = N::VALUE + 1;
}
impl<N: Number + GTZero> Number for Negative<N> {
    const VALUE: isize = -(N::VALUE);
}

impl<N: Number> StrRepr for N {
    fn str_repr() -> String {
        format!("{}", N::VALUE)
    }
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

#[macro_export]
macro_rules! m {
    (eq $N:ty, $M:ty) => {
        <$N as $crate::numbers::PeanoEqual<$M>>::Equal
    };
    (add $N:ty, $M:ty) => {
        <$N as $crate::numbers::PeanoAdd<$M>>::Val
    };
    (lt $N:ty, $M:ty) => {
        <$N as $crate::numbers::PeanoLT<$M>>::LT
    };
}

pub trait PeanoEqual<OtherNumber: Number> {
    type Equal: Bool;
}
/// 0 == 0
impl PeanoEqual<Zero> for Zero {
    type Equal = True;
}
/// 0 != n + 1 for all n >= 0
impl<N: Number + GEQZero> PeanoEqual<Successor<N>> for Zero {
    type Equal = False;
}
/// -n != 0 because n >= 1 is guaranteed
impl<N: Number + GTZero> PeanoEqual<Negative<N>> for Zero
{
    type Equal = False;
}

/// n + 1 != 0 for all n >= 0
impl<N: Number + GEQZero> PeanoEqual<Zero> for Successor<N> {
    type Equal = False;
}
/// n1 + 1 == n2 + 1 if and only if n1 == n2
impl<N: Number + GEQZero, M: Number + PeanoEqual<N> + GEQZero> PeanoEqual<Successor<M>> for Successor<N> {
    type Equal = m!(eq M, N);
}
/// -m != n
impl<N: Number + GEQZero, M: Number + PeanoAdd<N> + GTZero> PeanoEqual<Negative<M>> for Successor<N>
{
    type Equal = False;
}

/// -(n + 1) != 0 for all n >= 0
impl<N: Number + GTZero> PeanoEqual<Zero> for Negative<N> {
    type Equal = False;
}
/// -n == -m if n == m
impl<N: Number + GTZero, M: Number + PeanoEqual<N> + GTZero> PeanoEqual<Negative<M>> for Negative<N> {
    type Equal = m!(eq M, N);
}
/// -n == m + 1 never true since -n < 0 and m+1 >= 1
impl<N: Number + GTZero, M: Number + PeanoAdd<N> + GEQZero> PeanoEqual<Successor<M>> for Negative<N>
{
    type Equal = False;
}

/// peano add
pub trait PeanoAdd<OtherNumber: Number> {
    type Val: Number;
}
/// N + 0 = 0
impl<N: Number> PeanoAdd<N> for Zero {
    type Val = N;
}
/// (N + 1) + M = (N + M) + 1
impl<N: Number + PeanoAdd<M> + GEQZero, M: Number + GEQZero> PeanoAdd<M> for Successor<N> 
where 
    <N as PeanoAdd<M>>::Val: GEQZero
{
    type Val = Successor<m!(add N, M)>;
}
/// -m + n + 1
impl<N: Number + GEQZero, M: Number + GTZero> PeanoAdd<Negative<M>> for Successor<N> 
where 
    Negative<M>: PeanoAdd<Self>
{
    type Val = m!(add Negative<M>, Self);
}

// -(n + 1) + m + 1 = -n - 1 + m + 1 = -n + m
impl<N: Number + GTZero, M: Number + GEQZero> PeanoAdd<Successor<M>> for Negative<Successor<N>>
where
    Negative<N>: PeanoAdd<M>
{
    type Val = m!(add Negative<N>, M);
}

/// -1 + m + 1 = m
impl<M: Number + GEQZero> PeanoAdd<Successor<M>> for Negative<Successor<Zero>> {
    type Val = M;
}
/// -m + (-n) = -(m + n)
impl<M: Number + GTZero + PeanoAdd<N>, N: Number + GTZero> PeanoAdd<Negative<M>> for Negative<N>
where
    <M as PeanoAdd<N>>::Val: GTZero
{
    type Val = Negative<m!(add M, N)>;
}
impl<N: Number + GTZero> PeanoAdd<Zero> for Negative<N>
{
    type Val = Self;
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
impl<N: Number + GEQZero> PeanoLT<Successor<N>> for Zero {
    type LT = True;
}
/// n + 1 < 0 is false for any n >= 0
impl<N: Number + GEQZero> PeanoLT<Zero> for Successor<N> {
    type LT = False;
}
/// n + 1 < m + 1 if and only if n < m
impl<M: Number + GEQZero, N: Number + PeanoLT<M> + GEQZero> PeanoLT<Successor<M>> for Successor<N> {
    type LT = m!(lt N, M);
}

pub trait PeanoAbsDiff<OtherNumber: Number> {
    type AbsDiff: Number;
}
/// |0 - 0| = 0
impl PeanoAbsDiff<Zero> for Zero {
    type AbsDiff = Zero;
}
/// |0 - (n + 1)| = n + 1
impl<N: Number + GEQZero> PeanoAbsDiff<Successor<N>> for Zero {
    type AbsDiff = Successor<N>;
}
/// |n + 1 - 0| = n + 1
impl<N: Number + GEQZero> PeanoAbsDiff<Zero> for Successor<N> {
    type AbsDiff = Successor<N>;
}
/// |(n + 1) - (m + 1)| = |n - m|
impl<N: Number + GEQZero, M: Number + PeanoAbsDiff<N> + GEQZero> PeanoAbsDiff<Successor<M>> for Successor<N> {
    type AbsDiff = <M as PeanoAbsDiff<N>>::AbsDiff;
}

pub trait Range {
    type Range: List;
}
impl Range for Zero {
    type Range = Nil;
}
/// Returns list that counts down from N-1. The list has N elements in it;
impl<N: Number + Range + GEQZero> Range for Successor<N> {
    type Range = Cons<N, <N as Range>::Range>;
}

pub struct RangeFn;
impl<N: Range> Function<N> for RangeFn {
    type Apply = N::Range;
}

#[cfg(test)]
mod test {
    use crate::numbers::{Negative, Number, Successor, Zero, N1};

    #[test]
    fn test() {
        type NEG1 = Negative<N1>;
        type ZERO = m!(add NEG1, Successor<Zero>);
        type ONE = m!(add ZERO, Successor<Zero>);

        println!("{:?}", std::any::type_name::<NEG1>());

        assert_eq!(NEG1::VALUE, -1);
        assert_eq!(ZERO::VALUE, 0);
        assert_eq!(ONE::VALUE, 1);
    }

    #[test]
    fn test_adding_neg_nums() {
        type NEG1 = Negative<N1>;

        assert_eq!(-2, <m!(add NEG1, NEG1)>::VALUE);
    }

    use crate::lists::StrRepr;
    #[test]
    fn test_equality() {
        type FUNNY_ZERO = Successor<Negative<N1>>;
        type NEG_2 = Successor<Negative<Successor<Zero>>>;

        type wut = m!(eq FUNNY_ZERO, NEG_2);

        
    }
}
