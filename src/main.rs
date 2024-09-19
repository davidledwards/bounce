mod ansi;
mod opt;
mod play;
mod term;

use crate::play::Player;
use crate::opt::Options;
use std::env;
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

// Simple error type used across project.
pub type Result<T> = std::result::Result<T, String>;

const USAGE: &str = r#"
usage: bounce OPTIONS
       bounce --help

  Bounces a ball around the terminal and destroys barriers. The program stops
  when all barriers are gone, though press ^C to exit prematurely.

  options:
    --delay MILLIS      : delay in milliseconds before ball advances
                          (default: 50)
    --lines COUNT       : number of horizontal and vertical barriers
                          (default: 50)
    --ball CHAR         : character to use for ball (default is '●')
    --ball-color COLOR  : color of ball (default: red)
    --line-color COLOR  : color of lines (default: gray)

  COLOR options:
    red, green, yellow, blue, magenta, cyan, white, gray
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
        let (rows, cols) = term::size()?;
        let mut player = Player::new(rows, cols, &opts);
        let delay = Duration::from_millis(opts.delay);
        while player.more() {
            player.next();
            thread::sleep(delay);
        }
    }
    Ok(())
}
