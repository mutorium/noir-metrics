/*
    File metrics test fixture - block comment test
    2nd line
    Todo: test
*/

fn main() {
    let x = 1;
    let y = 2;
    constrain x + y == 3;
}

// todo: A passing test
#[test]
fn test_main() {
    main(1, 2);
}

// fixme: A failing test
#[test(should_fail)]
fn test_fail() {
    assert(false);
}