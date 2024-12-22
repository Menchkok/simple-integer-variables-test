use std::io;


fn main() {
    

    let mut first = String::new();
    let mut second = String::new();

    io::stdin().read_line(&mut first).expect("Error");
    io::stdin().read_line(&mut second).expect("Error");

    let mut first:i32 = first.trim().parse().expect("msg");
    let mut second:i32 = second.trim().parse().expect("msg");

    
    first = first + second;
    second = first - second;
    first = first - second;

    println!("{}",first);
    print!("{}",second);




}
