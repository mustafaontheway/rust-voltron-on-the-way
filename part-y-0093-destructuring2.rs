fn main() {

    let ages = [2u8, 14, 98, 47, 87];

    let [child_age1, child_age2, ..] = ages;

    println!("Child age 1: {child_age1}");

    println!("Child age 2: {child_age2}");
}

// Child age 1: 2
// Child age 2: 14


// cargo run main.rs
