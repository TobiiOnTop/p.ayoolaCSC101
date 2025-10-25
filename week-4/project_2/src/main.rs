use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Are you experienced? (yes/no)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experience_input = input1.trim();

    println!("Enter your age:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age: u32 = input2.trim().parse().expect("Not a valid number");

    let incentive: u32;

    if experience_input == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            incentive = 1_300_000;         }
    } else if experience_input == "no" {
        incentive = 100_000;
    } else {
        println!("Invalid response. Please type yes or no.");
        return;
    }

    println!("The annual incentive is: {}", incentive);
}
