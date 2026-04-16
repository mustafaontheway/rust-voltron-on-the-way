fn main() {

    let mut evens: Vec<u8> = Vec::new();

    let mut odds: Vec<u8> = Vec::new();

    for num in 10u8..=37 {

        if num % 2 == 0 {

            evens.push(num);

        } else {

            odds.push(num);
        }
    }

    println!("Even numbers: {:?}", evens);

    println!("Odd numbers: {:?}", odds);
}

// Even numbers: [10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36]
// Odd numbers: [11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37]
