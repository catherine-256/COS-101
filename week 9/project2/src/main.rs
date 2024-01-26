//Rust program to manage student-related data 

fn main() {
    let name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold ", "Blanca Edemoh"];
    let matriculation_number = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec![300 , 100, 200, 200, 100 ];

     for i in 0..level.len(){

            println!("PAU STUDENT MANAGEMENT INFORMATION SYSTEM");

        println! ("          Student name : {}
          Matriculation number : {}
          Department : {}
          Level : {}\n", name[i], matriculation_number[i], department[i], level[i]);
        
    }


