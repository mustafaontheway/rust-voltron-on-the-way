fn main() {

    let mut person_data: (String, u8, u16) = ("Ayhan Bilir".to_string(), 29, 4200);

    person_data.2 = 4300;

    let person_data = person_data; // now immutable

    println!("{person_data:?}") // ("Ayhan Bilir", 29, 4300)
}

