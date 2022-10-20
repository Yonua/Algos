mod difficult;
mod easy;

fn main() {
    println!(
        "Hello, world {}!",
        difficult::_2246(vec![1, 2], String::from(""))
    );
    println!("{}", easy::_2220(0, 1));
}
