//! implementation of conway's game of life inside the type system

use std::marker::PhantomData;

use crate::{
    booleans::{False, If, IfOutput, True, And, Or},
    functions::{Function, Map, Predicate},
    lists::{Cons, List, Nil, StrRepr, ListConcatAll},
    m, make_list,
    numbers::{Negative, Number, PeanoAdd, Successor, Zero, N1, PeanoEqual, GEQZero, N3, N2},
    typeif, logic,
    functions::{Increment, Filter}, func_call, pred_call
};

#[cfg(test)]
#[allow(unused)]
mod working_regular_impl {
    use std::{collections::HashMap, convert::TryInto};

    type Cell = (i32, i32);

    fn iterate(active_cells: &mut Vec<Cell>) {
        fn get_cell_neighbors(cell: Cell) -> Vec<Cell> {
            let mut ret = Vec::new();

            let (x, y) = cell;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i != 0 || j != 0 {
                        ret.push((x + i, y + j));
                    }
                }
            }

            ret
        }

        let mut neighbor_counts: HashMap<Cell, i32> = HashMap::new();
        for cell in active_cells.iter().copied() {
            for neighbor in get_cell_neighbors(cell) {
                *neighbor_counts.entry(neighbor).or_insert(0) += 1
            }
        }

        *active_cells = neighbor_counts
            .into_iter()
            .filter(|(cell, count)| *count == 3 || (active_cells.contains(cell) && *count == 2))
            .map(|(cell, _)| cell)
            .collect();
    }

    fn print_board(board: &Vec<Cell>) -> Option<()> {
        let smallest_x = board.iter().copied().map(|(x, y)| x).min()?;
        let largest_x = board.iter().copied().map(|(x, y)| x).max()?;
        let smallest_y = board.iter().copied().map(|(x, y)| y).min()?;
        let largest_y = board.iter().copied().map(|(x, y)| y).max()?;

        for i in smallest_y..=largest_y {
            for j in smallest_x..=largest_x {
                if board.contains(&(j, i)) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            print!("\n");
        }

        Some(())
    }
}

#[derive(Default)]
pub struct Cell<X: Number, Y: Number>(PhantomData<X>, PhantomData<Y>);

impl<X: Number, Y: Number> StrRepr for Cell<X, Y> {
    fn str_repr() -> String {
        format!("cell({}, {})", X::VALUE, Y::VALUE)
    }
}

#[derive(Default)]
pub struct Delta<X: Number, Y: Number>(PhantomData<X>, PhantomData<Y>);
type Neg1 = Negative<Successor<Zero>>;
type Pos1 = Successor<Zero>;
type NeighborlyDeltas = make_list!(
    Delta<Neg1, Neg1>,
    Delta<Neg1, Zero>,
    Delta<Neg1, Pos1>,

    Delta<Zero, Neg1>,
    Delta<Zero, Pos1>,

    Delta<Pos1, Neg1>,
    Delta<Pos1, Zero>,
    Delta<Pos1, Pos1>,
);

struct ApplyDeltaToCell<T>(PhantomData<T>);
impl<X: Number, Y: Number, D1: Number, D2: Number> Function<Delta<D1, D2>>
    for ApplyDeltaToCell<Cell<X, Y>>
where
    X: PeanoAdd<D1>,
    Y: PeanoAdd<D2>,
{
    type Apply = Cell<m!(add X, D1), m!(add Y, D2)>;
}

struct GetCellNeighbors;
impl<X: Number, Y: Number> Function<Cell<X, Y>> for GetCellNeighbors
where
    X: PeanoAdd<Neg1>,
    X: PeanoAdd<Zero>,
    X: PeanoAdd<Pos1>,
    Y: PeanoAdd<Neg1>,
    Y: PeanoAdd<Zero>,
    Y: PeanoAdd<Pos1>,
{
    type Apply = <NeighborlyDeltas as Map<ApplyDeltaToCell<Cell<X, Y>>>>::Output;
}

