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
            let red = u8::from_str_radix(&input[1..=2], 16).unwrap();
            let green = u8::from_str_radix(&input[3..=4], 16).unwrap();
            let blue = u8::from_str_radix(&input[5..=6], 16).unwrap();
            return Color::new_rgb(red,green,blue);
        } else {
            let mut v: Vec<u16> = Vec::new();
            let mut num: u16 = 0;
            let mut flag : bool = false;
            for i in input.chars() {
                if i.is_numeric() {
                    num = num * 10 + i.to_digit(10).unwrap() as u16;
                    flag = true;
                } else if flag{
                    v.push(num);
                    num = 0;
                    flag = false;
                }

            }
            return Color::new_hsv(v[0], v[1] as u8, v[2] as u8);
        }
    }
}
