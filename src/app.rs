mod area;
mod canvas;
mod panel;

use std::io;

use crossterm::{
    event::{read, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind},
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
                    Point::new(6, 1, Corner::TopLeft),
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
                Event::Mouse(event) => {
                    self.handle_mouse_event(event)?;
                }
                _ => {}
            }
        }
    }

    fn draw_all(&mut self, t_size: (u16, u16)) -> crossterm::Result<()> {
        self.right_panel.draw(&mut self.painter, t_size)?;
        self.canvas.draw(&mut self.painter, t_size)?;

        self.draw_borders(t_size)?;

        self.painter.flush()?;

        Ok(())
    }

    // Draw major borders.
    fn draw_borders(&mut self, t_size: (u16, u16)) -> crossterm::Result<()> {
        // Canvas border
        let (canvas_x, canvas_y) = self.canvas.area.start.absolute_position(t_size);
        let (canvas_width, canvas_height) = self.canvas.area.size(t_size);

        if canvas_x > 0 && canvas_y > 0 {
            self.painter.draw_box(
                canvas_x - 1,
                canvas_y - 1,
                canvas_width + 2,
                canvas_height + 2,
            )?;
        }

        Ok(())
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> crossterm::Result<()> {
        let t_size = size()?;

        let (x, y) = (event.column, event.row);

        // Canvas is the only component that handles release events,
        // and it needs to handle them regardless of the position.
        if let MouseEventKind::Up(MouseButton::Left) = event.kind {
            return self.canvas.release(
                &mut self.painter,
                self.right_panel.get_tool(),
                &self.right_panel.brush,
            );
        }

        if self.right_panel.area.check_inside(x, y, t_size) {
            self.right_panel.mouse_event(event, &mut self.painter)?;
        } else if self.canvas.area.check_inside(x, y, t_size) {
            self.canvas.mouse_event(
                event,
                &mut self.painter,
                self.right_panel.get_tool(),
                &self.right_panel.brush,
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
