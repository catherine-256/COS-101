use std::io;

fn main(){
    println!("
    
       for _ in 0..150{
        
        println!("
        Input your name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("
        Input your email:");
        let mut email = String::new();
        io::stdin().read_line(&mut s_mail).expect("Failed to read line");

        println!("
        Input your Department:");
        let mut department = String::new();
        io::stdin().read_line(&mut department).expect("Failed to read line");

        println!("
        Input your Level:");
        let mut level = String::new();
        io::stdin().read_line(&mut level).expect("Failed to read line");
        let l:i32 = level.trim().parse().expect("Failed to read line");

        println!("
        Are currently a course rep?:");
        let mut post = String::new();
        io::stdin().read_line(&mut post).expect("Failed to read line");
        let class_rep:bool = post.trim().parse().expect("Failed to read line");

        println!("
        Input your GPA:");
        let mut grade = String::new(); io::stdin().read_line(&mut grade).expect("Failed to read line");
        let g:f32 = grade.trim().parse().expect("Failed to line");
        
        println!("
        Enter your State of Origin:");
        let mut state = String::new();
        io::stdin().read_line(&mut state).expect("Failed to read line");


        if class_rep == true && l > 100 && g > 4.0 {
        println!("
    
            Proceed with the voting process");
        }
        else {
        println!("
        
            Sorry, you are not eligible to vote");
        }

}
