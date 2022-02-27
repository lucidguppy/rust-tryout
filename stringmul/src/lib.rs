pub mod string_utils {
    /// multiply a string a given number of times
    pub fn multiply_string(value: String, times: u64) -> String{
        let mut retval = "".to_string();
        for _ in 0..times {
            retval.push_str(&value)
        }
        retval
    }
    pub fn reverse(value: String) -> String{
        value
            .chars()
            .rev()
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use crate::string_utils::{multiply_string, reverse};

    #[test]
    fn it_works() {
        let result =multiply_string("foo".to_string(), 3);
        assert_eq!(result, "foofoofoo");
    }
    #[test]
    fn test_reverse() {
        let result =reverse("foo".to_string());
        assert_eq!(result, "oof");
    }
}
