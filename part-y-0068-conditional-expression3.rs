fn main() {

    let some_nums = [-12i8, 0, 33, 24, 47, -5, 100];

    let (e, o) = return_vecs(&some_nums);

    println!("Even numbers: {:?}", e);

    println!("Odd numbers: {:?}", &o);
}

fn return_vecs(numbers: &[i8]) -> (Vec<i8>, Vec<i8>) {

    let mut evens: Vec<i8> = Vec::new();

    let mut odds: Vec<i8> = Vec::new();

    for num in numbers {

        if num % 2 == 0 {

            evens.push(*num);

        } else {
            
            odds.push(*num);
        }
    }

    (evens, odds)
}

// Even numbers: [-12, 0, 24, 100]
// Odd numbers: [33, 47, -5]
