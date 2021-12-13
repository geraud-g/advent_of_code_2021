#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Point {
    pub y: usize,
    pub x: usize,
}


impl Point {
    pub fn from_usize(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    pub fn get_neighbours(&self, height: usize, width: usize) -> Vec<Self> {
        // TODO : Impl iterator instead
        let mut neighbours = vec![];

        if self.y > 0 { // UP
            neighbours.push(Point::from_usize(self.y - 1, self.x));
        }

        if self.y < (height - 1) { // DOWN
            neighbours.push(Point::from_usize(self.y + 1, self.x));
        }
        if self.x > 0 { // LEFT
            neighbours.push(Point::from_usize(self.y, self.x - 1));
        }
        if self.x < (width - 1) { // RIGHT
            neighbours.push(Point::from_usize(self.y, self.x + 1));
        }
        neighbours
    }

    pub fn get_neighbours_with_diag(&self, height: usize, width: usize) -> Vec<Self> {
        // TODO : Impl iterator instead
        let mut neighbours = vec![];

        if self.y > 0 { // UP
            neighbours.push(Point::from_usize(self.y - 1, self.x));
            if self.x > 0 { // UP LEFT
                neighbours.push(Point::from_usize(self.y - 1, self.x - 1));
            }
            if self.x < (width - 1) { // UP RIGHT
                neighbours.push(Point::from_usize(self.y - 1, self.x + 1));
            }
        }

        if self.y < (height - 1) { // DOWN
            neighbours.push(Point::from_usize(self.y + 1, self.x));
            if self.x > 0 { // DOWN LEFT
                neighbours.push(Point::from_usize(self.y + 1, self.x - 1));
            }
            if self.x < (width - 1) { // DOWN RIGHT
                neighbours.push(Point::from_usize(self.y + 1, self.x + 1));
            }
        }


        if self.x > 0 { // LEFT
            neighbours.push(Point::from_usize(self.y, self.x - 1));
        }
        if self.x < (width - 1) { // RIGHT
            neighbours.push(Point::from_usize(self.y, self.x + 1));
        }
        neighbours
    }
}
