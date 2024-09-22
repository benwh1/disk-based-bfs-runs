pub struct Cube {
    corners: u8,
    edges: [u8; 10],
    centers: [u8; 10],
}

impl Cube {
    pub fn new() -> Self {
        Self {
            corners: 0,
            edges: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            centers: [0, 0, 0, 0, 1, 1, 2, 2, 3, 3],
        }
    }

    pub fn u(&mut self) {
        self.corners = (self.corners + 1) % 4;
        self.edges[0..8].rotate_right(2);
        self.centers[0..4].rotate_right(1);
    }

    pub fn uinv(&mut self) {
        self.corners = (self.corners + 3) % 4;
        self.edges[0..8].rotate_left(2);
        self.centers[0..4].rotate_left(1);
    }

    pub fn r(&mut self) {
        let x = self.edges[0];
        self.edges[0] = self.edges[8];
        self.edges[8] = self.edges[9];
        self.edges[9] = self.edges[5];
        self.edges[5] = x;
        self.centers[2..10].rotate_left(2);
    }

    pub fn rinv(&mut self) {
        let x = self.edges[0];
        self.edges[0] = self.edges[5];
        self.edges[5] = self.edges[9];
        self.edges[9] = self.edges[8];
        self.edges[8] = x;
        self.centers[2..10].rotate_right(2);
    }

    pub fn edge_coord(&self) -> u32 {
        combinatorics::indexing::encode_permutation(self.edges) as u32
    }

    pub fn center_coord(&self) -> u32 {
        combinatorics::indexing::encode_multiset(self.centers, [4, 2, 2, 2]) as u32
    }

    pub fn center_corner_coord(&self) -> u32 {
        self.center_coord() * 4 + self.corners as u32
    }

    pub fn set_edge_coord(&mut self, coord: u32) {
        self.edges = combinatorics::indexing::decode_permutation(coord as u64);
    }

    pub fn set_center_coord(&mut self, coord: u32) {
        self.centers = combinatorics::indexing::decode_multiset(coord as u128, [4, 2, 2, 2]);
    }

    pub fn set_center_corner_coord(&mut self, coord: u32) {
        self.corners = (coord % 4) as u8;
        self.set_center_coord(coord / 4);
    }
}
