fn main() {

    let mut ages = [17u8, 4, 41, 11, 58, 88, 77, 6, 96];

    for i in 0..ages.len() {

        if ages[i] >= 18 {

            ages[i] -= 5;
        }
    }

    println!("Ages: {:?}", ages);
}

// Ages: [17, 4, 36, 11, 53, 83, 72, 6, 91]
