use std::{cmp::min, vec};

use crossterm::{
    event::{MouseButton, MouseEvent, MouseEventKind},
    style::{ContentStyle, StyledContent},
};

use crate::{painter::Painter, utils::AddSubOrZero};

use super::{area::Area, panel::Tool};

pub struct Canvas {
    pub area: Area,
    content: Vec<Vec<StyledContent<char>>>,
    // Fields that depends on current terminal window size.
    relative: Relative,
}

impl Canvas {
    // {content_width} and {content_height} are the size of the drawing area,
    // which is different to {area}, the size of the Canvas widget/component.
    pub fn new(area: Area, content_width: usize, content_height: usize) -> Canvas {
        let content = vec![vec![Self::empty(); content_width]; content_height];

        Canvas {
            area,
            content,
            relative: Relative::zero(),
        }
    }

    // Empty character. Used for creating new Canvas and with Eraser tool.
    fn empty() -> StyledContent<char> {
        StyledContent::new(ContentStyle::default(), ' ')
    }

    pub fn draw(&mut self, painter: &mut Painter, t_size: (u16, u16)) -> crossterm::Result<()> {
        self.set_relative(t_size);

        self.draw_border(painter)?;

        self.draw_content(painter)
    }

    fn set_relative(&mut self, t_size: (u16, u16)) {
        let (x, y) = self.area.start.absolute_position(t_size);
        let (width, height) = self.area.size(t_size);
        let content_width = self.content[0].len();
        let content_height = self.content.len();

        let transform_x = (((content_width as i32) - (width as i32)) / 2) - (x as i32);
        let transform_y = (((content_height as i32) - (height as i32)) / 2) - (y as i32);

        let visible_content_width = min(content_width as u16, width);
        let visible_content_height = min(content_height as u16, height);

        let content_start_x = if (content_width as u16) < width {
            transform_x.unsigned_abs() as u16
        } else {
            x
        };

        let content_start_y = if (content_height as u16) < height {
            transform_y.unsigned_abs() as u16
        } else {
            y
        };

        self.relative = Relative::new(
            (transform_x, transform_y),
            (visible_content_width, visible_content_height),
            (content_start_x, content_start_y),
        );
    }

    // Transforms absolute position to content position.
    // Returns None if parameters can't be converted.
    // (If conversion result is less than 0, or bigger than content size).
    fn apply_transform(&self, x: u16, y: u16) -> Option<(usize, usize)> {
        let content_width = self.content[0].len();
        let content_height = self.content.len();
        let (transform_x, transform_y) = self.relative.transform;

        let result_x = i32::from(x) + transform_x;
        let result_y = i32::from(y) + transform_y;

        // If transformed position is negative or bigger than content size, return None.
        if result_x < 0
            || result_x >= content_width as i32
            || result_y < 0
            || result_y >= content_height as i32
        {
            return None;
        }

        Some((result_x as usize, result_y as usize))
    }

    fn draw_content(&self, painter: &mut Painter) -> crossterm::Result<()> {
        let (start_x, start_y) = self.relative.content_start;
        let (visible_width, visible_height) = self.relative.visible_content_size;

        if let Some((content_x, content_y)) = self.apply_transform(start_x, start_y) {
            for iy in 0..visible_height {
                // Write first character in each line.
                painter.write(
                    start_x,
                    start_y + iy,
                    self.content[content_y + usize::from(iy)][content_x],
                )?;

                // Start from 1 because first character is already written.
                for ix in 1..visible_width {
                    // Write without moving cursor, because cursor is already in place.
                    painter.write_in_place(
                        self.content[content_y + usize::from(iy)][content_x + usize::from(ix)],
                    )?;
                }
            }
        }

        Ok(())
    }

    fn draw_border(&self, painter: &mut Painter) -> crossterm::Result<()> {
        // TODO: redo this. It doesn't work well with small screen size,
        // and it breaks the rule that each componenet shouldn't interact with outside its area.

        let (start_x, start_y) = self.relative.content_start;
        let (visible_width, visible_height) = self.relative.visible_content_size;

        if start_x == 0 || start_y == 0 {
            return Ok(());
        }

        painter.draw_box(
            start_x - 1,
            start_y - 1,
            visible_width + 2,
            visible_height + 2,
        )
    }

