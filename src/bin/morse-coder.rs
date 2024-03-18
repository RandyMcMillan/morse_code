use std::io;

fn main() {
    println!("Hello Welcome to No Re Morse, Just kidding this is a Morse Code translator/decoder");
    println!("----------------------------------------------------------------------------------");
    println!("Please enter morse code with ONE space between letters and TWO space between words\n(. - .--.  . - .--. = ETP ETP)");
    println!("Please enter words with ONE space between them and in all capital letters");
    loop {
        println!(
            "----------------------------------------------------------------------------------"
        );
        println!(
            "Enter decode to translate from morse code or encode to translate to morse code:   "
        );
        println!(
            "----------------------------------------------------------------------------------"
        );
        let mut code_choice = String::new();
        io::stdin()
            .read_line(&mut code_choice)
            .expect("Failed to read line");
        let code_choice = code_choice.trim().to_string();
        println!("New Message: {}", morse_code::code_translator(code_choice));

        println!("Run Again(Y?N): ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim().to_string();
        if choice.to_uppercase() != String::from("Y") {
            break;
        }
    }

    println!("Well Thanks For Using My CLI application");
    println!("If You Want to help make this project better contribute here: https://github.com/dmarcr1997/rust_morse_code_decoder_cli");
}
