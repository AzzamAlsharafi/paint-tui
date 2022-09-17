mod area;

use std::io;

use crossterm::{
    event::{read, Event, KeyCode, MouseButton, MouseEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, size}, style::Stylize,
};

use crate::painter::Painter;

use self::{area::{Area, Corner, Point}};

pub struct App {
    painter: Painter,
}

impl App {
    pub fn new(stdout: io::Stdout) -> App {
        App {
            painter: Painter::new(stdout),
        }
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        enable_raw_mode()?;

        self.painter.start()?;

        self.draw()?;

        loop {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        return self.exit();
                    }
                }
                Event::Resize(_, _) => {
                    self.painter.clear()?;
                    self.draw()?;
                }
                Event::Mouse(event) => {
                    if let MouseEventKind::Down(MouseButton::Left) = event.kind {
                        // Left mouse click event   
                    }
                }
                _ => {}
            }
        }
    }

    fn draw(&mut self) -> crossterm::Result<()> {
        let t_size = size()?;
        
        self.painter.flush()?;

        Ok(())
    }

    fn exit(&mut self) -> crossterm::Result<()> {
        self.painter.stop()?;

        disable_raw_mode()?;

        Ok(())
    }
}
