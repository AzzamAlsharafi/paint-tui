mod area;
mod solid;

use std::io;

use crossterm::{
    event::{read, Event, KeyCode, MouseButton, MouseEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, size}, style::Stylize,
};

use crate::painter::Painter;

use self::{solid::Solid, area::{Area, Corner, Point}};

pub struct App {
    painter: Painter,
    panels: Vec<Solid>,
}

impl App {
    pub fn new(stdout: io::Stdout) -> App {
        let panels = vec![
            Solid::new(Area::new(Point::new(1, 6, Corner::TopLeft), Point::new(5, 6, Corner::BottomLeft)), crossterm::style::Color::Blue),
            Solid::new(Area::new(Point::new(5, 6, Corner::TopRight), Point::new(1, 6, Corner::BottomRight)), crossterm::style::Color::Red),
            Solid::new(Area::new(Point::new(1, 1, Corner::TopLeft), Point::new(1, 5, Corner::TopRight)), crossterm::style::Color::Green),
            Solid::new(Area::new(Point::new(1, 5, Corner::BottomLeft), Point::new(1, 1, Corner::BottomRight)), crossterm::style::Color::Yellow),
            Solid::new(Area::new(Point::new(7, 7, Corner::TopLeft), Point::new(7, 7, Corner::BottomRight)), crossterm::style::Color::Magenta),
        ];

        App {
            painter: Painter::new(stdout),
            panels,
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
                        let point = Point::new(event.column, event.row, Corner::TopLeft);

                        let t_size = size()?;

                        let (x, y) = point.absolute_position(t_size.0, t_size.1);

                        self.painter.write(x, y, 'X'.blue())?;
                        self.painter.flush()?;
                    }
                }
                _ => {}
            }
        }
    }

    fn draw(&mut self) -> crossterm::Result<()> {
        let t_size = size()?;
        
        for panel in &self.panels {
            panel.draw(&mut self.painter, t_size)?;
        }
        
        self.painter.flush()?;

        Ok(())
    }

    fn exit(&mut self) -> crossterm::Result<()> {
        self.painter.stop()?;

        disable_raw_mode()?;

        Ok(())
    }
}
