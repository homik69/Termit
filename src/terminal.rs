use std::io::Write;

pub fn get_termit_size() -> (usize, usize) {
    unsafe {
        let mut winsize: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut winsize) == 0
            && winsize.ws_col > 0
        {
            (winsize.ws_col as usize, winsize.ws_row as usize)
        } else {
            (80, 24)
        }
    }
}

pub struct RawTermit {
    org_state: libc::termios,
}

impl RawTermit {
    pub fn new() -> std::io::Result<Self> {
        unsafe {
            let mut termit: libc::termios = std::mem::zeroed();

            let get_attr = libc::tcgetattr(libc::STDIN_FILENO, &mut termit);
            if get_attr == -1 {
                return Err(std::io::Error::last_os_error());
            }

            let mut raw = termit;

            raw.c_lflag &= !(libc::ICANON | libc::ECHO);

            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &raw);

            print!("\x1b[?1049h");
            print!("\x1b[?25l");
            std::io::stdout().flush().expect("prank");

            return Ok(RawTermit { org_state: termit });
        }
    }
}

impl Drop for RawTermit {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.org_state);
            print!("\x1b[?25h");
            print!("\x1b[?1049l");
            std::io::stdout().flush().expect("prank");
        }
    }
}
