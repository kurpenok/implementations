use std::io::Write;

use binary_search::binary_search;

fn main() {
    let mut s_array = String::new();
    print!("[>] Enter array: ");
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut s_array)
        .ok()
        .expect("[-] Failed to read array!");
    let array: Vec<i32> = s_array
        .trim()
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

    let mut s_target = String::new();
    print!("[>] Enter target: ");
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut s_target)
        .ok()
        .expect("[-] Failed to read target!");
    let target: i32 = s_target
        .trim()
        .parse::<i32>()
        .ok()
        .expect("[-] Failed to parse target!");

    match binary_search(&array, target) {
        Some(index) => println!("[+] Index of target: {}", index),
        None => println!("[-] Target doesn`t found!"),
    }
}
