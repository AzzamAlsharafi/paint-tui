use std::cmp::min;

use crossterm::style::{Attribute, StyledContent, Stylize};

use crate::{constant::symbols, painter::Painter};

use super::area::Area;

pub struct RightPanel {
    pub area: Area,
    tools: Vec<Tool>,
    active_tool: usize, // index of current active tool
    pub brush: StyledContent<char>,
}

impl RightPanel {
    pub fn new(area: Area) -> RightPanel {
        RightPanel {
            area,
            tools: vec![
                Tool::Select,
                Tool::Move,
                Tool::Rectangle,
                Tool::Circle,
                Tool::Brush,
                Tool::Erase,
                Tool::Bucket,
                Tool::ColorPicker,
                Tool::Text,
            ],
            active_tool: 0,
            brush: 'X'.cyan(),
        }
    }

    pub fn get_tool(&self) -> &Tool {
        &self.tools[self.active_tool]
    }

    pub fn draw(&self, painter: &mut Painter, t_size: (u16, u16)) -> crossterm::Result<()> {
        let (x, y) = self.area.start.absolute_position(t_size);
        let (_, height) = self.area.size(t_size);

        for i in 0..self.tools.len() {
            let y = y + (i * 3) as u16;

            // Check if there's enought space for this button
            if height < 3 || y > height.abs_diff(3) {
                break;
            }

            if self.active_tool == i {
                painter.set_attribute(Attribute::Reverse)?;
            }

            painter.draw_box(x, y, 5, 3)?;

            painter.write(x + 1, y + 1, self.tools[i].icon())?;

            if self.active_tool == i {
                painter.set_attribute(Attribute::Reset)?;
            }
        }

        Ok(())
    }

    // cx, cy: click x, click y.
    pub fn click(
        &mut self,
        painter: &mut Painter,
        _cx: u16,
        cy: u16,
        t_size: (u16, u16),
    ) -> crossterm::Result<()> {
        let (_, y) = self.area.start.absolute_position(t_size);
        let (_, height) = self.area.size(t_size);

        let visible_buttons = min(height / 3, self.tools.len() as u16);

        if (visible_buttons * 3) + y > cy {
            self.active_tool = usize::from((cy - y) / 3);

            self.draw(painter, t_size)?;
            painter.flush()?;
        }

        Ok(())
    }
}

pub enum Tool {
    Select,
    Move,
    Rectangle,
    Circle,
    Brush,
    Erase,
    Bucket,
    ColorPicker,
    Text,
}

impl Tool {
    fn icon(&self) -> &str {
        match self {
            Tool::Select => return symbols::SELECT,
            Tool::Move => return symbols::MOVE,
            Tool::Rectangle => return symbols::RECTANGLE,
            Tool::Circle => return symbols::CIRCLE,
            Tool::Brush => return symbols::BRUSH,
            Tool::Erase => return symbols::ERASE,
            Tool::Bucket => return symbols::BUCKET,
            Tool::ColorPicker => return symbols::COLOR_PICKET,
            Tool::Text => return symbols::TEXT,
        }
    }
}
