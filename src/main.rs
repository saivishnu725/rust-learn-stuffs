fn main() {
    // Mutable variables
    let mut x: u32 = 5;
    println!("value = {}", x);
    x = 10;
    println!("value = {}", x);
    // Tuples
    let tup: (u32, f64, bool) = (200, 20.0, true);
    let (a, b, _c) = tup;
    println!("A = {}", a);
    println!("B = {}", b);
    println!("C = {}", tup.2);
    // Arrays
    let ar = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    println!("ar = {}", ar[1]);
    let _ar1: [i32; 5] = [1, 2, 3, 4, 5];
    
}
