use std::env::args;
use std::fs::File;
use std::io::{stdin, ErrorKind, Read};
//use std::io::{stdout, Write};
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
    let mut buffer = String::new();
    let stdin = stdin();
    // let mut buffer = Vec::new();
    // let mut stdout = stdout();
    // let mut handle = stdin.lock();

    if args.is_empty() {
        // this doesn't exactly do what `cat` normally does.
        // we need to read and repeat each line until user gives EOF
        // so far we just take everything and put it in the buffer before
        // printing it out at the end, if it's valid

        // handle.read_to_end(&mut buffer).unwrap();
        // stdout.write_all(&buffer);
        loop {
            match stdin.read_line(&mut buffer) {
                Ok(0) => {
                    exit(0);
                }
                Ok(_) => Some(Ok(&buffer)),
                Err(error) => Some(Err(error)),
            };
            print!("{buffer}");
            buffer.clear();
        }
    }

    for i in args.into_iter() {
        let file_result = File::open(&i);
        let mut cont = String::new();

        let file = match file_result {
            Ok(mut file) => match file.read_to_string(&mut cont) {
                Ok(_) => cont,
                Err(error) => match error.kind() {
                    ErrorKind::InvalidData => {
                        fails.push(("read", i, "invalid UTF-8 or binary".to_string()));
                        continue;
                    }
                    _ => {
                        fails.push(("read", i, error.to_string()));
                        //continue;
                        panic!("{:?}", error);
                    }
                },
            },
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    fails.push(("open", i, "file not found".to_string()));
                    continue;
                }
                ErrorKind::PermissionDenied => {
                    fails.push(("open", i, "permission denied".to_string()));
                    continue;
                }
                _ => {
                    fails.push(("open", i, error.to_string()));
                    //continue;
                    panic!("{:?}", error);
                }
            },
        };
        print!("{file}");
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
