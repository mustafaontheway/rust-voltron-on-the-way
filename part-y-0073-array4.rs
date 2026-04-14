fn main() {

    let mut ages: [u8; 10] = [11, 18, 10, 13, 26, 46, 33, 8, 99, 55];

    for age in &mut ages {

        if *age >= 18 {

            *age -= 4;
        }
    }

    println!("{:?}", ages);
}

// [11, 14, 10, 13, 22, 42, 29, 8, 95, 51]
