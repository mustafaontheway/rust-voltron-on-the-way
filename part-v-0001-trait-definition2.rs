fn main() {

    let rect = Rectangle {width: 4.25, height: 3 as f32};
    let c = Circle {radius: 5.4};

    rect.calculate_area();
    rect.calculate_perimeter();

    c.calculate_area();
    c.calculate_perimeter();
}

trait ShapeGeo {

    fn calculate_area(&self);

    fn calculate_perimeter(&self);
}

struct Rectangle {

    width: f32,
    height: f32
}

struct Circle {

    radius: f32
}

impl ShapeGeo for Rectangle {

    fn calculate_area(&self) {
        println!("Width: {} - height: {} => Area: {}", self.width, self.height, self.width * self.height)
    }

    fn calculate_perimeter(&self) {
        println!("Width: {} - height: {} => Perimeter: {}", self.width, self.height, 2.0 * (self.width + self.height))
    }
}

impl ShapeGeo for Circle {

    fn calculate_area(&self) {
        println!("Radius: {} => Area: {}", self.radius, self.radius * self.radius * 3.14)
    }

    fn calculate_perimeter(&self) {
        println!("Radius: {} => Perimeter: {}", self.radius, 2.0 * self.radius * 3.14)
    }
}

// Width: 4.25 - height: 3 => Area: 12.75
// Width: 4.25 - height: 3 => Perimeter: 14.5
// Radius: 5.4 => Area: 91.56241
// Radius: 5.4 => Perimeter: 33.912003       
