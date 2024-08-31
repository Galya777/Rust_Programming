use std::collections::HashMap;
use std::io::{Write, Read, BufReader, BufRead};

#[derive(Debug)]
pub enum InterpreterError {
    RuntimeError { line_number: u16, message: String },
    UnknownVariable { name: String },
    NotANumber { value: String },
    SyntaxError { code: String },
    IoError(std::io::Error),
}

impl From<std::io::Error> for InterpreterError {
    fn from(source: std::io::Error) -> Self {
        Self::IoError(source)
    }
}

#[derive(Debug)]
enum Statement {
    Print { value: String },
    Read { var_name: String },
    Goto { line_number: u16 },
    If { condition: String, line_number: u16 },
}

pub struct Interpreter<'a, R: Read, W: Write> {
    code_lines: HashMap<u16, Statement>,
    variables: HashMap<String, u16>,
    input: BufReader<R>,
    output: &'a mut W,
}

impl<'a, R: Read, W: Write> Interpreter<'a, R, W> {
    pub fn new(input: R, output: &'a mut W) -> Self {
        Self {
            code_lines: HashMap::new(),
            variables: HashMap::new(),
            input: BufReader::new(input),
            output,
        }
    }

    pub fn add(&mut self, code: &str) -> Result<u16, InterpreterError> {
        let (line_number, statement) = parse_code_line(code)?;

        self.code_lines.insert(line_number, statement);

        Ok(line_number)
    }

    pub fn run(&mut self) -> Result<(), InterpreterError> {
        let mut line_numbers = self.code_lines.keys().cloned().collect::<Vec<_>>();
        line_numbers.sort();

        let Some(&(mut current_line_number)) = line_numbers.get(0) else {
            return Ok(());
        };

        macro_rules! runtime_error {
            ($($arg:tt)*) => {
                InterpreterError::RuntimeError {
                    line_number: current_line_number,
                    message: format!($($arg)*),
                }
            }
        }

        loop {
            let statement = self.code_lines.get(&current_line_number).unwrap();

            match statement {
                Statement::Print { value } => {
                    match self.eval_value(value) {
                        Ok(number) => writeln!(self.output, "{}", number)?,
                        Err(InterpreterError::UnknownVariable { name }) => {
                            return Err(runtime_error!("Unknown variable: {name}"));
                        },
                        _ => writeln!(self.output, "{}", value)?,
                    }
                },
                Statement::Read { var_name } => {
                    let mut user_input = String::new();
                    self.input.read_line(&mut user_input)?;
                    let user_input = user_input.trim();

                    let value = user_input.parse().
                        map_err(|_| runtime_error!("Not a number: {user_input}"))?;

                    self.variables.insert(var_name.clone(), value);
                },
                Statement::Goto { line_number } => {
                    if self.code_lines.contains_key(line_number) {
                        current_line_number = *line_number;
                        continue;
                    } else {
                        return Err(runtime_error!("Invalid line number for GOTO: {line_number}"));
                    }
                },
                Statement::If { condition, line_number } => {
                    let mut parts = condition.split_whitespace();

                    let left = self.eval_value(parts.next().unwrap()).
                        map_err(|e| runtime_error!("{:?}", e))?;
                    let op = parts.next().unwrap();
                    let right = self.eval_value(parts.next().unwrap()).
                        map_err(|e| runtime_error!("{:?}", e))?;

                    let result = match op {
                        ">" => if left > right { 1 } else { 0 },
                        "<" => if left < right { 1 } else { 0 },
                        "=" => if left == right { 1 } else { 0 },
                        _ => unreachable!(),
                    };

                    if result > 0 {
                        if self.code_lines.contains_key(line_number) {
                            current_line_number = *line_number;
                            continue;
                        } else {
                            return Err(runtime_error!("Invalid line number for GOTO: {line_number}"));
                        }
                    }
                },
            }

            let line_number_index = line_numbers.binary_search(&current_line_number).
                map_err(|_| runtime_error!("Couldn't find line number {current_line_number}"))?;

            if let Some(line_number) = line_numbers.get(line_number_index + 1) {
                current_line_number = *line_number;
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn eval_value(&self, value: &str) -> Result<u16, InterpreterError> {
        let first_char = value.chars().next().unwrap();

        if first_char.is_uppercase() {
            self.variables.get(value).
                ok_or_else(|| InterpreterError::UnknownVariable { name: value.to_string() }).
                map(|n| *n)
        } else {
            value.trim().parse().
                map_err(|_| InterpreterError::NotANumber { value: value.to_string() })
        }
    }
}

fn parse_code_line(input: &str) -> Result<(u16, Statement), InterpreterError> {
    macro_rules! syntax_error {
        () => { InterpreterError::SyntaxError { code: input.to_owned() } }
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    let line_number = parts.get(0).
        ok_or_else(|| syntax_error!())?.
        parse().
        map_err(|_| syntax_error!())?;

    let statement =
        match parts.get(1) {
            Some(&"PRINT") => {
                let value = parts.get(2).ok_or_else(|| syntax_error!())?.to_string();
                if parts.len() > 3 {
                    return Err(syntax_error!());
                }
                Statement::Print { value }
            },
            Some(&"READ") => {
                let var_name = parts.get(2).ok_or_else(|| syntax_error!())?.to_string();
                if !var_name.chars().next().unwrap().is_uppercase() {
                    return Err(syntax_error!());
                }
                if parts.len() > 3 {
                    return Err(syntax_error!());
                }
                Statement::Read { var_name }
            },
            Some(&"GOTO") => {
                let line_number = parts.get(2).ok_or_else(|| syntax_error!())?.to_string().
                    parse().map_err(|_| syntax_error!())?;
                if parts.len() > 3 {
                    return Err(syntax_error!());
                }
                Statement::Goto { line_number }
            },
            Some(&"IF") => {
                let condition = parts[2..=4].join(" ");

                if !matches!(parts.get(5), Some(&"GOTO")) {
                    return Err(syntax_error!());
                }

                let line_number = parts.get(6).ok_or_else(|| syntax_error!())?.to_string().
                    parse().map_err(|_| syntax_error!())?;

                if parts.len() > 7 {
                    return Err(syntax_error!());
                }

                Statement::If { condition, line_number }
            },
            _ => { return Err(syntax_error!()) }
        };

    Ok((line_number, statement))
}
