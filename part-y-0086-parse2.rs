fn main() {

    let user_age = "17";

    let user_age_u8 = user_age.parse::<u8>().expect("Not a number");

    println!("User age: {user_age_u8}");

    let mut user_age_i8 = user_age.parse::<i8>().unwrap_or(0);

    user_age_i8 = -5; // u8 is more logical dtype

    println!("User age: {user_age_i8}");
}

// User age: 17
// User age: -5
