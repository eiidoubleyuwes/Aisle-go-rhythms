//Implimenatation of the mod-exponent algorithm
// This algorithm is suppoded take in three parameters, base, exponent, and mod, and return base^exponent mod
// The algorithm is implemented using the divide and conquer approach

fn main() {
    //Allows user input for all three parameters
    let mut base = String::new();
    let mut exponent = String::new();
    let mut modu = String::new();
    println!("Enter the base: ");
    std::io::stdin().read_line(&mut base).unwrap();
    println!("Enter the exponent: ");
    std::io::stdin().read_line(&mut exponent).unwrap();
    println!("Enter the mod: ");
    std::io::stdin().read_line(&mut modu).unwrap();

    //Converts the string to integers
    let base: i32 = base.trim().parse().unwrap();
    let exponent: i32 = exponent.trim().parse().unwrap();
    let modu: i32 = modu.trim().parse().unwrap();
    
    //Converts the exponent to binary
    let mut binary: Vec<i32> = Vec::new();
    let mut exp = exponent;
    while exp > 0 {
        binary.push(exp % 2);
        exp = exp / 2;
    }
    binary.reverse();
    //Prints the binary
    println!("The Binary of the exponent : {:?}", binary);
    //Prints the result
    println!("The result of the mod-exponent algorithm is: {}", mod_exponent(base, binary, modu));
 
}

