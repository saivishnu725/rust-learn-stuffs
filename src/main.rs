fn main() {
    // _tuples();
    // _arrays();
    // _variables();
    // println!("Function = {}", _func(10, 20));
    // _cond_if();
    // _cond_inf_loop();
    // _cond_loop();
    // _cond_while();
    _cond_for_arrays();
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
// Infinite / Recursive Loop
fn _cond_inf_loop() {
    let mut a = 0;
    loop {
        println!("{}\n", a);
        a += 1;
    }
}
// Loop
fn _cond_loop() {
    let mut a = 0;
    let b = loop {
        println!("a = {}", a);
        a += 1;
        if a > 10 {
            break a;
        }
    };
    println!("\nB = {}", b);
}
// While
fn _cond_while() {
    let a = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let mut l = 0;
    while l < 10 {
        println!("Value at index {} = {}", l, a[l]);
        l += 1;
    }
}
// For loop for array
fn _cond_for_arrays() {
    let a = [10, 20, 30, 40, 50];
    for e in a.iter() {
        println!("e = {}", e);
    }
    for number in (1..10).rev() {
        println!("Number = {}", number);
    }
}
