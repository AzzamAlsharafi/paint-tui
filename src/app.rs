mod area;
mod canvas;
mod panel;

use std::io;

use crossterm::{
    event::{read, Event, KeyCode, MouseButton, MouseEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, size},
};

use crate::painter::Painter;

use self::{
    area::{Area, Corner, Point},
    canvas::Canvas,
    panel::RightPanel,
};

pub struct App {
    painter: Painter,
    right_panel: RightPanel,
    canvas: Canvas,
}

impl App {
    pub fn new(stdout: io::Stdout) -> App {
        App {
            painter: Painter::new(stdout),
            right_panel: RightPanel::new(Area::new(
                Point::new(0, 0, Corner::TopLeft),
                Point::new(4, 0, Corner::BottomLeft),
            )),
            canvas: Canvas::new(
                Area::new(
                    Point::new(6, 0, Corner::TopLeft),
                    Point::new(0, 0, Corner::_BottomRight),
                ),
                50,
                20,
            ),
        }
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        enable_raw_mode()?;

        self.painter.start()?;

        self.draw_all(size()?)?;

        loop {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        return self.exit();
                    }
                }
                Event::Resize(width, height) => {
                    self.painter.clear()?;
                    self.draw_all((width, height))?;
                }
                Event::Mouse(event) => match event.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        self.handle_left_click(event.column, event.row)?
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        self.handle_drag(event.column, event.row)?
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn draw_all(&mut self, t_size: (u16, u16)) -> crossterm::Result<()> {
        self.right_panel.draw(&mut self.painter, t_size)?;
        self.canvas.draw(&mut self.painter, t_size)?;

        self.painter.flush()?;

        Ok(())
    }

    fn handle_left_click(&mut self, x: u16, y: u16) -> crossterm::Result<()> {
        let t_size = size()?;

        if self.right_panel.area.check_inside(x, y, t_size) {
            self.right_panel.click(&mut self.painter, x, y, t_size)?;
        } else if self.canvas.area.check_inside(x, y, t_size) {
            self.canvas.click(
                &mut self.painter,
                self.right_panel.get_tool(),
                &self.right_panel.brush,
                x,
                y,
            )?;
        }

        Ok(())
    }

    fn handle_drag(&mut self, x: u16, y: u16) -> crossterm::Result<()> {
        let t_size = size()?;

        if self.canvas.area.check_inside(x, y, t_size) {
            self.canvas.drag(
                &mut self.painter,
                self.right_panel.get_tool(),
                &self.right_panel.brush,
                x,
                y,
            )?;
        }

        Ok(())
    }

    fn exit(&mut self) -> crossterm::Result<()> {
        self.painter.stop()?;

        disable_raw_mode()?;

        Ok(())
    }
}
