use std::{io};
use std::cmp::Ordering;
use rand::Rng;

fn main() {


    let secret_number: u32 = rand::thread_rng().gen_range(0..=100);
    let mut guesses: u32 = 0;
    println!("Informe um número");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler número!\n");

        println!("Você chutou: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        guesses += 1;

        match guess.cmp(&secret_number)  {
            Ordering::Equal => {
                println!("Você ganhou!\n");
                break;
            }
            Ordering::Greater => {
                println!("Está para baixo!");
                println!("Tente novamente");
            },
            Ordering::Less => {
                println!("Está para cima!");
                println!("Tente novamente");
            }
        }
    }

    println!("Você usou {} tentaivas",guesses);
}
