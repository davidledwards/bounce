//! Options parser.

use crate::Result;
use crate::ansi;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;

static COLORS: LazyLock<HashMap<&str, u8>> = LazyLock::new(|| {
    HashMap::from([
        ("red", ansi::RED),
        ("green", ansi::GREEN),
        ("yellow", ansi::YELLOW),
        ("blue", ansi::BLUE),
        ("magenta", ansi::MAGENTA),
        ("cyan", ansi::CYAN),
        ("white", ansi::WHITE),
        ("gray", ansi::GRAY),
    ])
});

pub struct Options {
    pub help: bool,
    pub lines: u32,
    pub delay: u64,
    pub ball_char: char,
    pub ball_color: u8,
    pub line_color: u8,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            help: false,
            lines: 50,
            delay: 50,
            ball_char: '●',
            ball_color: ansi::RED,
            line_color: ansi::GRAY,
        }
    }
}

impl Options {
    pub fn parse<T>(args: T) -> Result<Options>
    where
        T: IntoIterator<Item = String>,
    {
        let mut opts = Options::default();
        let mut it = args.into_iter();
        while let Some(arg) = it.next() {
            match arg.as_str() {
                "--help" => opts.help = true,
                "--lines" => opts.lines = parse_number(&arg, it.next())?,
                "--delay" => opts.delay = parse_number(&arg, it.next())?,
                "--ball" => opts.ball_char = parse_ball(&arg, it.next())?,
                "--ball-color" => opts.ball_color = parse_color(&arg, it.next())?,
                "--line-color" => opts.line_color = parse_color(&arg, it.next())?,
                _ => return Err(format!("{arg}: unexpected argument")),
            };
        }
        Ok(opts)
    }
}

fn parse_number<T>(arg: &str, next_arg: Option<String>) -> Result<T>
where
    T: FromStr,
{
    match next_arg {
        Some(a) => match a.parse::<T>() {
            Ok(delay) => Ok(delay),
            Err(_) => Err(format!("{a}: invalid argument following {arg}")),
        }
        None => Err(format!("{arg}: expecting number to follow")),
    }
}

fn parse_ball(arg: &str, next_arg: Option<String>) -> Result<char> {
    match next_arg {
        Some(a) => match a.chars().next() {
            Some(c) => Ok(c),
            None => Err(format!("{arg}: expecting character")),
        }
        None => Err(format!("{arg}: expecting character"))
    }
}

fn parse_color(arg: &str, next_arg: Option<String>) -> Result<u8> {
    match next_arg {
        Some(a) => match COLORS.get(a.to_lowercase().as_str()) {
            Some(color) => Ok(*color),
            None => Err(format!("{a}: unrecognized color")),
        }
        None => Err(format!("{arg}: expecting color"))
    }
}
