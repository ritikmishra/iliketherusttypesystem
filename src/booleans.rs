pub struct True;
pub struct False;

pub trait Bool {}
impl Bool for True {}
impl Bool for False {}

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
