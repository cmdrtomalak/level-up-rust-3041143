use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    red:   u8,
    green: u8,
    blue:  u8,
} // TODO: design data structure

#[derive(Debug)]
enum RgbError {
    IncorrectLength,
    InvalidCharacter(char),
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    // TODO: implement trait
    fn r(&self) -> u8 {
        self.red
    }

    fn g(&self) -> u8 {
        self.green
    }

    fn b(&self) -> u8 {
        self.blue
    }
}

impl FromStr for Rgb {
    // TODO: implement trait
    type Err = RgbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 7 {
            return Err(RgbError::IncorrectLength);
        }

        let (r, g, b) = (&s[1..3], &s[3..5], &s[5..7]);

        let mut rgb: Vec<u8> = vec![];

        for el in [r, g, b] {
            let mut digit_num = 0;
            let mut carry = 0;

            for hex_char in el.chars() {
                if carry == 1 {
                    digit_num *= 16;
                    carry = 0;
                }

                let n: u8 = match hex_char {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'a' => 10,
                    'b' => 11,
                    'c' => 12,
                    'd' => 13,
                    'e' => 14,
                    'f' => 15,
                    _ => return Err(RgbError::InvalidCharacter(hex_char)),
                };
                digit_num += n;
                carry = 1;
            }

            rgb.push(digit_num);
            digit_num = 0;
        }

        Ok(Rgb {
            red: rgb[0],
            green: rgb[1],
            blue: rgb[2],
        })
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    // 
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short () {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}

