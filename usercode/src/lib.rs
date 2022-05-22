use mymacro::my_macro;

#[my_macro()]
fn foo() {
    // Fail so we see output that tests are running:
    assert!(false);
}
