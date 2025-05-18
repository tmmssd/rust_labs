use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::rng();
    let secret_number: u8 = rng.random();
    println!("Secret number is {}", secret_number);
}
