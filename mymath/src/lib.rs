pub mod maths {
    /// returns an integer that is the square of the passed in value
    ///
    ///     use mymath::maths::square;
    ///     assert_eq!(square(3), 9);
    ///     assert_eq!(square(2), 4);
    ///
    pub fn square(value: u64) -> u64 {
        value * value
    }
}

#[cfg(test)]
mod tests {
    use crate::maths::square;
    #[test]
    fn it_works() {
        assert_eq!(square(2), 4);
    }
}