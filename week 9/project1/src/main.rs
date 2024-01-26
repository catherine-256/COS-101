//Rust program to create a file that save the high quality category of drinks 

use std::io::Write;

fn main() {

   println!("WELCOME TO NIGERIA BREWERY LIMITED!");

    let drink_1 = "Lager:\n \n33 Export \n Desperados \nGoldberg,\nGulder \nHeineken \n Star\n";
    let drink_2 = "\nStout:\n \nLegend \n Turbo king\nWilliams\n";
    let drink_3 = "\nNon-Alcoholic:\n\nMaltina \n Amstel Malta \nMalta Gold \nFayrouz\n";


    let mut file = std::fs::File::create("drinks.txt").expect("Failed to read line");
     
         file.write_all(drink_1.as_bytes()).expect("Failed to read line");
             
             file.write_all(drink_2.as_bytes()).expect("Failed to read line");
    
              file.write_all(drink_3.as_bytes()).expect("Failed to read line");


    
    println!("\n Data written to file.");

    println!("THANK YOU FOR CHOOSING NIGERIA BREWERY LIMITED!");

}














































}