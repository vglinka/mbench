#[macro_use]
extern crate mbench;

fn my_func(x: u32) { vec![x; u16::MAX as usize].sort(); }

fn main() {
    let mut text = String::new();
    microbench!(
        text = "Hello world!".to_string();
        my_func(42);
        // ... code ....
    );
    println!("{text}");
}
