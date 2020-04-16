extern crate iotest;

#[test]
fn it_works() {
    println!("{:?}", iotest::command_read());
    assert_eq!(1,1);
}