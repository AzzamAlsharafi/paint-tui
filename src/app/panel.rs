use std::cmp::min;

use crossterm::style::{Attribute, StyledContent, Stylize};

use crate::{constant::symbols, painter::Painter};

use super::area::Area;

pub struct RightPanel {
    pub area: Area,
    tools: Vec<Tool>,
    active_tool: usize, // index of current active tool
    pub brush: StyledContent<char>,
    relative: Relative, // Fields that depends on terminal window size.
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
            relative: Relative::zero(),
        }
    }

    pub fn get_tool(&self) -> &Tool {
        &self.tools[self.active_tool]
    }

    pub fn draw(&mut self, painter: &mut Painter, t_size: (u16, u16)) -> crossterm::Result<()> {
        self.set_relative(t_size);

        self.draw_panel(painter)
    }

    fn set_relative(&mut self, t_size: (u16, u16)) {
        let (_, height) = self.area.size(t_size);

        let visible_buttons = min(height / 3, self.tools.len() as u16);
        let panel_start = self.area.start.absolute_position(t_size);

        self.relative = Relative::new(visible_buttons, panel_start);
    }

    fn draw_panel(&self, painter: &mut Painter) -> crossterm::Result<()> {
        let (start_x, start_y) = self.relative.panel_start;

        for i in 0..self.relative.visible_buttons {
            let (x, y) = (start_x, start_y + (i * 3));
            let tool_index = usize::from(i);

            if self.active_tool == tool_index {
                painter.set_attribute(Attribute::Reverse)?;
            }

            painter.draw_box(x, y, 5, 3)?;

            painter.write(x + 1, y + 1, self.tools[tool_index].icon())?;

            if self.active_tool == tool_index {
                painter.set_attribute(Attribute::Reset)?;
            }
        }

        Ok(())
    }

    pub fn click(
        &mut self,
        painter: &mut Painter,
        _click_x: u16,
        click_y: u16,
    ) -> crossterm::Result<()> {
        let (_, start_y) = self.relative.panel_start;
        let visible_buttons = self.relative.visible_buttons;

        // This should never be true, but just in case.
        if click_y < start_y {
            return Ok(());
        }

        let index = (click_y - start_y) / 3;

        if index < visible_buttons {
           self.active_tool = usize::from(index);
           
           self.draw_panel(painter)?;
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

// Fields that depends on terminal window size.
struct Relative {
    visible_buttons: u16,
    panel_start: (u16, u16)
}

impl Relative {
    fn zero() -> Relative {
        Relative { visible_buttons: 0, panel_start: (0, 0) }
    }

    fn new(visible_buttons: u16, panel_start: (u16, u16)) -> Relative {
        Relative { visible_buttons, panel_start }
    }
}
