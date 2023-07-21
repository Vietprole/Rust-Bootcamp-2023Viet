// fn main() {
//     println!("Hello, Rust Bootcamp by VBI Academy!");
// }
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[derive(Hash)]
enum KeyTest{
    One,
    Two,
    Three,
    Four

}
fn main() {
    let mut hasher = DefaultHasher::new();
    let input: Vec<KeyTest> = vec![KeyTest::One, KeyTest::Two, KeyTest::Three, KeyTest::Four];
    input.hash(&mut hasher);
    let result = hasher.finish();
    println!("{}", result);
}