fn main() {

    let mut last_year_sales = quarterly_sales_usd(100_000, 97_600, 99_400, 105_000);

    println!("{:?}", last_year_sales); // [100000, 97600, 99400, 105000]

    last_year_sales[1] = 96_300;

    println!("{:?}", last_year_sales); // [100000, 96300, 99400, 105000]

}

pub fn quarterly_sales_usd(q1: u32, q2: u32, q3: u32, q4: u32) -> Vec<u32> {
    
    vec![q1, q2, q3, q4]
}
