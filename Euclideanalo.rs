//This is my take on the Euclidean Algorithm in rust
/*
The Algorithm works as follows:
It uses GCD of the two numbers using the formula: b=aq+r and a = cq1+r1.
This process continues until the last non-zero reminder

This is a simple algorithm
*/
use std::io;

fn main(){
    println!("Hello there enter the first number you want to calculate the GCD");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read value");
    println!("Enter the second number you want to calculate the GCD");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read value");
    println!("You have entered {} and {}",a,b);
    

}