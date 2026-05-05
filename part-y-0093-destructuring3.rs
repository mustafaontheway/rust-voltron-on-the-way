fn main() {

    let ages = [2u8, 14, 98, 47, 87];

    let [.., last_age] = ages;

    println!("Last age of array: {last_age}");

}

// Last age of array: 87
