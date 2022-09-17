use std::io;

use crossterm::{
    cursor,
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode
        
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    }
};

pub struct App {
    stdout: io::Stdout,
}

impl App {
    pub fn new(stdout: io::Stdout) -> App {
        App {
            stdout,
        }
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        enable_raw_mode()?;

        execute!(
            self.stdout,
            EnterAlternateScreen,
            cursor::Hide,
            DisableLineWrap,
            EnableMouseCapture
        )?;

        loop {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        return self.exit();
                    }
                },
                _ => {}
            }
        }
    }

    fn exit(&mut self) -> crossterm::Result<()> {
        execute!(
            self.stdout,
            DisableMouseCapture,
            EnableLineWrap,
            cursor::Show,
            LeaveAlternateScreen
        )?;

        disable_raw_mode()?;

        Ok(())
    }
}
