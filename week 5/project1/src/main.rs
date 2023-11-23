//Rust program to find the roots of a quadratic equation

use std::io;

fn main() {

    println!("Enter the value for a:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut a:f32 = input1.trim().parse().expect("Failed to read input");

    println!("Enter the value for b:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let mut b:f32 = input2.trim().parse().expect("Failed to read input");

    println!("Enter the value for c:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let mut c:f32 = input3.trim().parse().expect("Failed to read input");

    let d:f32 = 4.0;

    let calculation1:f32 = b.powf(2.0); 
    let calculation2:f32 = d * a * c;

    let calculation3:f32 = calculation1 - calculation2;
    let calculation4:f32 = 2.0 * a;

    let calculation5:f32 = -b +calculation3.sqrt();
    let calculation6:f32 = -b - calculation3;

    let calculation7:f32 = calculation5 / calculation4;
    let calculation8:f32 = calculation6 / calculation4;
    

    println!("The roots of the quadratic equation are: {},{}", calculation7,calculation8);


    let x = b.powf(2.0) - 4.0 * a * c;
    let x1 = b.powf(2.0) - 4.0 * a * c;
    let x2 = b.powf(2.0) - 4.0 * a * c;

    if x > 0.0 {
        println!("There are two distinct real roots");
    }
    else if x1 == 0.0 {
        println!("There is one repeated real root");
    }
    else if x2 < 0.0 {
        println!("There are no real roots");
    }
    else {
        println!("The equation does not have any real roots");
    }

}
