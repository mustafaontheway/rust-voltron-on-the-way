fn main() {

    let mut ages = [17u8, 4, 41, 11, 58, 88, 77, 6, 96];

    for age in &mut ages { // &mut ages => address info, not value

        if *age >= 18 { // *age => value

            *age -= 5;
        }
    }

    println!("Ages: {:?}", ages)
}

// Ages: [17, 4, 36, 11, 53, 83, 72, 6, 91]
