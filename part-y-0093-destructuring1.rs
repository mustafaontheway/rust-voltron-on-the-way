fn main() {

    let ages = [22u8, 45, 98, 45];

    let oldest_ages: u8;

    [_, _, oldest_ages, _] = ages;

    assert_eq!(oldest_ages, ages[2]); 
}

