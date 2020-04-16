pub mod util {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use crate::util::add;
    #[test]
    fn it_works() {
        assert_eq!(6, add(2, 4));
        assert_eq!(1, add(-2, 3));
    }
}
