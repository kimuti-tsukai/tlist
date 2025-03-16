use tlist::list;

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
