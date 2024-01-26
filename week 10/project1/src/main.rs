//Rust program to calculate the total cost supposin a customer purchases 3 from each brand 

struct Laptop {
brand:String,
price:u32,
}

impl Laptop {
    fn calc_total(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let laptops = vec![
        Laptop { brand: String::from("Toshiba"), price: 550_000 },
        Laptop { brand: String::from("Dell"), price: 850_000 },
        Laptop { brand: String::from("HP"), price: 650_000 },
        Laptop { brand: String::from("IBM"), price: 755_000 },
        };


    let price_of_toshiba = toshiba.calculate_cost;
    let price_of_dell = dell.calculate_cost;
    let price_of_hp = hp.calculate_cost;
    let price_of_ibm = ibm.calculate_cost;

    let total_cost = price_of_hp + price_of_ibm + price_of_toshiba + price_of _dell;

      println!("Cost for laptops from each brand:{}",total_cost);

}



