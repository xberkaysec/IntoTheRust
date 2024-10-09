use std::any::{Any};

fn build_vector() -> Vec<i16> {
    let mut vector = Vec::<i16>::new();
    vector.push(30i16);
    vector.push(45i16);
    vector
}

fn print_type_of<T: Any>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    println!("{:?}", build_vector());
    print_type_of(&build_vector());
}
