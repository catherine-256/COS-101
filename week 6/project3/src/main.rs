use std::io;

fn main() {
    
    let mut input = String::new();
    println!(" Enter the value of n:");
    io::stdin().read_line(&mut input).expect("Failed to read line");


      let n:u32 = input.trim().parse().expect("Failed to read line");


    for x in 1..=n {
        for y in 1..=12 {
            let z = x * y;
            println!("{} timestable", x);
         println!(" {} x {} = {}", x, y, z);
        }
    }
}
