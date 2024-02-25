//This is my simple take on the binary search algorithm in Rust
//The function takes in a vector of integers and a target integer
//It returns the index of the target integer in the vector if it exists
//Otherwise, it returns -1

fn binary_search(arr: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return mid as i32;
        }
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1
}