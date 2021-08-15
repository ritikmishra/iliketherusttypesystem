use std::marker::PhantomData;

#[derive(Default)]
pub struct Nil;
#[derive(Default)]
pub struct Cons<X, XS>(PhantomData<X>, PhantomData<XS>);

pub trait List {}
impl List for Nil {}
impl<Item, Rest> List for Cons<Item, Rest> where Rest: List {}

pub trait First {
    type First;
}
impl First for Nil {
    type First = Nil;
}
impl<ItemType, Rest> First for Cons<ItemType, Rest> {
    type First = ItemType;
}

pub trait ListConcat: List {
    type ConcatWith<OtherList: List + ListConcat>: List;
}
impl ListConcat for Nil {
    // Nil ++ OtherList == OtherList
    type ConcatWith<OtherList: List + ListConcat> = OtherList;
}
impl<Item, BS: ListConcat> ListConcat for Cons<Item, BS> {
    type ConcatWith<OtherList: List + ListConcat> = Cons<Item, BS::ConcatWith<OtherList>>;
}

// Concat all of the lists in a list together
pub trait ListConcatAll {
    type ListConcatAll: List + ListConcat;
}
impl ListConcatAll for Nil {
    type ListConcatAll = Nil;
}
/// Implement `ListConcatAll` for `Cons`
/// Given `FirstList` is `ListConcat` (can be concated to other lists)
/// and `OtherLists` are `ListConcatAll<ListConcatAll: ListConcat>`
///     (i.e we can concat them all together, and the result can be concated)
///
/// concat all the lists in `Cons` together
impl<FirstList: ListConcat, OtherLists: List + ListConcatAll<ListConcatAll: ListConcat>>
    ListConcatAll for Cons<FirstList, OtherLists>
where
    <FirstList as ListConcat>::ConcatWith<<OtherLists as ListConcatAll>::ListConcatAll>: ListConcat,
{
    type ListConcatAll = <FirstList as ListConcat>::ConcatWith<OtherLists::ListConcatAll>;
}


