extern crate add;
pub mod one {
    pub fn addone(a: i32) -> i32{
        add::util::add(a, 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::one::addone;
    #[test]
    fn it_works() {
        assert_eq!(1, addone(0));
        assert_eq!(2, addone(1));
    }
}
