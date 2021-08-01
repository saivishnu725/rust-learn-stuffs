fn main() {
    // _tuples();
    // _arrays();
    // _variables();
    // println!("Function = {}", _func(10, 20));
    _cond_if();
}

// Tuples
fn _tuples() {
    let tup: (u32, f64, bool) = (200, 20.0, true);
    let (a, b, _c) = tup;
    println!("A = {}", a);
    println!("B = {}", b);
    println!("C = {}", tup.2);
}
// Arrays
fn _arrays() {
    let ar = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    println!("ar = {}", ar[1]);
    let _ar1: [i32; 5] = [1, 2, 3, 4, 5];
    let y = { 20 / 1 };
    println!("{}", y);
}
// Mutable variables
fn _variables() {
    let mut x: u32 = 5;
    println!("value = {}", x);
    x = 10;
    println!("value = {}", x);
}
// Functions
fn _func(x: i32, y: i32) -> i32 {
    println!("x = {}", x);
    println!("y = {}", y);
    x + y
}
// If else
fn _cond_if() {
    let num = 3;
    if num < 5 {
        println!("Less than five");
    } else {
        println!("More than five");
    }
}
