use std::io::Read;
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

#[derive(Debug)]
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

    pub fn read_key(&self, timeout_ms: i32) -> Option<Key> {
        let mut poll_fd = libc::pollfd {
            fd: libc::STDIN_FILENO,
            events: libc::POLLIN,
            revents: 0,
        };

        let ready = unsafe { libc::poll(&mut poll_fd, 1, timeout_ms) };
        if ready <= 0 {
            return None;
        }

        let mut buf = [0u8; 4];
        let bytes_read = std::io::stdin().read(&mut buf).unwrap_or(0);

        if bytes_read == 1 {
            match buf[0] {
                b'q' | b'Q' => Some(Key::Quit),
                b'\t' => Some(Key::Tab),
                b => Some(Key::Char(b as char)),
            }
        } else if bytes_read >= 3 && buf[0] == 0x1b && buf[1] == b'[' {
            match buf[2] {
                b'A' => Some(Key::Up),
                b'B' => Some(Key::Down),
                _ => Some(Key::Other),
            }
        } else {
            Some(Key::Other)
        }
    }
}

impl Drop for RawTermit {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.org_state);
            print!("\x1b[?25h");
            print!("\x1b[?1049l");
            std::io::stdout().flush().ok();
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Tab,
    Up,
    Down,
    Quit,
    Other,
}
