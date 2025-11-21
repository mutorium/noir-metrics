/*
    Project metrics fixture - file 1
*/

fn main(x: u64, y: pub u64) {
    assert(x != y);
}

#[test]
fn test_main() {
    main(1, 2);

    // Uncomment to make test fail
    // main(1, 1);
}

#[test(should_fail)]
fn test_fail() {
    assert(false);
}
