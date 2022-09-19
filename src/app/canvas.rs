use std::vec;

use crossterm::style::{ContentStyle, StyledContent};

use crate::{constant::symbols, painter::Painter, utils::{DiffOrZero, AddSubOrZero}};

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

    // Returns whether (cx, cy) is inside drawing area, and content_x, content_y.
    // Used in click() and drag(), hence the name.
    fn mouse_event_common(&self, cx: u16, cy: u16, t_size: (u16, u16)) -> (bool, usize, usize) {
        let (x, y) = self.area.start.absolute_position(t_size);
        let (width, height) = self.area.size(t_size);

        let x_offset = width.abs_diff(self.content[0].len() as u16) / 2;
        let y_offset = height.abs_diff(self.content.len() as u16) / 2;

        let check_inside = (cx >= x + x_offset && cx < x + x_offset + self.content[0].len() as u16)
            && (cy >= y + y_offset && cy < y + y_offset + self.content.len() as u16);

        let content_x = if width > self.content[0].len() as u16 {
            usize::from(cx.abs_diff(x + x_offset))
        } else {
            usize::from(cx.abs_diff(x.diff_or_zero(&x_offset)))
        };
        
        let content_y = if height > self.content.len() as u16{
            usize::from(cy.abs_diff(y + y_offset))
        } else {
            usize::from(cy.abs_diff(y.diff_or_zero(&y_offset)))
        };

        (check_inside, content_x, content_y)
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
        let (check_inside, content_x, content_y) = self.mouse_event_common(cx, cy, t_size);

        if check_inside {
            match tool {
                Tool::Select => {}
                Tool::Move => {}
                Tool::Rectangle => {}
                Tool::Circle => {}
                Tool::Brush => self.brush(painter, cx, cy, content_x, content_y, brush)?,
                Tool::Erase => self.brush(painter, cx, cy, content_x, content_y, &Self::ersr())?,
                Tool::Bucket => {
                    self.bucket(painter, cx, cy, t_size, content_x, content_y, brush)?
                }
                Tool::ColorPicker => {}
                Tool::Text => {}
            }
        }

        Ok(())
    }

    pub fn drag(
        &mut self,
        painter: &mut Painter,
        tool: &Tool,
        brush: &StyledContent<char>,
        cx: u16,
        cy: u16,
        t_size: (u16, u16),
    ) -> crossterm::Result<()> {
        let (check_inside, content_x, content_y) = self.mouse_event_common(cx, cy, t_size);

        if check_inside {
            match tool {
                Tool::Select => {}
                Tool::Move => {}
                Tool::Rectangle => {}
                Tool::Circle => {}
                Tool::Brush => self.brush(painter, cx, cy, content_x, content_y, brush)?,
                Tool::Erase => self.brush(painter, cx, cy, content_x, content_y, &Self::ersr())?,
                Tool::Bucket => {}
                Tool::ColorPicker => {}
                Tool::Text => {}
            }
        }

        Ok(())
    }

    fn brush(
        &mut self,
        painter: &mut Painter,
        cx: u16,
        cy: u16,
        content_x: usize,
        content_y: usize,
        brush: &StyledContent<char>,
    ) -> crossterm::Result<()> {
        self.content[content_y][content_x] = *brush;

        painter.write(cx, cy, brush)?;
        painter.flush()?;

        Ok(())
    }

    // Eraser.
    fn ersr() -> StyledContent<char> {
        StyledContent::new(ContentStyle::default(), ' ')
    }

    fn bucket(
        &mut self,
        painter: &mut Painter,
        cx: u16,
        cy: u16,
        t_size: (u16, u16),
        content_x: usize,
        content_y: usize,
        brush: &StyledContent<char>,
    ) -> crossterm::Result<()> {
        let selected = self.content[content_y][content_x];

        let mut done: Vec<(usize, usize)> = vec![];

        self.apply_bucket(
            painter, cx, cy, t_size, content_x, content_y, brush, &selected, &mut done
        )?;

        self.draw(painter, t_size)?;
        painter.flush()?;

        Ok(())
    }

    fn apply_bucket(
        &mut self,
        painter: &mut Painter,
        cx: u16,
        cy: u16,
        t_size: (u16, u16),
        content_x: usize,
        content_y: usize,
        brush: &StyledContent<char>,
        selected: &StyledContent<char>,
        done: &mut Vec<(usize, usize)>
    ) -> crossterm::Result<()> {
        // Check that we're still inside canvas, if not return.
        if !(content_x < self.content[0].len() && content_y < self.content.len()) {
            return Ok(());
        }

        // Check if this position has been done before, if yes return.
        if done.contains(&(content_x, content_y)){
            return Ok(());
        }

        // Mark position as done.
        done.push((content_x, content_y));

        let current = self.content[content_y][content_x];

        if &current != selected {
            return Ok(());
        }

        self.content[content_y][content_x] = *brush;

        let adjacent: [(i16, i16); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Spread.
        for add in adjacent {
            self.apply_bucket(
                painter,
                cx.add_sub_or_zero(&add.0),
                cy.add_sub_or_zero(&add.1),
                t_size,
                content_x.add_sub_or_zero(&add.0),
                content_y.add_sub_or_zero(&add.1),
                brush,
                selected,
                done
            )?;
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
