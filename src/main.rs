mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut cipher_text = String::new();
        let mut buffer = String::new();

        for ch in text.chars() {
            if (!buffer.is_empty() && !buffer.contains(ch)) || buffer.len() == 9 {
                // changing element, input encoded buffer into final string
                cipher_text.push_str(&buffer.len().to_string());
                cipher_text.push(buffer.pop().unwrap());

                buffer = String::new();
            }

            buffer.push(ch);
        }

        // input the final elements
        cipher_text.push_str(&buffer.len().to_string());
        cipher_text.push(buffer.pop().unwrap());

        cipher_text
    }
    
    pub fn decode(text: &str) -> String {
        let mut plaintext = String::new();

        let mut n_char = 0;
        let mut buf = vec![];
        for ch in text.chars() {
            n_char += 1;
            buf.push(ch);
            if n_char > 1 {
                let mut repeat = buf[0] as u8 - 48;
                while repeat > 0 {
                    plaintext.push(buf[1]);
                    repeat -= 1;
                }
                buf = vec![];
                n_char = 0;
            }

        }

        plaintext
    }
}

fn main() {
    //
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    println!("{}", decode(&encode(input)));
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
