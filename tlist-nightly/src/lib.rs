#![feature(specialization, fn_traits, unboxed_closures)]

pub use tlist::*;

pub trait TBool {
    const VALUE: bool;

    type Or<T: TBool>: TBool;
}

pub struct TTrue;
pub struct TFalse;

impl TBool for TTrue {
    const VALUE: bool = true;

    type Or<T: TBool> = TTrue;
}

impl TBool for TFalse {
    const VALUE: bool = false;

    type Or<T: TBool> = T;
}

pub trait IsEqual<T> {
    type Output: TBool;
}

impl<T, U> IsEqual<U> for T {
    default type Output = TFalse;
}

impl<T> IsEqual<T> for T {
    type Output = TTrue;
}

pub trait Contains<T> {
    type Output: TBool;
}

impl<T> Contains<T> for Nil {
    type Output = TFalse;
}

impl<T, Head, Tail> Contains<T> for Cons<Head, Tail>
where
    Head: IsEqual<T>,
    Tail: Contains<T>,
{
    type Output = <Head::Output as TBool>::Or<Tail::Output>;
}

pub trait Map<F> {
    type Output;
    
    fn map(self, f: F) -> Self::Output;
}

impl<F> Map<F> for Nil {
    type Output = Nil;
    
    fn map(self, _: F) -> Self::Output {
        Nil
    }
}

impl<F, T, Head, Tail> Map<F> for Cons<Head, Tail>
where
    F: FnMut(Head) -> T,
    Tail: Map<F>,
{
    type Output = Cons<T, Tail::Output>;
    
    fn map(self, mut f: F) -> Self::Output {
        Cons::new(f(self.0), self.1.map(f))
    }
}

#[cfg(test)]
mod tests {
    use tlist::list;

    use crate::Map;

    #[allow(non_camel_case_types)]
    struct my_print;

    impl<T: std::fmt::Display> FnOnce<(T,)> for my_print {
        type Output = ();

        extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
            println!("{}", args.0);
        }
    }
    
    impl<T: std::fmt::Display> FnMut<(T,)> for my_print {
        extern "rust-call" fn call_mut(&mut self, args: (T,)) -> Self::Output {
            println!("{}", args.0);
        }
    }
    
    impl<T: std::fmt::Display> Fn<(T,)> for my_print {
        extern "rust-call" fn call(&self, args: (T,)) -> Self::Output {
            println!("{}", args.0);
        }
    }
    
    #[test]
    fn map_test() {
        let l = list!(1i32, "str", true, 'c');
        
        l.map(my_print);
    }
}
