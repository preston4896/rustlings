// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let message = String::from("hello");
    call_me(&message);
}

fn call_me(num: &String) {
    for i in 0..1 {
        println!("Message: {}", num);
    }
}
