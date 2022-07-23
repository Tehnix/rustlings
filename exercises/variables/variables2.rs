// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

fn type_of<T>(_: T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn main() {
    let x = 2; // inferred to be i32
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
    println!("x is type {}", type_of(x));
}