struct CellComparisonFunction;
impl<Cell1X: Number, Cell1Y: Number, Cell2X: Number, Cell2Y: Number> Function<
    (Cell<Cell1X, Cell1Y>,
    Cell<Cell2X, Cell2Y>)
> for CellComparisonFunction
where 
        Cell1X: PeanoEqual<Cell2X>,
        Cell1Y: PeanoEqual<Cell2Y>,
        <Cell1X as PeanoEqual<Cell2X>>::Equal: And<<Cell1Y as PeanoEqual<Cell2Y>>::Equal>
{
    type Apply = logic!(and
        m!(eq Cell1X, Cell2X),
        m!(eq Cell1Y, Cell2Y)
    );
}

pub trait IncrementItemCounter<K, Cmp> {
    type Output: List;
}

impl<K, Cmp> IncrementItemCounter<K, Cmp> for Nil {
    type Output = Cons<(K, N1), Nil>;
}

impl<K1, K2, N: GEQZero + Number, XS, Cmp> IncrementItemCounter<K1, Cmp> for Cons<(K2, N), XS>
where
    Cmp: Predicate<(K1, K2)>,
    XS: IncrementItemCounter<K1, Cmp>,
    <If<<Cmp as Predicate<(K1, K2)>>::BoolApply, Cons<(K2, Successor<N>), XS>, Cons<(K2, N), <XS as IncrementItemCounter<K1, Cmp>>::Output>> as IfOutput>::Output: List,
    If<<Cmp as Predicate<(K1, K2)>>::BoolApply, Cons<(K2, Successor<N>), XS>, Cons<(K2, N), <XS as IncrementItemCounter<K1, Cmp>>::Output>>: IfOutput
{
    type Output = typeif!(
        pred_call!(Cmp[K1, K2]),
        Cons<(K2, func_call!(Increment[N])), XS>,
        Cons<(K2, N), <XS as IncrementItemCounter<K1, Cmp>>::Output>
    );
}


struct CountInstances;
impl<AccList, Cmp> Function<(AccList, Nil, Cmp)> for CountInstances {
    type Apply = AccList;
}
impl<AccList,X, XS, Cmp> Function<(AccList, Cons<X, XS>, Cmp)> for CountInstances
where
    AccList: IncrementItemCounter<X, Cmp>,
    CountInstances: Function<(<AccList as IncrementItemCounter<X, Cmp>>::Output, XS, Cmp)>
 {
    type Apply = func_call!(CountInstances[<AccList as IncrementItemCounter<X, Cmp>>::Output, XS, Cmp]);
}

struct Contains<Cmp>(PhantomData<Cmp>);
impl<Cmp, Target> Function<(Nil, Target)> for Contains<Cmp> {
    type Apply = False;
}
impl<X, XS, Cmp, Target> Function<(Cons<X, XS>, Target)> for Contains<Cmp>
where
    Cmp: Predicate<(X, Target)>,
    Self: Function<(XS, Target)>,
    If<<Cmp as Predicate<(X, Target)>>::BoolApply, True, <Contains<Cmp> as Function<(XS, Target)>>::Apply>: IfOutput,
 {
    type Apply = typeif!(
        pred_call!(Cmp[X, Target]),
        True,
        func_call!(Self[(XS, Target)])
    );
}


type ContainsCell = Contains<CellComparisonFunction>;

struct CellShouldLive<AliveCellList>(PhantomData<AliveCellList>);
impl<X: Number, Y: Number, N: Number, AliveCellList> Function<(Cell<X, Y>, N)> for CellShouldLive<AliveCellList>
where
    N: PeanoEqual<N2>,
    N: PeanoEqual<N3>,
    ContainsCell: Function<(AliveCellList, Cell<X, Y>)>,
    <Contains<CellComparisonFunction> as Function<(AliveCellList, Cell<X, Y>)>>::Apply: And<<N as PeanoEqual<N2>>::Equal>, 
    <N as PeanoEqual<N3>>::Equal: Or<<<Contains<CellComparisonFunction> as Function<(AliveCellList, Cell<X, Y>)>>::Apply as And<<N as PeanoEqual<N2>>::Equal>>::And>
{
    type Apply = logic!(or
        m!(eq N, N3),
        logic!(
            and 
            func_call!(ContainsCell[AliveCellList, Cell<X, Y>]),
            m!(eq N, N2)
        )
    );
}

