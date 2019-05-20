#![feature(unsize)]

use std::marker::PhantomData;
use std::marker::Unsize;

pub trait Item<'a, T: ?Sized + 'a> {
    type Iter: DoubleEndedIterator<Item = &'a T>;

    fn iter(&'a self) -> Self::Iter;
}

pub struct Node<'a, I: ?Sized, L: Item<'a, I>, R: Item<'a, I>>(pub L, pub R, PhantomData<&'a I>);
impl<'a, I: ?Sized + 'a, L: Item<'a, I>, R: Item<'a, I>> Node<'a, I, L, R> {
    #[inline]
    pub fn new(left: L, right: R) -> Self {
        Node(left, right, Default::default())
    }
}

impl<'a, I: ?Sized + 'a, L: Item<'a, I>, R: Item<'a, I>> Item<'a, I> for Node<'a, I, L, R> {
    type Iter = std::iter::Chain<L::Iter, R::Iter>;

    #[inline]
    fn iter(&'a self) -> Self::Iter {
        self.0.iter().chain(self.1.iter())
    }
}

pub struct Leaf<'a, O: Unsize<I>, I: ?Sized>(pub O, PhantomData<&'a I>);
impl<'a, O: Unsize<I>, I: ?Sized + 'a> Leaf<'a, O, I> {
    #[inline]
    pub fn new(data: O) -> Self {
        Leaf(data, Default::default())
    }
}

impl<'a, O: Unsize<I>, I: ?Sized + 'a> Item<'a, I> for Leaf<'a, O, I> {
    type Iter = std::iter::Once<&'a I>;

    #[inline]
    fn iter(&'a self) -> Self::Iter {
        std::iter::once(&self.0 as &I)
    }
}

#[macro_export]
macro_rules! static_list {
    ($w:expr) => (Leaf::new($w));
    ($w:expr,) => (Leaf::new($w));
    ($w:expr, $($rest:tt)*) => (Node::new(Leaf::new($w), static_list!($($rest)*)));
}

#[macro_export]
macro_rules! static_list_type {
    (&$l:tt $o:ty; $w:ty) => (Leaf<$l, $w, $o>);
    (&$l:tt $o:ty; $w:ty,) => (Leaf<$l, $w, $o>);
    (&$l:tt $o:ty; $w:ty, $($rest:tt)*) => (Node<$l, $o, Leaf<$l, $w, $o>, static_list_type!(&$l $o; $($rest)*)>);
}
