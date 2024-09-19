mod ansi;
mod bounce;
mod opt;
mod term;

use crate::bounce::Bounce;
use crate::opt::Options;
use crate::term::term_size;
use std::env;
use std::io::{self, IsTerminal};
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

pub type Result<T> = std::result::Result<T, String>;

const USAGE: &str = r#"
usage: bounce OPTIONS
       bounce --help

  optional:
    --delay MILLIS      : delay in milliseconds before ball advances
                          (default: 50)
    --lines COUNT       : number of horizontal and vertical barriers
                          (default: 50)
    --ball-color COLOR  : color of ball (default: blue)
    --line-color COLOR  : color of lines (default: red)

  colors:
    red, green, yellow, blue, magenta, cyan, white
"#;

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            println!("{e}");
            println!("use --help for options");
            ExitCode::from(1)
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}

fn run() -> Result<()> {
    let opts = Options::parse(env::args().skip(1))?;
    if opts.help {
        println!("{USAGE}");
    } else {
        let (rows, cols) = detect_size()?;
        let mut bounce = Bounce::new(rows, cols, opts.lines, opts.ball_color, opts.line_color);
        let delay = Duration::from_millis(opts.delay);
        while bounce.more() {
            bounce.next();
            thread::sleep(delay);
        }
    }
    Ok(())
}

fn detect_size() -> Result<(u16, u16)> {
    if io::stdin().is_terminal() {
        Ok(term_size()?)
    } else {
        Err("not a terminal".to_string())
    }
}
