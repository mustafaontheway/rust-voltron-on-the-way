fn main() {

    let ceo_info = ("Aykan Işık", "İzmir", true, 1965u16);

    let (full_name, .. , birth_year) = &ceo_info;

    println!("CEO full name: {full_name} and birth year: {birth_year}")
}

// CEO full name: Aykan Işık and birth year: 1965


// cargo run 
