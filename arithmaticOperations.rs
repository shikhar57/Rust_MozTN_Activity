use std::{i8,i16,i32,i64,u8,u16,u32,u64,f32,f64,isize,usize};
use std::io::stdin;
fn main() {
println!("Arithmetic Operations Calculator");

println!("Provide the first number");

let mut n1 = String::new();

io::stdin().read_line(&mut n1)
.expect("Error: Line not read");
println!("Provide the second number: ");
let mut n2 = String::new();
io::stdin().read_line(&mut n2)
.expect("Error: Line not read");
println!("Add: {:?}", n1+n2);

println!("Sub: {:?}", n1-n2);

println!("Multi: {:?}", n1*n2);

println!("Div: {:?}", n1/n2);
}
