fn main() {

    let mut base = YearCounter::new(2000);

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
    
    fn new(y: u16) -> Self {
        
        YearCounter { year: y }
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

// 2001
// 2000


