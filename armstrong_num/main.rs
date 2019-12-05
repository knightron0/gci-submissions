use std::io;

fn main(){
	let mut input = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).unwrap();
    let mut n: i32 = input.trim().parse().unwrap();
    let ogn: i32 = n;
    let mut pw:u32 = input.len() as u32;
    pw -= 1;
    let mut ans: i32 = 0;
    while n != 0 {
        let bse:i32 = n % 10;
        ans += bse.pow(pw);
        n = n/10;
    }
    if ans == ogn {
        println!("Yes, it is an Armstrong number");
    } else {
        println!("No, it is not an Armstrong number");
    }
}