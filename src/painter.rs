use std::{
    fmt::Display,
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    style::{Attribute, Print, SetAttribute},
    terminal::{
        Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};

use crate::constant::symbols;

// Painter module. All drawing/painting to the terminal screen should be done from here.
pub struct Painter {
    stdout: io::Stdout,
}

impl Painter {
    pub fn new(stdout: io::Stdout) -> Painter {
        Painter { stdout }
    }

    pub fn start(&mut self) -> crossterm::Result<()> {
        execute!(
            self.stdout,
            EnterAlternateScreen,
            cursor::Hide,
            DisableLineWrap,
            EnableMouseCapture
        )
    }

    pub fn stop(&mut self) -> crossterm::Result<()> {
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

    // Prints content to a specific position.
    pub fn write<D: Display>(&mut self, x: u16, y: u16, content: D) -> crossterm::Result<()> {
        queue!(self.stdout, cursor::MoveTo(x, y), Print(content))
    }

    // Fill area starting from position (x, y) with (width, height) size with {fill} characters.
    pub fn _fill<D: Display>(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        fill: D,
    ) -> crossterm::Result<()> {
        for i in 0..height {
            self.stdout.queue(cursor::MoveTo(x, y + i))?;

            for _ in 0..width {
                self.stdout.queue(Print(&fill))?;
            }
        }

        Ok(())
    }

    pub fn draw_box(&mut self, x: u16, y: u16, width: u16, height: u16) -> crossterm::Result<()> {
        queue!(
            self.stdout,
            cursor::MoveTo(x, y),
            Print(symbols::TOP_LEFT),
            Print(symbols::HORIZONTAL.repeat(usize::from(width - 2))),
            Print(symbols::TOP_RIGHT)
        )?;

        for i in 1..(height - 1) {
            queue!(
                self.stdout,
                cursor::MoveTo(x, y + i),
                Print(symbols::VERTICAL),
                cursor::MoveToColumn(x + (width - 1)),
                Print(symbols::VERTICAL)
            )?;
        }

        queue!(
            self.stdout,
            cursor::MoveTo(x, y + (height - 1)),
            Print(symbols::BOTTOM_LEFT),
            Print(symbols::HORIZONTAL.repeat(usize::from(width - 2))),
            Print(symbols::BOTTOM_RIGHT)
        )?;

        Ok(())
    }

    pub fn set_attribute(&mut self, attribute: Attribute) -> crossterm::Result<()> {
        self.stdout.queue(SetAttribute(attribute))?;

        Ok(())
    }

    pub fn clear(&mut self) -> crossterm::Result<()> {
        self.stdout.queue(Clear(ClearType::All))?;

        Ok(())
    }

    pub fn flush(&mut self) -> crossterm::Result<()> {
        self.stdout.flush()
    }
}
