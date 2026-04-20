fn main() {

    let mut name = "Mustafa".to_string();

    let clone_name = name.clone();

    name = "Mustafa Büyükdereli".to_string();

    println!("{name}");
    println!("{clone_name}");
}

// Mustafa Büyükdereli
// Mustafa
