fn main() {

    let mut initial_coor = Coordinates::new(2.13, 3.456, 12.45);

    initial_coor.print_coordinates();

    initial_coor.update_coordinates(-3.21, -4.45, -9.99);

    initial_coor.print_coordinates();
}

#[derive(Debug)]
struct Coordinates {

    x: f32,
    y: f32,
    z: f32
}

impl Coordinates {

    fn new(x: f32, y: f32, z: f32) -> Self {

        Self { x, y, z }
    }

    fn update_coordinates(&mut self, x: f32, y: f32, z: f32) {

        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn print_coordinates(&self) {

        println!("x coor: {:?}", self.x);
        println!("y coor: {:?}", self.y);
        println!("z coor: {:?}", self.z);
    }
    
}
