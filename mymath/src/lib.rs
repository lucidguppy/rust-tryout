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
    pub fn sum_squares(value: &Vec<u64>) -> u64 {
        /*  this also works...
         *  value.iter().fold(0,|accum, value| accum + value*value)
         */

        match value.clone().into_iter().reduce(|accum, value| {
            accum + value*value
        }) {
            None=> 0,
            Some(val)=>val
        }


    }
}

#[cfg(test)]
mod tests {
    use crate::maths::{square, sum_squares};
    #[test]
    fn it_works() {
        assert_eq!(square(2), 4);
    }
    #[test]
    fn test_sum_squares() {
        let x = vec![1,2,3];
        assert_eq!(sum_squares(&x), 14);
        assert_eq!(x[0], 1);
    }
}