fn main() {
    let mut x: u32 = 5;
    println!("value = {}", x);
    x = 10;
    println!("value = {}", x);
    let tup: (u32, f64, bool) = (200, 20.0, true);
    let (a, b, _c) = tup;
    println!("A = {}", a);
    println!("B = {}", b);
    println!("C = {}", tup.2);
}
