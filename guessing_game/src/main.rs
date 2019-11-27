use std::io;
use rand::Rng;

fn main() {
    println!("Tebaklah bilangannya!");
    println!("Ketiklah tebakan anda...");

    let toGuess = rand::thread_rng().gen_range(1,101);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Input gagal...");

    println!("Tebakkan anda adalah {}",guess);
}
