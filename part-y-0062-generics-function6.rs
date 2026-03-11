fn main() {

    let v1 = create_vector(3.12f32, 1.8, 5.89);

    println!("{:?}", v1); // [3.12, 1.8, 5.89]

    let v2 = create_vector("mustafa", "ayhan", "bengü");

    println!("{:?}", v2); // ["mustafa", "ayhan", "bengü"]
}

fn create_vector<T>(x: T, y: T, z: T) -> Vec<T> {

    vec![x, y, z]
}