    pub fn mouse_event(
        &mut self,
        event: MouseEvent,
        painter: &mut Painter,
        tool: &Tool,
        brush: &StyledContent<char>,
    ) -> crossterm::Result<()> {
        let (click_x, click_y) = (event.column, event.row);

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                self.click(painter, tool, brush, click_x, click_y)?;
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                self.drag(painter, tool, brush, click_x, click_y)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn click(
        &mut self,
        painter: &mut Painter,
        tool: &Tool,
        brush: &StyledContent<char>,
        click_x: u16,
        click_y: u16,
    ) -> crossterm::Result<()> {
        if let Some((content_x, content_y)) = self.apply_transform(click_x, click_y) {
            match tool {
                Tool::Select => {}
                Tool::Move => {}
                Tool::Rectangle => {}
                Tool::Circle => {}
                Tool::Brush => {
                    self.brush(painter, click_x, click_y, content_x, content_y, brush)?
                }
                Tool::Erase => self.brush(
                    painter,
                    click_x,
                    click_y,
                    content_x,
                    content_y,
                    &Self::empty(),
                )?,
                Tool::Bucket => {
                    self.bucket(painter, click_x, click_y, content_x, content_y, brush)?
                }
                Tool::ColorPicker => {}
                Tool::Text => {}
            }
        }

        Ok(())
    }

    fn drag(
        &mut self,
        painter: &mut Painter,
        tool: &Tool,
        brush: &StyledContent<char>,
        click_x: u16,
        click_y: u16,
    ) -> crossterm::Result<()> {
        if let Some((content_x, content_y)) = self.apply_transform(click_x, click_y) {
            match tool {
                Tool::Select => {}
                Tool::Move => {}
                Tool::Rectangle => {}
                Tool::Circle => {}
                Tool::Brush => {
                    self.brush(painter, click_x, click_y, content_x, content_y, brush)?
                }
                Tool::Erase => self.brush(
                    painter,
                    click_x,
                    click_y,
                    content_x,
                    content_y,
                    &Self::empty(),
                )?,
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
        click_x: u16,
        click_y: u16,
        content_x: usize,
        content_y: usize,
        brush: &StyledContent<char>,
    ) -> crossterm::Result<()> {
        self.content[content_y][content_x] = *brush;

        painter.write(click_x, click_y, brush)?;
        painter.flush()?;

        Ok(())
    }

    fn bucket(
        &mut self,
        painter: &mut Painter,
        click_x: u16,
        click_y: u16,
        content_x: usize,
        content_y: usize,
        brush: &StyledContent<char>,
    ) -> crossterm::Result<()> {
        let selected = self.content[content_y][content_x];

        self.apply_bucket(
            painter, click_x, click_y, content_x, content_y, brush, &selected,
        )?;

        self.draw_content(painter)?;
        painter.flush()?;

        Ok(())
    }

    fn apply_bucket(
        &mut self,
        painter: &mut Painter,
        x: u16,
        y: u16,
        content_x: usize,
        content_y: usize,
        brush: &StyledContent<char>,
        selected: &StyledContent<char>,
    ) -> crossterm::Result<()> {
        // Check that we're still inside canvas, if not return.
        if content_x >= self.content[0].len() || content_y >= self.content.len() {
            return Ok(());
        }

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
                x.add_sub_or_zero(&add.0),
                y.add_sub_or_zero(&add.1),
                content_x.add_sub_or_zero(&add.0),
                content_y.add_sub_or_zero(&add.1),
                brush,
                selected,
            )?;
        }

        Ok(())
    }
}

// Fields that depends on terminal window size.
struct Relative {
    // Transforms absolute position to its corresponding content position.
    transform: (i32, i32),

    // Visible content width and height.
    visible_content_size: (u16, u16),

    // Absolute position of content starting point.
    content_start: (u16, u16),
}

impl Relative {
    fn zero() -> Relative {
        Relative {
            transform: (0, 0),
            visible_content_size: (0, 0),
            content_start: (0, 0),
        }
    }

    fn new(
        transform: (i32, i32),
        visible_content_size: (u16, u16),
        content_start: (u16, u16),
    ) -> Relative {
        Relative {
            transform,
            visible_content_size,
            content_start,
        }
    }
}
