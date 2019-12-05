use std::io;

fn main(){
    let mut input = String::new();
    println!("Enter a year: ");
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    if n % 4==0{
        if n % 100 == 0{
            if n % 400 == 0{
                println!("Yes, it is a leap year!");
            } else {
                println!("No, it is not a leap year!");
            }
        } else {
            println!("Yes, it is a leap year!");
        }
    } else {
        println!("No, it is not a leap year!");
    }
}