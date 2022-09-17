use crossterm::style::{Color, Stylize};

use crate::painter::Painter;

use super::area::Area;

// A solid plane with a color. This is used for testing Area and Point.
pub struct Solid {
    area: Area,
    color: Color,
}

impl Solid {
    pub fn new(area: Area, color: Color) -> Solid {
        Solid { area, color }
    }

    pub fn draw(&self, painter: &mut Painter, t_size: (u16, u16)) -> crossterm::Result<()> {
        let (x, y) = self.area.start.absolute_position(t_size.0, t_size.1);
        let (width, height) = self.area.size(t_size.0, t_size.1);

        painter.fill(x, y, width, height, '█'.with(self.color))?;

        Ok(())
    }
}
