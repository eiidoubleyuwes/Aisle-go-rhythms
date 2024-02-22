//Implimenting a sieve of erastothene to find all prime numbers less than a given number.
//It should allow for user input to set the limit.


fn main (){
   let mut limit = String::new();
    println!("Enter the limit: ");
    std::io::stdin().read_line(&mut limit).unwrap();
    //using u32 for only positive integers
    let limit: u32 = limit.trim().parse().unwrap();
    //List of all numbers from 2 to the limit
    let numbers: Vec<u32> = (2..limit).collect();
    //List of all prime numbers
    let mut primes: Vec<u32> = Vec::new();

    //Iterate through the numbers list
    for number in numbers.iter(){
        //If the number is prime, add it to the primes list
        if is_prime(number){
            primes.push(*number);
            println!("{}", number);
        }
    } 
}
//Function to check if a number is prime
fn is_prime(number: &u32) -> bool{
    //Iterate through all numbers from 2 to the square root of the number
    for i in 2..((*number as f64).sqrt() as u32 + 1){
        //If the number is divisible by any of the numbers, it is not prime
        if number % i == 0{
            return false;
        }
        
    }
    //If the number is not divisible by any of the numbers, it is prime
    true
}

