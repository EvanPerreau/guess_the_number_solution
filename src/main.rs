use rand::random;

fn main() {
    let random_number = random::<u8>();
    let mut guess = String::new();
    println!("Guess the number!");
    while guess.trim() != &random_number.to_string() {
        guess.clear();
        std::io::stdin().read_line(&mut guess).unwrap();
        if guess.trim() < &random_number.to_string() {
            println!("Too small!");
        } else if guess.trim() > &random_number.to_string() {
            println!("Too big!");
        }
    }
    println!("You win!");
}
