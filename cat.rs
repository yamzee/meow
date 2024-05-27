use std::env::args;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::process::exit;

// define all the escape codes, _ for no unused warn
const _EC_RED: &str = "\x1b[31m";
const _EC_BLK: &str = "\x1b[30m";
const _EC_BG_RED: &str = "\x1b[41m";
const _EC_BOLD: &str = "\x1b[1m";
const _EC_UL: &str = "\x1b[4m";
const _EC_RESET: &str = "\x1b[0m";

// pull first bit high to set background instead
fn rgb_it(bg: bool, r: u8, g: u8, b: u8) -> String {
    let mut fg = "3";

    if bg {
        fg = "4";
    }

    return "\x1b[".to_owned()
        + fg
        + "8;2;"
        + &r.to_string()
        + ";"
        + &g.to_string()
        + ";"
        + &b.to_string()
        + "m";
}

fn main() {
    let _ec_err = rgb_it(true, 0xAD, 0xAB, 0x0D);
    let args: Vec<String> = args().skip(1).collect();
    let mut fails: Vec<(&str, String, String)> = vec![];

    if args.is_empty() {
        eprintln!("Need at least one arg!");
        exit(1);
    }

    for i in args.into_iter() {
        let file_result = File::open(&i);
        let mut cont = String::new();

        let mut file = match file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    let tfail = ("open", i, "file not found".to_string());
                    fails.push(tfail);
                    continue;
                }
                ErrorKind::PermissionDenied => {
                    let tfail = ("open", i, "permission denied".to_string());
                    fails.push(tfail);
                    continue;
                }
                _ => {
                    let tfail = ("open", i, error.to_string());
                    fails.push(tfail);
                    //continue;
                    panic!("{:?}", error);
                }
            },
        };

        let result = match file.read_to_string(&mut cont) {
            Ok(_) => cont,
            Err(error) => match error.kind() {
                ErrorKind::InvalidData => {
                    let tfail = ("read", i, "invalid UTF-8 or binary".to_string());
                    fails.push(tfail);
                    continue;
                }
                _ => {
                    let tfail = ("read", i, error.to_string());
                    fails.push(tfail);
                    //continue;
                    panic!("{:?}", error);
                }
            },
        };
        //file.read_to_string(&mut cont).unwrap();
        print!("{result}");
    }

    if !fails.is_empty() {
        for i in fails.into_iter() {
            eprintln!(
                "{_ec_err}{_EC_BLK}{_EC_BOLD}ERROR{_EC_RESET} \
                - Failed to {} {}: {}",
                i.0, i.1, i.2
            );
        }
        exit(1);
    }
}
