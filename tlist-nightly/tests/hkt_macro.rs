#![feature(fn_traits, unboxed_closures)]

use tlist_nightly::hkt;

#[test]
fn test_hkt() {
    #[hkt]
    fn foo<T>(_: T) {}
    
    fn f(_: impl FnMut(i32) + FnMut(&str)) {}
    f(foo);
}