struct FirstTupleElement;
impl<A, B> Function<(A, B)> for FirstTupleElement {
    type Apply = A;
}

struct SingleGOLIter;
impl<CurrentCells> Function<CurrentCells> for SingleGOLIter 
where 
    CurrentCells: Map<GetCellNeighbors>,
    <CurrentCells as Map<GetCellNeighbors>>::Output: ListConcatAll,
    CountInstances: Function<(Nil, <<CurrentCells as Map<GetCellNeighbors>>::Output as ListConcatAll>::ListConcatAll, CellComparisonFunction)>,
    <CountInstances as Function<(Nil, <<CurrentCells as Map<GetCellNeighbors>>::Output as ListConcatAll>::ListConcatAll, CellComparisonFunction)>>::Apply: Filter<CellShouldLive<CurrentCells>>,
    <<CountInstances as Function<(Nil, <<CurrentCells as Map<GetCellNeighbors>>::Output as ListConcatAll>::ListConcatAll, CellComparisonFunction)>>::Apply as Filter<CellShouldLive<CurrentCells>>>::Output: Map<FirstTupleElement>
{
    type Apply = <<
        func_call!(CountInstances[
            Nil,
            <<CurrentCells as Map<GetCellNeighbors>>::Output as ListConcatAll>::ListConcatAll,
            CellComparisonFunction
        ])
        as Filter<CellShouldLive<CurrentCells>>>::Output 
        as Map<FirstTupleElement>>::Output;
}

#[cfg(test)]
mod test {
    use crate::{
        booleans::{False, True},
        functions::Function,
        gameoflife::{Cell, IncrementItemCounter, SingleGOLIter},
        lists::StrRepr,
        make_list,
        numbers::{Zero, N1, N2, N3, Negative}, func_call,
    };

    #[test]
    fn test_cell_neighbors() {
        type origin = Cell<Zero, Zero>;
        type left = Cell<Negative<N1>, Zero>;
        type right = Cell<N1, Zero>;
        type cells = make_list!(left,origin,right,);

        type result = func_call!(SingleGOLIter[cells]);
        println!("{}", result::str_repr());
        type result2 = func_call!(SingleGOLIter[result]);
        println!("{}", result2::str_repr());
    }

    #[test]
    fn test_increment_item_counter() {
        struct Apple;
        impl StrRepr for Apple {}
        struct Banana;
        impl StrRepr for Banana {}
        struct Orange;
        impl StrRepr for Orange {}
        type counter = make_list!((Apple, N1), (Banana, N3), (Orange, N2),);

        type False2 = False;

        struct EqualTypes;
        impl Function<(Apple, Apple)> for EqualTypes {
            type Apply = True;
        }
        impl Function<(Apple, Banana)> for EqualTypes {
            type Apply = False2;
        }
        impl Function<(Apple, Orange)> for EqualTypes {
            type Apply = False2;
        }
        impl Function<(Banana, Apple)> for EqualTypes {
            type Apply = False2;
        }
        impl Function<(Banana, Banana)> for EqualTypes {
            type Apply = True;
        }
        impl Function<(Banana, Orange)> for EqualTypes {
            type Apply = False2;
        }
        impl Function<(Orange, Apple)> for EqualTypes {
            type Apply = False2;
        }
        impl Function<(Orange, Banana)> for EqualTypes {
            type Apply = False2;
        }
        impl Function<(Orange, Orange)> for EqualTypes {
            type Apply = True;
        }

        type incremented_counter = <counter as IncrementItemCounter<Orange, EqualTypes>>::Output;

        println!("{}", counter::str_repr());
        println!("{}", incremented_counter::str_repr())
    }
}
