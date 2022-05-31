enum Value {
    Int(i32),
    Float(f64),
    Text(String),
}

mod value_parser;
mod errors;
use errors::ErrorReport;
use value_parser::*;

fn trim(code: &mut String) {
    *code = code.to_owned()[1..].to_string();
}

fn assignement(code: &mut String) -> Result<(), ErrorReport> {
    trim(code);
    let id = parse_int(code)?;
    println!("{}", id);
    return Ok(());
}

fn execute(code: &mut String) -> Result<(), ErrorReport> {
    loop {
        if code.len() == 0 {
            return Ok(());
        }

        match &code.to_owned()[0..1] {
            " " => trim(code),
            "=" => assignement(code)?,
            _ => ()
        }
    }
}

fn main() {
    let code = &mut String::from("= 1852");
    let result = execute(code);
    println!("{:?}", result);
}
