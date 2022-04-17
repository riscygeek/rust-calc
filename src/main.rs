extern crate num;
use std::io::Write;
use std::io;

fn advance(chars: &mut Vec<char>, n: usize) {
    *chars = chars[n..].to_vec()
}

fn skip_ws(chars: &mut Vec<char>) {
    while chars[0].is_whitespace() {
        advance(chars, 1);
    }
}

fn atom(chars: &mut Vec<char>) -> Result<i32, String> {
    let mut value = 0i32;

    skip_ws(chars);

    if chars[0] == '(' {
        advance(chars, 1);
        value = expr(chars)?;
        if chars[0] != ')' {
            return Err(format!("Expected ')', got '{}'", chars[0]));
        }
        advance(chars, 1);
    } else if chars[0].is_digit(10) {
        while chars[0].is_digit(10) {
            value = value * 10 + (chars[0] as i32) - ('0' as i32);
            advance(chars, 1);
        }
    } else {
        return Err(format!("Expected '(', or digit, got '{}'", chars[0]));
    }

    skip_ws(chars);

    Ok(value)
}

fn power(chars: &mut Vec<char>) -> Result<i32, String> {
    let mut left = atom(chars)?;

    while chars[0] == '*' && chars[1] == '*' {
        advance(chars, 2);

        let right = atom(chars)?;

        left = num::pow(left, right as usize);
    }
    Ok(left)
}

fn unary(chars: &mut Vec<char>) -> Result<i32, String> {
    skip_ws(chars);

    if chars[0] == '+' {
        advance(chars, 1);
        unary(chars)
    } else if chars[0] == '-' {
        advance(chars, 1);
        Ok(-(unary(chars)?))
    } else {
        power(chars)
    }
}

fn multiplication(chars: &mut Vec<char>) -> Result<i32, String> {
    let mut left = unary(chars)?;

    while "*/".contains(chars[0]) {
        let op = chars[0];
        advance(chars, 1);

        let right = unary(chars)?;

        left = if op == '*' { left * right } else { left / right };
    }

    Ok(left)
}

fn addition(chars: &mut Vec<char>) -> Result<i32, String> {
    let mut left = multiplication(chars)?;

    while "+-".contains(chars[0]) {
        let op = chars[0];
        advance(chars, 1);

        let right = multiplication(chars)?;

        left = if op == '+' { left + right } else { left - right };
    }

    Ok(left)
}

fn expr(chars: &mut Vec<char>) -> Result<i32, String> {
    addition(chars)
}

fn parse(s: &String) -> Result<i32, String> {
    let mut chars : Vec<char> = s.chars().collect();
    chars.push('\0');

    let value = addition(&mut chars)?;

    if chars[0] != '\0' {
        return Err(format!("Failed to parse '{}", chars.into_iter().collect::<String>()));
    }

    Ok(value)
}

fn main() {
    loop {
        // Print a prompt.
        print!("> ");
        io::stdout().flush().unwrap();

        // Read a line from stdin.
        let mut line : String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line from stdin.");

        if line.is_empty() {
            break;
        } else if line == "\n" {
            continue;
        }

        let value = parse(&line);

        match value {
            Ok(val) => println!("{}", val),
            Err(err) => println!("Error: {}", err),
        }
    }
}
