

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    pub position: (usize, usize),
    pub pipe: char,
    pub is_start_position: bool
}

#[derive(Debug, Clone, PartialEq)]
enum NeighborType {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    NONE,
}

impl Tile {

    pub fn is_connected(&self, other: &Tile) -> bool {
        if self.is_neighbored(other) {
            let neighbor_type = self.get_neighbor_type(other);
            let connecting_pipes = match (self.pipe, neighbor_type) {
                ('|', NeighborType::NORTH) => vec!['|','F','7'],    // N,E,W
                ('|', NeighborType::SOUTH) => vec!['|','L','J'],    // S,E,W
                ('-', NeighborType::EAST) => vec!['-','J','7'],     // E,N,S
                ('-', NeighborType::WEST) => vec!['-','L','F'],     // E,N,S
                ('L', NeighborType::NORTH) => vec!['|','F','7'],    // N,E,W
                ('L', NeighborType::EAST) => vec!['-','J','7'],     // E,N,S
                ('J', NeighborType::NORTH) => vec!['|','F','7'],    // N,E,W
                ('J', NeighborType::WEST) => vec!['-','L','F'],     // W,N,S
                ('7', NeighborType::SOUTH) => vec!['|','L','J'],    // S,E,W
                ('7', NeighborType::WEST) => vec!['-','L','F'],     // W,N,S
                ('F', NeighborType::SOUTH) => vec!['|','L','J'],    // S,E,W
                ('F', NeighborType::EAST) => vec!['-','J','7'],     // E,N,S
                (_,_) => vec![]
            };
            return connecting_pipes.contains(&other.pipe)
        }
        false
    }

    fn is_neighbored(&self, other: &Tile) -> bool {
        self.get_neighbor_type(other) != NeighborType::NONE
    }

    fn get_neighbor_type(&self, other: &Tile) -> NeighborType {
        let (x,y) = self.position;
        let (other_x,other_y) = other.position;
        let x_dist = x.abs_diff(other_x);
        let y_dist = y.abs_diff(other_y);
        if (x_dist > 0 && y_dist > 0) || x_dist > 1 || y_dist > 1 {
            return NeighborType::NONE;
        }
        if x_dist > 0 && other_x > x {
            return NeighborType::SOUTH;
        }
        if x_dist > 0 && other_x < x {
            return NeighborType::NORTH;
        }
        if y_dist > 0 && other_y < y {
            return NeighborType::WEST;
        }
        if y_dist > 0 && other_y > y {
            return NeighborType::EAST;
        }
        panic!()
    }
}
