use std::{any::Any, marker::PhantomData};

// Natural Numbers
pub trait Nat {
    const VALUE: usize;

    type Add<T: Nat>: Nat;
    type Mul<T: Nat>: Nat;
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Zero;
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Succ<N: Nat>(PhantomData<N>);

impl Nat for Zero {
    const VALUE: usize = 0;

    type Add<T: Nat> = T;
    type Mul<T: Nat> = Zero;
}

impl<N: Nat> Nat for Succ<N> {
    const VALUE: usize = N::VALUE + 1;

    type Add<T: Nat> = Succ<N::Add<T>>;
    type Mul<T: Nat> = <N::Mul<T> as Nat>::Add<T>;
}

// Optional
pub trait Optional<T> {
    type AsRef<'a>: Optional<&'a T>
    where
        T: 'a;
    type AsMut<'a>: Optional<&'a mut T>
    where
        T: 'a;

    fn to_option(self) -> Option<T>;
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct TNone;
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct TSome<T>(T);

impl<T> Optional<T> for TNone {
    type AsRef<'a>
        = TNone
    where
        T: 'a;
    type AsMut<'a>
        = TNone
    where
        T: 'a;

    fn to_option(self) -> Option<T> {
        None
    }
}

impl<T> Optional<T> for TSome<T> {
    type AsRef<'a>
        = TSome<&'a T>
    where
        T: 'a;
    type AsMut<'a>
        = TSome<&'a mut T>
    where
        T: 'a;

    fn to_option(self) -> Option<T> {
        Some(self.0)
    }
}

// Type-level list
pub trait HList {
    fn get<N>(&self) -> &<Self as Get<N>>::Output
    where
        Self: Get<N>,
    {
        Get::get(self)
    }

    fn get_mut<N>(&mut self) -> &mut <Self as Get<N>>::Output
    where
        Self: GetMut<N>,
    {
        GetMut::get_mut(self)
    }

    fn get_optional<N>(&self) -> <Self::Optional as Optional<Self::Output>>::AsRef<'_>
    where
        Self: GetOptional<N>,
    {
        GetOptional::get_optional(self)
    }

    fn get_optional_mut<N>(&mut self) -> <Self::Optional as Optional<Self::Output>>::AsMut<'_>
    where
        Self: GetOptionalMut<N>,
    {
        GetOptionalMut::get_optional_mut(self)
    }

    fn push<T>(self, value: T) -> Cons<T, Self>
    where
        Self: Sized,
    {
        Cons(value, self)
    }

    fn pop(self) -> (Self::Removed, Self::Output)
    where
        Self: Remove<Zero> + Sized,
    {
        Remove::remove(self)
    }

    fn insert<N, T>(self, value: T) -> Self::Output
    where
        Self: Insert<N, T> + Sized,
    {
        Insert::insert(self, value)
    }

    fn remove<N>(self) -> (Self::Removed, Self::Output)
    where
        Self: Remove<N> + Sized,
    {
        Remove::remove(self)
    }

    fn remove_optional<N>(self) -> (Self::Optional, Self::Output)
    where
        Self: RemoveOptional<N> + Sized,
    {
        RemoveOptional::remove_optional(self)
    }
}

#[macro_export]
macro_rules! list {
    () => {
        $crate::Nil
    };
    ($head:expr) => {
        $crate::Cons($head, $crate::Nil)
    };
    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::Cons($head, $crate::list!($($tail),+))
    };
}

#[macro_export]
macro_rules! List {
    () => {
        $crate::Nil
    };
    ($head:ty) => {
        $crate::Cons<$head, $crate::Nil>
    };
    ($head:ty, $($tail:ty),+ $(,)?) => {
        $crate::Cons<$head, $crate::List!($($tail),+)>
    };
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Nil;
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Cons<Head, Tail>(Head, Tail);

impl Nil {}
impl<Head, Tail> Cons<Head, Tail> {}

impl HList for Nil {}
impl<Head, Tail: HList> HList for Cons<Head, Tail> {}

pub trait Get<N> {
    type Output;

    fn get(&self) -> &Self::Output;
}

impl<Head, Tail> Get<Zero> for Cons<Head, Tail> {
    type Output = Head;

    fn get(&self) -> &Self::Output {
        &self.0
    }
}

impl<N: Nat, Head, Tail> Get<Succ<N>> for Cons<Head, Tail>
where
    Tail: Get<N>,
{
    type Output = <Tail as Get<N>>::Output;

    fn get(&self) -> &Self::Output {
        self.1.get()
    }
}

pub trait GetMut<N>: Get<N> {
    fn get_mut(&mut self) -> &mut Self::Output;
}

impl<Head, Tail> GetMut<Zero> for Cons<Head, Tail> {
    fn get_mut(&mut self) -> &mut Self::Output {
        &mut self.0
    }
}

impl<N: Nat, Head, Tail> GetMut<Succ<N>> for Cons<Head, Tail>
where
    Tail: GetMut<N>,
{
    fn get_mut(&mut self) -> &mut Self::Output {
        self.1.get_mut()
    }
}

pub trait GetOptional<N> {
    type Output;
    type Optional: Optional<Self::Output>;

    fn get_optional(&self) -> <Self::Optional as Optional<Self::Output>>::AsRef<'_>;
}

pub trait GetOptionalMut<N>: GetOptional<N> {
    fn get_optional_mut(&mut self) -> <Self::Optional as Optional<Self::Output>>::AsMut<'_>;
}

impl<N> GetOptional<N> for Nil {
    type Output = ();
    type Optional = TNone;

    fn get_optional(&self) -> <Self::Optional as Optional<Self::Output>>::AsRef<'_> {
        TNone
    }
}

impl<N> GetOptionalMut<N> for Nil {
    fn get_optional_mut(&mut self) -> <Self::Optional as Optional<Self::Output>>::AsMut<'_> {
        TNone
    }
}

