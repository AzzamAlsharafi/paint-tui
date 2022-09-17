use std::{io, time::Duration};

use crossterm::{
    cursor,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

pub struct App {
    stdout: io::Stdout,
}

impl App {
    pub fn new(stdout: io::Stdout) -> App {
        App { stdout }
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
                }
                _ => {}
            }
        }
    }

    fn exit(&mut self) -> crossterm::Result<()> {
        self.stdout.execute(DisableMouseCapture)?;

        // Exhauste all events before closing.
        // If this is not done there will be some text
        // written in the terminal when the app closes.
        // I assume this text is because there are some queued events.
        loop {
            if poll(Duration::from_millis(100))? {
                read()?;
            } else {
                break;
            }
        }

        execute!(
            self.stdout,
            EnableLineWrap,
            cursor::Show,
            LeaveAlternateScreen
        )?;

        disable_raw_mode()?;

        Ok(())
    }
}
