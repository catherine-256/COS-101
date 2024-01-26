fn main(){
    //an arrray without data type 
    let mut a = [5, 4, 3, 2, 1];

    // an array with data type and size 
    let mut b:  [i32; 5] = [1, 2, 3, 4, 5];


    //an array with default values 
    let mut c = [3;5];

    println!("a = {:?}",a);
    println!("b={:?}", b);
    println!("c= {:?}", c);

println!("{}", a[0]);

println!("{}", a[4]);

println!("{}", a[0] + a[4]);

a[4] = 9;
c[0] = 6;

println!("a = {:?}",a);
println!("c = {:?}",c);
let mut a_sum = 0;
let mut b_sum = 0;
let mut c_sum = 0;

for index in 0..5 {

    println!("{}", b[index] * 100 );

    a_sum += a[index];
    b_sum += b[index];
     c_sum += c[index];
}


println!("{}", a_sum);
println!("{}", b_sum);
println!("{}", c_sum);

println!("{}", a.len());
println!("{}", b.len());
println!("{}", c.len());


}