fn main() {

    let mut base = YearCounter::new();

    base.inc_year();

    println!("{:?}", base.get_year());

    base.dec_year();

    println!("{:?}", base.get_year());
}

#[derive(Debug)]
struct YearCounter {

    year: u16
}

impl YearCounter {
    
    fn new() -> Self {
        
        YearCounter { year: 2026 }
    }

    fn inc_year(&mut self) {

        self.year += 1
    }

    fn dec_year(&mut self) {

        self.year -= 1
    }

    fn get_year(&self) -> u16 {

        self.year
    }
}

// 2027
// 2026



