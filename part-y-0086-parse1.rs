fn main() {

    let p1 = sum_nums("3333", "2222");

    println!("Parse result 1: {p1}"); // Parse result 1: 5555

    let p2 = sum_nums("3333i", "2222");

    //println!("Parse result 2: {p2}"); // panic!

}

fn sum_nums(x: &str, y: &str) -> u64 {

    x.parse::<u64>().expect("Mustafa says parse error") + y.parse::<u64>().expect("Mustafa says parse error")
}