impl<Head, Tail> GetOptional<Zero> for Cons<Head, Tail> {
    type Output = Head;
    type Optional = TSome<Head>;

    fn get_optional(&self) -> <Self::Optional as Optional<Self::Output>>::AsRef<'_> {
        TSome(&self.0)
    }
}

impl<Head, Tail> GetOptionalMut<Zero> for Cons<Head, Tail> {
    fn get_optional_mut(&mut self) -> <Self::Optional as Optional<Self::Output>>::AsMut<'_> {
        TSome(&mut self.0)
    }
}

impl<N: Nat, Head, Tail> GetOptional<Succ<N>> for Cons<Head, Tail>
where
    Tail: GetOptional<N>,
{
    type Output = <Tail as GetOptional<N>>::Output;
    type Optional = <Tail as GetOptional<N>>::Optional;

    fn get_optional(&self) -> <Self::Optional as Optional<Self::Output>>::AsRef<'_> {
        self.1.get_optional()
    }
}

impl<N: Nat, Head, Tail> GetOptionalMut<Succ<N>> for Cons<Head, Tail>
where
    Tail: GetOptionalMut<N>,
{
    fn get_optional_mut(&mut self) -> <Self::Optional as Optional<Self::Output>>::AsMut<'_> {
        self.1.get_optional_mut()
    }
}

pub trait Insert<N, T> {
    type Output;

    fn insert(self, value: T) -> Self::Output;
}

impl<L, T> Insert<Zero, T> for L {
    type Output = Cons<T, L>;

    fn insert(self, value: T) -> Self::Output {
        Cons(value, self)
    }
}

impl<N: Nat, Head, Tail, T> Insert<Succ<N>, T> for Cons<Head, Tail>
where
    Tail: Insert<N, T>,
{
    type Output = Cons<Head, <Tail as Insert<N, T>>::Output>;

    fn insert(self, value: T) -> Self::Output {
        Cons(self.0, self.1.insert(value))
    }
}

pub trait Remove<N> {
    type Removed;
    type Output;

    fn remove(self) -> (Self::Removed, Self::Output);
}

impl<Head, Tail> Remove<Zero> for Cons<Head, Tail> {
    type Removed = Head;
    type Output = Tail;

    fn remove(self) -> (Self::Removed, Self::Output) {
        (self.0, self.1)
    }
}

impl<N: Nat, Head, Tail> Remove<Succ<N>> for Cons<Head, Tail>
where
    Tail: Remove<N>,
{
    type Removed = <Tail as Remove<N>>::Removed;
    type Output = Cons<Head, <Tail as Remove<N>>::Output>;

    fn remove(self) -> (Self::Removed, Self::Output) {
        let (removed, tail) = self.1.remove();
        (removed, Cons(self.0, tail))
    }
}

pub trait RemoveOptional<N> {
    type Removed;
    type Output;
    type Optional: Optional<Self::Removed>;

    fn remove_optional(self) -> (Self::Optional, Self::Output);
}

impl<N> RemoveOptional<N> for Nil {
    type Removed = ();
    type Output = Nil;
    type Optional = TNone;

    fn remove_optional(self) -> (Self::Optional, Self::Output) {
        (TNone, Nil)
    }
}

impl<Head, Tail> RemoveOptional<Zero> for Cons<Head, Tail> {
    type Removed = Head;
    type Output = Tail;
    type Optional = TSome<Head>;

    fn remove_optional(self) -> (Self::Optional, Self::Output) {
        (TSome(self.0), self.1)
    }
}

impl<N, Head, Tail> RemoveOptional<Succ<N>> for Cons<Head, Tail>
where
    N: Nat,
    Tail: RemoveOptional<N>,
{
    type Removed = <Tail as RemoveOptional<N>>::Removed;
    type Output = Cons<Head, <Tail as RemoveOptional<N>>::Output>;
    type Optional = <Tail as RemoveOptional<N>>::Optional;

    fn remove_optional(self) -> (Self::Optional, Self::Output) {
        let (removed, tail) = self.1.remove_optional();
        (removed, Cons(self.0, tail))
    }
}

