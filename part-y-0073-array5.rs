fn main() {

    let nums1 = [12, 32, 44, 76, 810000];

    let evens1 = even_nums(&nums1);

    println!("Even numbers 1 list: {evens1:?}");
}


fn even_nums(nums: &[i32]) -> Vec<i32> {

    let mut evens = Vec::new();

    for num in nums {

        if num % 2 == 0 {

            evens.push(*num);
        }
    }

    evens
}

// Even numbers 1 list: [12, 32, 44, 76, 810000]
