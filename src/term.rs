//! Terminal inquiry.

use crate::Result;
use libc::{c_int, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::io::{self, Error, IsTerminal};
use std::mem::MaybeUninit;

pub fn size() -> Result<(u16, u16)> {
    if io::stdin().is_terminal() {
        let win_size = unsafe {
            let mut win_size = MaybeUninit::<winsize>::uninit();
            os_result(libc::ioctl(STDOUT_FILENO, TIOCGWINSZ, win_size.as_mut_ptr()))?;
            win_size.assume_init()
        };
        Ok((win_size.ws_row, win_size.ws_col))
    } else {
        Err("not a terminal".to_string())
    }
}

fn os_result(err: c_int) -> Result<()> {
    if err < 0 {
        let e = Error::last_os_error();
        Err(format!("OS error: {e}"))
    } else {
        Ok(())
    }
}
