#[cfg(test)]
pub mod tests {
    #[test]
    pub fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    pub fn it_fails() {
        panic!("Test failed.!")
    }
}
