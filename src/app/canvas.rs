use std::vec;

use crossterm::style::{ContentStyle, StyledContent};

use crate::{constant::symbols, painter::Painter, utils::DiffOrZero};

use super::{area::Area, panel::Tool};

pub struct Canvas {
    pub area: Area,
    content: Vec<Vec<StyledContent<char>>>,
}

impl Canvas {
    // {content_width} and {content_height} are the size of the drawing area,
    // which is different to {area}, the size of the Canvas widget/component.
    pub fn new(area: Area, content_width: usize, content_height: usize) -> Canvas {
        let default_content = StyledContent::new(ContentStyle::new(), ' ');

        let content = vec![vec![default_content; content_width]; content_height];

        Canvas { area, content }
    }

    pub fn draw(&self, painter: &mut Painter, t_size: (u16, u16)) -> crossterm::Result<()> {
        let (x, y) = self.area.start.absolute_position(t_size);
        let (width, height) = self.area.size(t_size);

        // Offset to center the drawing area in case content is smaller than available space.
        let x_offset = width.diff_or_zero(&((self.content[0].len() + 2) as u16)) / 2;
        let y_offset = height.diff_or_zero(&((self.content.len() + 2) as u16)) / 2;

        painter.write_canvas_content(
            x + x_offset,
            y + y_offset,
            width,
            height,
            &self.content_with_border(),
        )
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
        let (width, height) = self.area.size(t_size);

        let x_offset = width.diff_or_zero(&(self.content[0].len() as u16)) / 2;
        let y_offset = height.diff_or_zero(&(self.content.len() as u16)) / 2;

        if (cx >= x + x_offset && cx < x + x_offset + self.content[0].len() as u16)
            && (cy >= y + y_offset && cy < y + y_offset + self.content.len() as u16)
        {
            let content_x = usize::from(cx.abs_diff(x + x_offset));
            let content_y = usize::from(cy.abs_diff(y + y_offset));

            match tool {
                Tool::Select => {}
                Tool::Move => {}
                Tool::Rectangle => {}
                Tool::Circle => {}
                Tool::Brush => {
                    self.content[content_y][content_x] = *brush;

                    painter.write(cx, cy, brush)?;
                    painter.flush()?;
                }
                Tool::Erase => {}
                Tool::Bucket => {}
                Tool::ColorPicker => {}
                Tool::Text => {}
            }
        }

        Ok(())
    }

    // Trash code, redo later.
    pub fn content_with_border(&self) -> Vec<Vec<StyledContent<char>>> {
        let mut content_with_border = vec![];

        let style = ContentStyle::new();

        let mut top = vec![StyledContent::new(style, symbols::HORIZONTAL); self.content[0].len()];

        top.insert(0, StyledContent::new(style, symbols::TOP_LEFT));
        top.push(StyledContent::new(style, symbols::TOP_RIGHT));

        let mut bottom =
            vec![StyledContent::new(style, symbols::HORIZONTAL); self.content[0].len()];

        bottom.insert(0, StyledContent::new(style, symbols::BOTTOM_LEFT));
        bottom.push(StyledContent::new(style, symbols::BOTTOM_RIGHT));

        content_with_border.push(top);

        for line in &self.content {
            let mut new_line: Vec<StyledContent<char>> =
                vec![StyledContent::new(style, symbols::VERTICAL)];

            for c in line {
                new_line.push(*c);
            }

            new_line.push(StyledContent::new(style, symbols::VERTICAL));

            content_with_border.push(new_line);
        }

        content_with_border.push(bottom);

        content_with_border
    }
}
