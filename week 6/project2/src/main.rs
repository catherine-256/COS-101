use std::io;

fn main() {

    println!("WELCOME TO THE RESEARCHERS PUBLICATION INCENTIVE SYSTEM");

    for _ in 0..500 {

        println!("Input your name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");

        println!("Input number of paper(s) published:");
        let mut paper = String::new();
        io::stdin().read_line(&mut paper).expect("Failed to read input");
        let p:i32 = paper.trim().parse().expect("Failed to input");

        let incentive1:i32 = 500_000;
        let incentive2:i32 = 800_000;
        let incentive3:i32 = 1_000_000;
        let incentive4:i32 = 100_000;

        if p >= 3 && p <= 5 {
            println!("
            your incentive is N{}", incentive1);
        }
        else if p > 5 && p <= 10 {
            println!("
            your incentive is N{}", incentive2);
        }
        else if p > 10 {
            println!("
            your incentive is N{}", incentive3);
        }
        else if p < 3 && p >= 1 {
            println!("
            your incentive is N{}", incentive4);
        }
        else {
            println!("
            Sorry, all the avaiable slots have been filled up.");
        }

    }

}
