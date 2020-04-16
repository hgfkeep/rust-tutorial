mod math;
use math::{add, sub};
fn some(x: u32, y: u32) -> u32 {
  add(x, y);
  sub(x, y)  
}
