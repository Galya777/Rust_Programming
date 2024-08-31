use std::fmt;

pub struct Room {
    pub name: String,
    pub north: String,
    pub south: String,
    pub east: String,
    pub west: String,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let middle_left = format!("{} - |", self.west);
        let middle_left_len = middle_left.chars().count();

        let (name, name_len) = pad_odd(&self.name);
        let (north, _) = pad_odd(&self.north);
        let (south, _) = pad_odd(&self.south);

        let middle = format!("{} {} | - {}", middle_left, name, self.east);

        let north_roof = format!("+{:-^len$}+", "N", len = name_len + 2);
        let south_roof = format!("+{:-^len$}+", "S", len = name_len + 2);
        let roof_offset = " ".repeat(middle_left_len - 1);
        let pipe_offset = " ".repeat(roof_offset.len() + north_roof.len() / 2);
        let inner_roof_width = north_roof.len() - 2;

        // Use new format strings syntax: https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html
        writeln!(f)?;
        writeln!(f, "{roof_offset}[{north:^inner_roof_width$}]")?;
        writeln!(f, "{pipe_offset}|")?;
        writeln!(f, "{roof_offset}{north_roof}")?;
        writeln!(f, "{middle}")?;
        writeln!(f, "{roof_offset}{south_roof}")?;
        writeln!(f, "{pipe_offset}|")?;
        write!(f, "{roof_offset}[{south:^inner_roof_width$}]")?;

        Ok(())
    }
}

fn pad_odd(input: &str) -> (String, usize) {
    let char_count = input.chars().count();

    if char_count % 2 == 0 {
        (format!("{} ", input), char_count + 1)
    } else {
        (input.to_owned(), char_count)
    }
}
