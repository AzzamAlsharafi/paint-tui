use std::{io::{self, Write}, time::Duration};

use crossterm::{cursor, style::{StyledContent, Print}, QueueableCommand, execute, terminal::{EnterAlternateScreen, DisableLineWrap, EnableLineWrap, LeaveAlternateScreen, Clear, ClearType}, event::{EnableMouseCapture, DisableMouseCapture, poll, read}, ExecutableCommand, queue};

// Painter module. All drawing/painting to the terminal screen should be done from here.
pub struct Painter {
    stdout: io::Stdout,
}

impl Painter {
    pub fn new(stdout: io::Stdout) -> Painter {
        Painter { stdout }
    }

    pub fn start(&mut self) -> crossterm::Result<()>{
        execute!(
            self.stdout,
            EnterAlternateScreen,
            cursor::Hide,
            DisableLineWrap,
            EnableMouseCapture
        )
    }

    pub fn stop(&mut self) -> crossterm::Result<()>{
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

        Ok(())
    }

    // Prints one char to a specific position.
    pub fn write(&mut self, x: u16, y: u16, content: StyledContent<char>) -> crossterm::Result<()> {
        queue!(self.stdout, cursor::MoveTo(x, y), Print(content))
    }

    // Fill area starting from position (x, y) with (width, height) size with {fill} characters.
    pub fn fill(&mut self, x: u16, y: u16, width: u16, height: u16, fill: StyledContent<char>) -> crossterm::Result<()> {
        for i in 0..height {
            self.stdout.queue(cursor::MoveTo(x, y + i))?;

            for _ in 0..width {
                self.stdout.queue(Print(fill))?;
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) -> crossterm::Result<()>{
        self.stdout.queue(Clear(ClearType::All))?;

        Ok(())
    }

    pub fn flush(&mut self) -> crossterm::Result<()>{
        self.stdout.flush()
    }
}