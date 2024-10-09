use std::any::{Any};

fn build_vector() -> Vec<i16> {
    let mut vector = Vec::new();
    vector.push(30);
    vector.push(45);
    vector
}

fn print_type_of<T: Any>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    println!("{:?}", build_vector());
    print_type_of(&build_vector());
}
