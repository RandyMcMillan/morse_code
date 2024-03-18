use std::io;

fn main() {
    loop {
        let mut code_choice = String::new();
        io::stdin()
            .read_line(&mut code_choice)
            .expect("Failed to read line");
        let code_choice = code_choice.trim().to_string();
        println!("{}", morse_code::code_translator(code_choice));
    }
}
