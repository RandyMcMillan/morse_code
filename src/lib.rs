use std::collections::HashMap;
use std::io;
struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

impl MorseDecoder {
    pub fn convert_to_morse(&self, message: &str) -> String {
        let mut refined: Vec<String> = vec![];
        for c in message.chars() {
            refined.push(c.to_string());
        }
        let mut decoded_string: Vec<String> = vec![];
        for code in refined {
            if code == " " {
                decoded_string.push("  ".to_string());
            } else {
                for (k, v) in self.morse_code.iter() {
                    if v.to_string() == code {
                        decoded_string.push(k.to_string());
                        decoded_string.push(" ".to_string());
                        break;
                    }
                }
            }
        }
        return decoded_string.join("").trim().to_string();
    }

    pub fn decode_morse(&self, encoded: &str) -> String {
        let refined: Vec<String> = encoded
            .split(" ")
            .map(|s| s.parse().expect("parse error"))
            .collect();
        let mut decoded_string: Vec<String> = vec![];
        let mut space_count = 0;
        for code in refined {
            if code == "" {
                if space_count < 1 {
                    space_count += 1;
                    decoded_string.push(" ".to_string());
                } else {
                    space_count = 0;
                }
            } else {
                for (k, v) in self.morse_code.iter() {
                    if k.to_string() == code {
                        decoded_string.push(v.to_string());
                        break;
                    }
                }
            }
        }
        return decoded_string.join("").trim().to_string();
    }
}

pub fn code_translator(choice: String) -> String {
    let morse_key = vec![
        ".-".to_string(),
        "-...".to_string(),
        "-.-.".to_string(),
        "-..".to_string(),
        ".".to_string(),
        "..-.".to_string(),
        "--.".to_string(),
        "....".to_string(),
        "..".to_string(),
        ".---".to_string(),
        "-.-".to_string(),
        ".-..".to_string(),
        "--".to_string(),
        "-.".to_string(),
        "---".to_string(),
        ".--.".to_string(),
        "--.-".to_string(),
        ".-.".to_string(),
        "...".to_string(),
        "-".to_string(),
        "..-".to_string(),
        "...-".to_string(),
        ".--".to_string(),
        "-..-".to_string(),
        "-.--".to_string(),
        "--..".to_string(),
    ];

    let letters = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
        "F".to_string(),
        "G".to_string(),
        "H".to_string(),
        "I".to_string(),
        "J".to_string(),
        "K".to_string(),
        "L".to_string(),
        "M".to_string(),
        "N".to_string(),
        "O".to_string(),
        "P".to_string(),
        "Q".to_string(),
        "R".to_string(),
        "S".to_string(),
        "T".to_string(),
        "U".to_string(),
        "V".to_string(),
        "W".to_string(),
        "X".to_string(),
        "Y".to_string(),
        "Z".to_string(),
    ];

    let decoder = MorseDecoder {
        morse_code: morse_key.into_iter().zip(letters.into_iter()).collect(),
    };

    if choice.to_uppercase() == String::from("ENCODE") {
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("EXAMPLE E X A M P L E");
        return decoder.convert_to_morse(&message.trim());
    } else if choice.to_uppercase() == String::from("DECODE") {
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect(". -..- .- -- .--. .-.. .   .   -..-   .-   --   .--.   .-..   .");
        return decoder.decode_morse(&message.trim());
    } else {
        return String::from("Try:\nencode\nEXAMPLE E X A M P L E\nOR\ndecode\n. -..- .- -- .--. .-.. .   .   -..-   .-   --   .--.   .-..   .");
    }
}
