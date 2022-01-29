use esp_idf_sys;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

extern "C" {
    pub fn esp_vfs_dev_uart_use_driver(
        uart_num: esp_idf_sys::uart_port_t
    ) -> ();
}

fn main() {
    esp_idf_sys::link_patches();

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
