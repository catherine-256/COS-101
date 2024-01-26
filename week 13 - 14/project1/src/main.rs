use std::io; 

fn main(){

println!("WELCOME TO GLOBACOM LTD!");

println!("THE USER TYPES ARE AS FOLLOWS:
*Administrator 
*Project manager 
*Employee 
*Customer 
*Vendor ");

println!("Enter your Usertype:}", );
let mut user_type = String::new();
io::stdin().read_line(&mut user).expect("Failed to read line");
let user_type = user_type.trim();

if user = "Administrator" {
let mut file = std::fs::File::open("Globacom_database.sql").unwrap():
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
  println!("Failed to read line")
}else if 
if user = "Project manager" {
let mut file = std::fs::File::open("project_table.sql").unwrap():
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
  println!("Failed to read line")
}else if 
if user = "Employee" {
let mut file = std::fs::File::open("staff_table.sql").unwrap():
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
  println!("Failed to read line")
}else if 
if user = "Customer" {
let mut file = std::fs::File::open("customer_table.sql").unwrap():
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
  println!("Failed to read line")
}else if 
if user = "Vendor" {
let mut file = std::fs::File::open("data_plan.sql").unwrap():
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
  println!("Failed to read line")
}
}
