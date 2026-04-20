fn main() {

    let mut name = "Mustafa".to_string();

    {
        let name_ref1 = &mut name;

        name_ref1.insert_str(7, " Buyukdereli");

        println!("{name_ref1}");

        println!("{name}");

        println!("Scope ended!");
    }

    let name_ref2 = &mut name;

    *name_ref2 = "Mustafa B".to_string();

    println!("{name_ref2}");

    println!("{name}");
}

// Mustafa Buyukdereli
// Mustafa Buyukdereli
// Scope ended!
// Mustafa B
// Mustafa B
