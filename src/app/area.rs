// Describes a rectangular area in the terminal screen.
pub struct Area {
    pub start: Point,
    pub end: Point
}

impl Area {
    pub fn new(start: Point, end: Point) -> Area {
        Area { start, end }
    }

    pub fn size(&self, t_width: u16, t_height: u16) -> (u16, u16) {
        let abs_start = self.start.absolute_position(t_width, t_height);
        let abs_end = self.end.absolute_position(t_width, t_height);

        (abs_start.0.abs_diff(abs_end.0) + 1, abs_start.1.abs_diff(abs_end.1) + 1)
    }
}

pub struct Point {
    x: u16,
    y: u16,
    corner: Corner
}

impl Point {
    pub fn new(x: u16, y: u16, corner: Corner) -> Point {
        Point { x, y, corner }
    }
    
    // Returns absolute position in terminal screen for this point.
    // In terminal screen (0, 0) is the top left corner.
    pub fn absolute_position(&self, t_width: u16, t_height: u16) -> (u16, u16) {
        let t_width = t_width - 1;
        let t_height = t_height - 1;

        match self.corner {
            Corner::TopLeft => return (self.x, self.y),
            Corner::TopRight => return (t_width - self.x, self.y),
            Corner::BottomLeft => return (self.x, t_height - self.y),
            Corner::BottomRight => return (t_width - self.x, t_height - self.y),
        }
    }
}

// Origin point location for a particular Point instance.
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}