pub mod stringmul {
    /// multiply a string a given number of times
    pub fn multiply_string(value: String, times: u64) -> String{
        let mut retval = "".to_string();
        for _ in 0..times {
            retval.push_str(&value)
        }
        retval
    }
}
#[cfg(test)]
mod tests {
    use crate::stringmul::multiply_string;

    #[test]
    fn it_works() {
        let result =multiply_string("foo".to_string(), 3);
        assert_eq!(result, "foofoofoo");
    }
}