pub trait All<T> {}

impl<T> All<T> for Nil {}
impl<T, Tail> All<T> for Cons<T, Tail> where Tail: All<T> {}

trait PopOptional<T> {
    fn pop_optional(self: Box<Self>) -> (Option<T>, Box<dyn PopOptional<T>>);
}

impl<T> PopOptional<T> for Nil {
    fn pop_optional(self: Box<Self>) -> (Option<T>, Box<dyn PopOptional<T>>) {
        (None, Box::new(Nil))
    }
}

impl<T, Tail> PopOptional<T> for Cons<T, Tail>
where
    Tail: PopOptional<T> + 'static,
{
    fn pop_optional(self: Box<Self>) -> (Option<T>, Box<dyn PopOptional<T>>) {
        let Cons(value, tail) = *self;
        (Some(value), Box::new(tail))
    }
}

pub struct IntoIter<T> {
    list: Option<Box<dyn PopOptional<T>>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let list = self.list.take()?;
        let (value, list) = list.pop_optional();
        self.list = Some(list);
        value
    }
}

impl IntoIterator for Nil {
    type Item = ();
    type IntoIter = IntoIter<()>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            list: Some(Box::new(self)),
        }
    }
}

impl<Head, Tail> IntoIterator for Cons<Head, Tail>
where
    Head: 'static,
    Tail: PopOptional<Head> + 'static,
{
    type Item = Head;
    type IntoIter = IntoIter<Head>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            list: Some(Box::new(self)),
        }
    }
}

pub trait AsRefList<'a> {
    type Output;

    fn as_ref_list(&'a self) -> Self::Output;
}

pub trait AsMutList<'a> {
    type Output;

    fn as_mut_list(&'a mut self) -> Self::Output;
}

impl<'a> AsRefList<'a> for Nil {
    type Output = Nil;

    fn as_ref_list(&'a self) -> Self::Output {
        Nil
    }
}

impl<'a, Head, Tail> AsRefList<'a> for Cons<Head, Tail>
where
    Head: 'a,
    Tail: AsRefList<'a>,
{
    type Output = Cons<&'a Head, <Tail as AsRefList<'a>>::Output>;

    fn as_ref_list(&'a self) -> Self::Output {
        Cons(&self.0, self.1.as_ref_list())
    }
}

impl<'a> AsMutList<'a> for Nil {
    type Output = Nil;

    fn as_mut_list(&'a mut self) -> Self::Output {
        Nil
    }
}

impl<'a, Head, Tail> AsMutList<'a> for Cons<Head, Tail>
where
    Head: 'a,
    Tail: AsMutList<'a>,
{
    type Output = Cons<&'a mut Head, <Tail as AsMutList<'a>>::Output>;

    fn as_mut_list(&'a mut self) -> Self::Output {
        Cons(&mut self.0, self.1.as_mut_list())
    }
}

pub trait AsDynRef<'a> {
    type Output: All<&'a dyn Any>;

    fn as_dyn_ref(&'a self) -> Self::Output;
}

pub trait AsDynMut<'a> {
    type Output: All<&'a mut dyn Any>;

    fn as_dyn_mut(&'a mut self) -> Self::Output;
}

pub trait ToDynBox {
    type Output: All<Box<dyn Any>>;

    fn to_dyn_box(self) -> Self::Output;
}

impl ToDynBox for Nil {
    type Output = Nil;

    fn to_dyn_box(self) -> Self::Output {
        Nil
    }
}

impl<Head, Tail> ToDynBox for Cons<Head, Tail>
where
    Head: 'static,
    Tail: ToDynBox,
{
    type Output = Cons<Box<dyn Any>, <Tail as ToDynBox>::Output>;

    fn to_dyn_box(self) -> Self::Output {
        Cons(Box::new(self.0), self.1.to_dyn_box())
    }
}

impl<'a> AsDynRef<'a> for Nil {
    type Output = Nil;

    fn as_dyn_ref(&'a self) -> Self::Output {
        Nil
    }
}

impl<'a, Head, Tail> AsDynRef<'a> for Cons<Head, Tail>
where
    Head: 'static,
    Tail: AsDynRef<'a>,
{
    type Output = Cons<&'a dyn Any, <Tail as AsDynRef<'a>>::Output>;

    fn as_dyn_ref(&'a self) -> Self::Output {
        Cons(&self.0, self.1.as_dyn_ref())
    }
}

impl<'a> AsDynMut<'a> for Nil {
    type Output = Nil;

    fn as_dyn_mut(&'a mut self) -> Self::Output {
        Nil
    }
}

impl<'a, Head, Tail> AsDynMut<'a> for Cons<Head, Tail>
where
    Head: 'static,
    Tail: AsDynMut<'a>,
{
    type Output = Cons<&'a mut dyn Any, <Tail as AsDynMut<'a>>::Output>;

    fn as_dyn_mut(&'a mut self) -> Self::Output {
        Cons(&mut self.0, self.1.as_dyn_mut())
    }
}
