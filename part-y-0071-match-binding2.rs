fn main() {

    let sales: u16 = 30000;

    let premium = match sales {
        
        s if s > 25_000 => 0.03 * s as f32,
        s if s > 15_000 => 0.02 * s as f32,
        _ => 0.0
    };

    println!("{}", premium) // 900
}

