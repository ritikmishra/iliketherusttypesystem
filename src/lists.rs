use std::marker::PhantomData;

#[derive(Default)]
pub struct Nil;
#[derive(Default)]
pub struct Cons<X, XS>(PhantomData<X>, PhantomData<XS>);

#[macro_export]
macro_rules! make_list {
    ($x:ty, $($xs:ty,)+) => {
        $crate::lists::Cons<$x, $crate::make_list!($($xs,)+)>
    };
    ($x:ty $(,)?) => {
        $crate::lists::Cons<$x, $crate::lists::Nil>
    };
    () => {
        $crate::lists::Nil
    };
}

pub trait List {}
impl List for Nil {}
impl<Item, Rest> List for Cons<Item, Rest> where Rest: List {}

trait Items {
    fn items() -> Vec<String>;
}
impl Items for Nil {
    fn items() -> Vec<String> {
        vec![]
    }
}
impl<Item: StrRepr, Rest: Items> Items for Cons<Item, Rest> {
    fn items() -> Vec<String> {
        let mut rest = Rest::items();
        rest.insert(0, Item::str_repr());
        rest
    }
}

pub trait StrRepr {
    fn str_repr() -> String {
        std::any::type_name::<Self>().into()
    }
}
impl StrRepr for Nil {
    fn str_repr() -> String {
        "[]".into()
    }
}
impl<I, R> StrRepr for Cons<I, R>
where
    Cons<I, R>: Items,
{
    fn str_repr() -> String {
        format!("[{}]", Self::items().join(", "))
    }
}

macro_rules! tuple_impl_str_repr {
    ($($t:ident,)*) => {
        impl<$($t,)*> StrRepr for ($($t,)*) where 
            $(
                $t: StrRepr,
            )*
        {
            fn str_repr() -> String {
                let mut ret = String::new();
                ret.push('(');

                $(
                    ret.push_str(&<$t as StrRepr>::str_repr());
                    ret.push_str(", ");
                )*
                ret.pop();
                ret.pop();

                ret.push(')');
                ret
            }
        }
    };
}

tuple_impl_str_repr!(T1,);
tuple_impl_str_repr!(T1,T2,);
tuple_impl_str_repr!(T1,T2,T3,);
tuple_impl_str_repr!(T1,T2,T3,T4,);
tuple_impl_str_repr!(T1,T2,T3,T4,T5,);
tuple_impl_str_repr!(T1,T2,T3,T4,T5,T6,);
tuple_impl_str_repr!(T1,T2,T3,T4,T5,T6,T7,);
tuple_impl_str_repr!(T1,T2,T3,T4,T5,T6,T7,T8,);
tuple_impl_str_repr!(T1,T2,T3,T4,T5,T6,T7,T8,T9,);

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
