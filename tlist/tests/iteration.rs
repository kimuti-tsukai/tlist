use tlist::{ToDynBox, list};

#[test]
fn into_iter() {
    let list = list![1, 2, 3];
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
}

#[test]
fn ref_into_iter() {
    let list = list![1, 2, 3];
    let mut iter = (&list).into_iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

#[test]
fn mut_into_iter() {
    let mut list = list![1, 2, 3];
    let mut iter = (&mut list).into_iter();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), None);
}

#[test]
fn different_types() {
    let list = list![1, "Hello", true, 'c', std::f64::consts::PI];

    let mut iter = list.to_dyn_box().into_iter();
    assert_eq!(
        iter.next().map(|v| v.downcast().unwrap()),
        Some(Box::new(1))
    );
    assert_eq!(
        iter.next().map(|v| v.downcast().unwrap()),
        Some(Box::new("Hello"))
    );
    assert_eq!(
        iter.next().map(|v| v.downcast().unwrap()),
        Some(Box::new(true))
    );
    assert_eq!(
        iter.next().map(|v| v.downcast().unwrap()),
        Some(Box::new('c'))
    );
    assert_eq!(
        iter.next().map(|v| v.downcast().unwrap()),
        Some(Box::new(std::f64::consts::PI))
    );
    assert_eq!(iter.next().map(|v| v.downcast().unwrap()), None::<Box<()>>);
}
