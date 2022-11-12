use std::str::FromStr;


#[derive(Debug, PartialEq)]

pub enum Color {
    RGB {
        red: u8,
        green: u8,
        blue: u8
    },
    HSV {
        hue: u16,
        saturation: u8,
        value: u8,
    }
}

impl Color {
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Self::RGB { red, green, blue }
    }

    pub fn new_hsv(hue: u16, saturation: u8, value: u8) -> Color {
        if hue > 360 || saturation > 100 || value > 100 {
            panic!("Invalid input");
        }

        Self::HSV { hue, saturation, value }
    } 


        pub fn from_str(input: &str) -> Self {
            if input.chars().nth(0) == Some('#') {
                Color::new_rgb( u8::from_str_radix(&input[1..3], 16).unwrap() , u8::from_str_radix(&input[3..5], 16).unwrap(), u8::from_str_radix(&input[5..], 16).unwrap())
            } else {
                let numbers: Vec<&str> = input[4.. input.len() - 1].split(",").collect();
                Color::new_hsv(numbers[0].parse::<u16>().unwrap(), numbers[1][0.. numbers[1].len() - 1].parse::<u8>().unwrap(), numbers[2][0.. numbers[2].len() - 1].parse::<u8>().unwrap())
            }
        }
    
    
}