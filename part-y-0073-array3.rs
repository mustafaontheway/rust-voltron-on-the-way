fn main() {

    let ages: [u8; 10] = [11, 18, 10, 13, 26, 46, 33, 8, 99, 55];

    let mut voters = Vec::new();

    for age in ages {

        if age >= 18 {

            voters.push(age);
        }
    }

    println!("Voters ages list: {voters:?}");
}

// Voters ages list: [18, 26, 46, 33, 99, 55]
