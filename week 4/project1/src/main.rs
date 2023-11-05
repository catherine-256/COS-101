// Rust program to calculate the speed of a car in kilometers

use std::io;

fn main() {
   
   let mut input1 = String::new();    
   let mut input2 = String::new();  

   println!("Enter the distance in miles:");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let d:f32 = input1.trim().parse().expect("Not a vaild number");

   println!("Enter the time in hours:");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let t:f32 = input1.trim().parse().expect("Not a vaild number");  

   let s:f32 = d / t;

   println!("Average speed: {}", s);
}
