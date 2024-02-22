//This is my take on linear search in Rust
//I should print the array and let the user visualize the search


fn main() {
    //Change array to Vec to allow users to add numbers to the Vec
    let mut arr = Vec::new();
    //Let user choose how many numbers to add to the array
    let mut number = String::new();
    println!("Enter the number of numbers to add to the array: ");
    std::io::stdin().read_line(&mut number).unwrap();
    let number: usize = number.trim().parse().unwrap();
    //Allow user to write numbers to the array
    println!("Enter {number} numbers to the array: ");
    for _ in 0..number {
        let mut num = String::new();
        std::io::stdin().read_line(&mut num).unwrap();
        let num: i32 = num.trim().parse().unwrap();
        arr.push(num);
    }
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //Allow user to write the number to search
    let mut num = String::new();
    println!("Enter the number to search: ");
    std::io::stdin().read_line(&mut num).unwrap();
    let num: i32 = num.trim().parse().unwrap();
    let mut found = false;
    for i in 0..arr.len() {
        if arr[i] == num {
            println!("Number found at index {}", i);
            found = true;
            break;
        }
    }
    if !found {
        println!("Number not found");
    }
    //Print the array
    println!("Array: {:?}", arr);
}