// Yep, this is yet another cat themed program!! Enjoy!! 
use std::io;   

fn main() {
    println!("CAT AGE CALCULATOR!");
    println!("Find out how old your cat is in human years!\n");

    // Get the cat's age from user
    println!("How old is your cat? (in cat years): ");
    
    let mut input = String::new();           // Mutable string for input
    io::stdin().read_line(&mut input)        // Read what user types
        .expect("Failed to read input");     // Handle errors

    // Convert text to number
    let cat_age: i32 = input                // Type annotation: i32 - a 32 bit intenger that stores both positive and negative numbers.
        .trim()                              // Remove spaces/newline!
        .parse()                            // Convert to number!
        .expect("Please enter a valid number!");

    // Ze calculation time!
    let human_age = if cat_age == 1 {
        15   // First year = 15 human years!
    } else if cat_age == 2 {
        24   // Two years = 24 human years!
    } else if cat_age > 2 {
        24 + (cat_age - 2) * 4   // 24 + extra years × 4!
    } else {
        0   // Kitten under 1 year!
    };

    // Results:
    println!("\nCALCULATION COMPLETE!");
    println!("Your {} year old cat is {} in human years!", cat_age, human_age);

    // Give personality based on age!
    if human_age < 15 {
        println!("Stage: Kitten! Full of energy and chaos!");
    } else if human_age < 25 {
        println!("Stage: Young Adult! Still playful but wiser!");
    } else if human_age < 45 {
        println!("Stage: Adult Cat! Dignified napper!");
    } else if human_age < 65 {
        println!("Stage: Mature Senior! Deserves extra treats!");
    } else {
        println!("Stage: ELDER! A living legend! Respect the wisdom!");
    }
}
