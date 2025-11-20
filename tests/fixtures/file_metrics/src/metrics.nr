// File metrics test fixture

fn main() {
    let x = 1;
    let y = 2;
    constrain x + y == 3;
}

// A passing test
#[test]
fn test_main() {
    main(1, 2);
}

// A failing test
#[test(should_fail)]
fn test_fail() {
    assert(false);
}