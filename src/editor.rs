use std::io::{self, stdout, Write};
use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("Error: {e}");
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit == true {
                println!("Goodbye.\r");
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("\x1b[2J \x1b[1;1H");
        // print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        io::stdout().flush()
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}
