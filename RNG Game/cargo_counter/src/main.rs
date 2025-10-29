fn main() {
    println!("Welcome to the RNG Game!");

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 5 {
            break "You win!";
        }
    };
    println!("{}", result);
}
