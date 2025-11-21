/*
    Project metrics fixture - file 2
*/

fn main(x: u64, y: pub u64) {
    assert(x != y);
}

// A passing test
#[test]
fn test_main() {
    main(3, 4);
}

#[test]
fn test_addition() {
    let x = 1;
    let y = 2;
    constrain x + y == 3;
}
