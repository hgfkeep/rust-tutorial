extern crate add;
extern crate addone;

fn main() {
    println!("1 + 6 = {}", add::util::add(1, 6));
    println!("3 + (-1) = {}", add::util::add(3, -1));
    println!("3 + 1 = {}", addone::one::addone(3));
}
