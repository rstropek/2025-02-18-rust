#![allow(unused_assignments)]

fn div(a: u8, b: u8) -> u8 {
    let result = if b == 0 {
        0
    } else {
        a / b
    };

    result
}

fn main() {
    let mut x = 42;
    //let x: u8 = 42;
    println!("x is {x}");

    let y = 10u16;
    let z = x as u16 + y;
    println!("z is {z}");

    x += 1;

    let x = div(1, 2);
    println!("x is {x}");

    let number_input = "42";
    let number_input: u8 = number_input.parse().unwrap();

    let mut a = 42;
    a += 1;
    let a = a;
    // ...

    let mut a = a;

    let b;
    // ...
    b = div(10, 5);
}
