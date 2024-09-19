use std::str::FromStr;

type Result<T> = std::result::Result<T, String>;

pub struct Options {
    pub help: bool,
    pub lines: u32,
    pub delay: u64,
    pub ball_color: u8,
    pub line_color: u8,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            help: false,
            lines: 50,
            delay: 50,
            ball_color: 34,
            line_color: 31,
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

fn parse_color(arg: &str, next_arg: Option<String>) -> Result<u8> {
    match next_arg {
        Some(a) => match a.to_lowercase().as_str() {
            "red" => Ok(31),
            "green" => Ok(32),
            "yellow" => Ok(33),
            "blue" => Ok(34),
            "magenta" => Ok(35),
            "cyan" => Ok(36),
            "white" => Ok(37),
            _ => Err(format!("{a}: unrecognized color")),
        }
        None => Err(format!("{arg}: expecting color"))
    }
}
