fn main() {

    let mut evens: Vec<u8> = Vec::new();

    let mut odds: Vec<u8> = Vec::new();

    for num in (10..=37).rev() {

        if num % 2 == 0 {

            evens.push(num);

        } else {

            odds.push(num);
        }
    }

    println!("Even numbers: {:?}", evens);

    println!("Odd numbers: {:?}", odds);
}

// Even numbers: [36, 34, 32, 30, 28, 26, 24, 22, 20, 18, 16, 14, 12, 10]
// Odd numbers: [37, 35, 33, 31, 29, 27, 25, 23, 21, 19, 17, 15, 13, 11]
