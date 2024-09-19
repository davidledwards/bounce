use libc::{c_int, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::io::Error;
use std::mem::MaybeUninit;

type Result<T> = std::result::Result<T, String>;

pub fn term_size() -> Result<(u16, u16)> {
    let win_size = unsafe {
        let mut win_size = MaybeUninit::<winsize>::uninit();
        os_result(libc::ioctl(STDOUT_FILENO, TIOCGWINSZ, win_size.as_mut_ptr()))?;
        win_size.assume_init()
    };
    Ok((win_size.ws_row, win_size.ws_col))
}

fn os_result(err: c_int) -> Result<()> {
    if err < 0 {
        let e = Error::last_os_error();
        Err(format!("OS error: {e}"))
    } else {
        Ok(())
    }
}
