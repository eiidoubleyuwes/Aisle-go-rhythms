//Implimenting a sieve of erastothene to find all prime numbers less than a given number.
//It should allow for user input to set the limit.


fn main (){
   let mut limit = String::new();
    println!("Enter the limit: ");
    std::io::stdin().read_line(&mut limit).unwrap();
    //using u32 for only positive integers
    let limit: u32 = limit.trim().parse().unwrap();
    //List of all numbers from 2 to the limit
    let mut numbers: Vec<u32> = (2..limit).collect();
    
}