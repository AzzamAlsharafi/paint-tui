// Describes a rectangular area in the terminal screen.
pub struct Area {
    pub start: Point,
    pub end: Point,
}

impl Area {
    pub fn new(start: Point, end: Point) -> Area {
        Area { start, end }
    }

    pub fn size(&self, t_width: u16, t_height: u16) -> (u16, u16) {
        let abs_start = self.start.absolute_position(t_width, t_height);
        let abs_end = self.end.absolute_position(t_width, t_height);

        // If end point is before start point then this is not a valid area.
        if abs_start.0 > abs_end.0 || abs_start.1 > abs_start.1 {
            return (0, 0);
        }

        (
            abs_start.0.abs_diff(abs_end.0) + 1,
            abs_start.1.abs_diff(abs_end.1) + 1,
        )
    }

    // Check if this position (absolute) is inside this area.
    pub fn check_inside(&self, x: u16, y: u16, t_width: u16, t_height: u16) -> bool{
        let abs_start = self.start.absolute_position(t_width, t_height);
        let abs_end = self.end.absolute_position(t_width, t_height);

        // If end point is before start point then this is not a valid area.
        if abs_start.0 > abs_end.0 || abs_start.1 > abs_start.1 {
            return false;
        }

        x >= abs_start.0 && x <= abs_end.0 && y >= abs_start.1 && y <= abs_end.1
    }
}

pub struct Point {
    x: u16,
    y: u16,
    corner: Corner,
}

impl Point {
    pub fn new(x: u16, y: u16, corner: Corner) -> Point {
        Point { x, y, corner }
    }

    // Returns absolute position in terminal screen for this point.
    // In terminal screen (0, 0) is the top left corner.
    pub fn absolute_position(&self, t_width: u16, t_height: u16) -> (u16, u16) {
        if t_width == 0 || t_height == 0 {
            return (0, 0);
        }

        let t_width = t_width - 1;
        let t_height = t_height - 1;

        match self.corner {
            Corner::TopLeft => return (self.x, self.y),
            Corner::_TopRight => return (Point::diff_or_zero(t_width, self.x), self.y),
            Corner::BottomLeft => return (self.x, Point::diff_or_zero(t_height, self.y)),
            Corner::_BottomRight => {
                return (
                    Point::diff_or_zero(t_width, self.x),
                    Point::diff_or_zero(t_height, self.y),
                )
            }
        }
    }

    fn diff_or_zero(a: u16, b: u16) -> u16 {
        if a > b {
            return a - b;
        } else {
            return 0;
        }
    }
}

// Origin point location for a particular Point instance.
pub enum Corner {
    TopLeft,
    _TopRight,
    BottomLeft,
    _BottomRight,
}
