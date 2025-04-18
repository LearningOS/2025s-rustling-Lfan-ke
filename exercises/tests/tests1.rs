// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn you_can_assert() {
        assert!(false);
    }

    #[test]
    fn this_func_will_pass() {
        assert!(true);
    }

    /// doc test
    /// ```rust
    /// asserteq!(233, 0xe9)
    /// ```
    fn doc_test() {}
}
