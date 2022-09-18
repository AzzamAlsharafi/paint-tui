use std::vec;

use crossterm::style::{ContentStyle, StyledContent};

use crate::painter::Painter;

use super::{
    area::Area,
    panel::Tool,
};

pub struct Canvas {
    pub area: Area,
    content: Vec<Vec<StyledContent<char>>>,
}

impl Canvas {
    // {content_width} and {content_height} are the size of the drawing area,
    // which is different to {area}, the size of the Canvas widget/component.
    pub fn new(area: Area, content_width: usize, content_height: usize) -> Canvas {
        let default_content = StyledContent::new(ContentStyle::new(), 'A');

        let content = vec![vec![default_content; content_width]; content_height];

        Canvas {
            area,
            content,
        }
    }

    pub fn draw(&self, painter: &mut Painter, t_size: (u16, u16)) -> crossterm::Result<()> {
        let (x, y) = self.area.start.absolute_position(t_size);
        let (width, height) = self.area.size(t_size);

        painter.write_canvas_content(x, y, width, height, &self.content)
    }

    // cx: click x, cy: click y
    pub fn click(
        &mut self,
        painter: &mut Painter,
        tool: &Tool,
        brush: &StyledContent<char>,
        cx: u16,
        cy: u16,
        t_size: (u16, u16),
    ) -> crossterm::Result<()> {
        let (x, y) = self.area.start.absolute_position(t_size);

        let content_x = usize::from(cx.abs_diff(x));
        let content_y = usize::from(cy.abs_diff(y));

        if content_x < self.content[0].len() && content_y < self.content.len() {
            match tool {
                Tool::Select => {},
                Tool::Move => {},
                Tool::Rectangle => {},
                Tool::Circle => {},
                Tool::Brush => {
                    self.content[content_y][content_x] = *brush;

                    painter.write(cx, cy, brush)?;
                    painter.flush()?;
                }
                Tool::Erase => {},
                Tool::Bucket => {},
                Tool::ColorPicker => {},
                Tool::Text => {},
            }
        }

        Ok(())
    }
}
