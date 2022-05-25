fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use serial_test::serial;

    #[test]
    fn test_1() {
        assert!(true);
    }

    #[test]
    // uncomment the following line and run `cargo valgrind test`
    //#[serial]
    fn test_2() {
        assert!(true);
    }
}
