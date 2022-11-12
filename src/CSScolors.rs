

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
    /// Конструира нова стойност от вариант `RGB` с дадените стойности за червено, зелено и синьо.
    ///
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        crate::CSScolors::Color::RGB{red, green, blue}
    }

    /// Конструира нова стойност от вариант `HSV` с дадените стойности.
    ///
    /// В случай, че hue е над 360 или saturation или value са над 100, очакваме да `panic!`-нете с
    /// каквото съобщение си изберете.
    ///
    pub fn new_hsv(hue: u16, saturation: u8, value: u8) -> Color {
        if hue <=360 && saturation<=100 && value<=100{
            crate::CSScolors::Color::HSV{hue, saturation, value}
        } else{
            panic!("Too high values!");       
        }
        
    }

      /// Ако `self` е `RGB`, тогава връщате неговите `red`, `green`, `blue` компоненти в този ред.
    /// Иначе, `panic!`-вате с каквото съобщение си изберете.
    ///
    pub fn unwrap_rgb(&self) -> (u8, u8, u8) {
       match &self{
       Color::RGB { red, green, blue } => return (*red, *green, *blue),  
        Color::HSV { hue, saturation, value } =>  panic!("Not a RGB!"),     
       }
    }

    /// Ако `self` е `HSV`, тогава връщате неговите `hue`, `saturation`, `value` компоненти в този
    /// ред. Иначе, `panic!`-вате с каквото съобщение си изберете.
    ///
    pub fn unwrap_hsv(&self) -> (u16, u8, u8) {
       match &self{
       Color::RGB { red, green, blue } => panic!("Not a HSV!"), 
        Color::HSV { hue, saturation, value } => return (*hue, *saturation, *value),     
       }
    }

     /// В случай, че варианта на `self` е `RGB`, очакваме низ със съдържание `#rrggbb`, където
    /// червения, зеления и синия компонент са форматирани в шестнадесетична система, и всеки от тях е
    /// точно два символа с малки букви (запълнени с нули).
    ///
    /// Ако варианта е `HSV`, очакваме низ `hsv(h,s%,v%)`, където числата са си напечатани в
    /// десетичната система, без водещи нули, без интервали след запетаите, вторите две завършващи на
    /// `%`.
    ///
    pub fn to_string(&self) -> String {
        match &self{
            Color::RGB { red, green, blue } => 
            return String::from("#")+ &(*red as u16).to_string() + &(*green as u16).to_string() + &(*blue as u16).to_string()
            , 
             Color::HSV { hue, saturation, value } => 
             return String::from("hsv(") + &hue.to_string() + "," + &saturation.to_string() + "% , " +
             &value.to_string() + &String::from("% )")
             ,     
            }
    }

       /// Инвертира цвят покомпонентно -- за всяка от стойностите се взема разликата с максимума.
    ///
    pub fn invert(&self) -> Self {
        match &self{
            Color::RGB { red, green, blue } => return Color::RGB{ red: 255-red, green: 255-green, blue: 255-blue },  
            Self::HSV { hue, saturation, value } => {
                Self::HSV { hue: 360 - hue, saturation: 100 - saturation, value: 100 - value }
            }    
            }
    }
}

#[test]
fn test_basic() {
    let color1 = Color::new_rgb(0, 0, 0);
    assert_eq!(color1.unwrap_rgb().0, 0);
    assert_eq!(&color1.to_string()[0..1], "#");

    let color2 = Color::new_hsv(0, 0, 0);
    assert_eq!(color2.unwrap_hsv().0, 0);

    assert_eq!(color1.invert().unwrap_rgb().0, 255);
}