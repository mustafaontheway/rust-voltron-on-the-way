fn main() {

    let mut ages: [u8; 10] = [0; 10];

    let length = ages.len();

    println!("Ages: {ages:?}");

    for i in 0..length {

        ages[i] = (i as u8) + 3;
    }

    println!("Ages: {ages:?}");
}

// Ages: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// Ages: [3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
