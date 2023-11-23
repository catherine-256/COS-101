//Rust program that displays food items on a menu and takes orders from customers

use std::io;

fn main() {
    let p = "Poundo Yam/Edinkaiko Soup";
    let f = "Fried Rice & Chicken";
    let a = "Amala & Ewedu Soup";
    let e = "Eba & Egusi Soup";
    let w = "White Rice & Stew";
loop{
    println!("Welcome to Catherine's eatery                             
  Poundo Yam/Edinkaiko Soup(P)   ==>     N3,200
  Fried Rice & Chicken(F)        ==>     N3,000
  Amala & Ewedu Soup(A)          ==>     N2,500
  Eba & Egusi Soup(E)            ==>     N2,000
  White Rice & Stew(W)           ==>     N2,500
");

    
        let mut food = String::new();
    println!("Please place your order:");
    io::stdin().read_line(&mut food).expect("Failed to read input");
    let food = food.trim().to_lowercase();

     if food == "p" || food == "poundo yam/edinkaiko soup"
    { println!("how many portions of {} would you like",p);} 
    
    else if food == "f" || food == "fried rice & chicken"
    { println!("how many portions of {} would you like",f);} 
    
    else if food == "a" || food == "amala & ewedu soup"
    { println!("how many portions of {} would you like",a); }
    
    else if food == "e" || food == "eba & egusi soup"
    { println!("how many portions of {} would you like",e); }
    
    else if food == "w" || food == "white rice & stew"
    { println!("how many portions of {} would you like",w);}

    else {
        println!("The item you've requested for is not on the menu");continue;
    } 
    
        let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity:f32 = quantity.trim().parse().expect("Failed to read input");

    
    if food == "p" || food == "Poundo Yam/Edinkaiko Soup"
    {
        let calc:f32 = 3200.0 * quantity;
        if calc > 10000.0 {
            let calc:f32 = calc * 0.95;
            println!("You have qualified for the discount and your bill is: N{}", calc);break;
    }
    else {

        println!("Your total bill is N{}", calc);break;
    } }      
    else if food == "f" || food == "Fried Rice & Chicken"
    {
        let calc1:f32 = 3000.0 * quantity;
        if calc1 > 10000.0 {
            let calc1:f32 = calc1 * 0.95;
            println!("You qualified for the discount and your bill is: N{}", calc1);break;
    }
    else {

        println!("Your total bill is N{}", calc1);break;
    }}

    else if food == "a" || food == "Amala & Ewedu Soup"
    {
        let calc2:f32 = 2500.0 * quantity;
        if calc2 > 10000.0 {
            let calc2:f32 = calc2 * 0.95;
            println!("You have qualified for the discount and your bill is: N{}", calc2);break;
    }
    else {

        println!("Your total bill is N{}", calc2);break;
    }}

    else if food == "e" || food == "Eba & Egusi Soup"
    {
        let mut calc3:f32 = 2000.0 * quantity;
        if calc3 > 10000.0 {
            let calc3:f32 = calc3 * 0.95;
            println!("You have qualified for the discount and your bill is: N{}", calc3);break;
    }
    else {

        println!("Your total bill is N{}", calc3);break;
    }}

    else if food == "w" || food == "White Rice & Stew"
    {
    let mut calc4:f32 = 2500.0 * quantity;
    if calc4 > 10000.0 {
        let calc4:f32 = calc4 * 0.95;
        println!("You have qualified for the discount and your bill is: N{}", calc4);break;
    }
    else {

    println!("Your total bill is N{}", calc4);break;
    }
    }
    
}}