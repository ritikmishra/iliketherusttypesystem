#![feature(generic_associated_types)]

use std::marker::PhantomData;

#[derive(Default)]
struct Nil;
#[derive(Default)]
struct Cons<X, XS>(PhantomData<X>, PhantomData<XS>);

trait List {}
impl List for Nil {}
impl<Item, Rest> List for Cons<Item, Rest> where Rest: List {}

trait First {
    type First;
}
impl First for Nil {
    type First = Nil;
}
impl<ItemType, Rest> First for Cons<ItemType, Rest> {
    type First = ItemType;
}

trait ListConcat: List {
    type ConcatWith<OtherList: List + ListConcat>;
}
impl ListConcat for Nil {
    // Nil ++ OtherList == OtherList
    type ConcatWith<OtherList: List + ListConcat> = OtherList;
}
impl<Item, BS: ListConcat> ListConcat for Cons<Item, BS> {
    type ConcatWith<OtherList: List + ListConcat> = Cons<Item, BS::ConcatWith<OtherList>>;
}

#[derive(Default)]
struct NC<const FOO: usize>;



fn main() {
    type OneTwoThree = Cons<NC<1>, Cons<NC<2>, Cons<NC<3>, Nil>>>;
    type FourFive = Cons<NC<4>, Cons<NC<5>, Nil>>;
    type Concated = <OneTwoThree as ListConcat>::ConcatWith<FourFive>;
    let foo: Concated = 4;
}
