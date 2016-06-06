extern crate alfred;
extern crate regex;

use std::env;
use std::io;
use std::io::prelude::*;
use std::process::exit;
use regex::Regex;

fn main() {
    let arg = env::args().skip(1).next();
    match process_argument(arg.as_ref().map_or("", |s| &s[..])) {
        Err(err) => {
            let _ = writeln!(&mut io::stderr(), "I/O error: {}", err);
            exit(1);
        }
        _ => {}
    }
}

fn process_argument(arg: &str) -> io::Result<()> {
    let mut xmlw = try!(alfred::XMLWriter::new(io::stdout()));
    if let Some(color_str) = extract_color(arg) {
        let item = alfred::ItemBuilder::new(&color_str[..])
                                       .subtitle("Storyboard color value")
                                       .arg(&color_str[..])
                                       .valid(true)
                                       .into_item();
        try!(xmlw.write_item(&item));
    } else {
        let item = alfred::ItemBuilder::new("Invalid input")
                                       .subtitle("Storyboard color value")
                                       .valid(false)
                                       .into_item();
        try!(xmlw.write_item(&item));
    }
    let mut stdout = try!(xmlw.close());
    stdout.flush()
}

fn extract_color(arg: &str) -> Option<String> {
    let mut result = "#".to_owned();
    let re = Regex::new(r"\d+(?:\.\d+)?").unwrap();
    for pos in re.find_iter(arg).take(3) {
        match arg[pos.0..pos.1].parse::<f64>() {
            Ok(value) => {
                let hex_value = (value * 255.0).round() as i32;
                result.push_str(&format!("{:02X}", hex_value));
            }
            Err(_) => return None
        }
    }
    if result == "#" {
        return None
    } else {
        return Some(result)
    }
}
