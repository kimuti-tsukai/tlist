use nat_macros::expr;
use tlist::{Nat, Succ, Zero};

#[allow(unused)]
#[test]
fn expr_test() {
    type One = Succ<Zero>;
    type Two = Succ<One>;
    type Three = expr!(+ One, Two);
    type Four = expr!(* Two, Two);
    type Five = expr!(+ Two, Three);
    type Six = expr!(+ One, Two, Three);
    type Seven = expr!(+ One, (* Two, Three));

    assert_eq!(One::VALUE, 1);
    assert_eq!(Two::VALUE, 2);
    assert_eq!(Three::VALUE, 3);
    assert_eq!(Four::VALUE, 4);
    assert_eq!(Five::VALUE, 5);
    assert_eq!(Six::VALUE, 6);
    assert_eq!(Seven::VALUE, 7);
}
