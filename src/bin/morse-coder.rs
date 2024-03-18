use std::io;

fn main() {
    //let mut buffer = String::new();
    ////io::stdin().read_line(&mut buffer)?;
    //let _ = io::stdin().read_line(&mut buffer);

    //// Now you can compare the content of buffer with another String
    //if buffer.trim() == "hello\n" {
    //    println!("You entered 'hello'");
    //} else {
    //    println!("You entered something else");
    //}
    //if buffer.trim() == "hello" {
    //    println!("2You entered 'hello'");
    //} else {
    //    println!("2You entered something else");
    //}

    loop {
        let mut code_choice = String::new();
        io::stdin()
            .read_line(&mut code_choice)
            .expect("Failed to read line");
        //let code_choice = code_choice.trim().to_string();
        //println!("1:code_choice={}",code_choice);
        //let code_choice = code_choice.lines().next().unwrap();
        //println!("2:code_choice={}",code_choice);

        if code_choice != "encode" || code_choice != "decode" {
            println!("33:!= evoke sub routine! {}", code_choice);
        }

        if code_choice.trim().to_string() != "encode" || code_choice.trim().to_string() != "decode"
        {
            println!("37:!= evoke sub routine! {}", code_choice);
        }

        if code_choice.trim() != "encode" || code_choice.trim() != "decode" {
            println!("41:!= evoke sub routine! {}", code_choice);
        }

        if code_choice.trim().to_string() != "encode" || code_choice.trim().to_string() != "decode"
        {
            println!("== evoke sub routine! {}", code_choice);
        } else {
            let code_choice = code_choice.trim().to_string();
            println!("{}", morse_code::code_translator(code_choice));
        }
    }
}
