use crate::{cube::Cube, transposition_tables::TranspositionTables};

#[derive(Clone)]
pub struct CoordCube<'a> {
    edges: u32,
    centers_corners: u32,
    transposition_tables: &'a TranspositionTables,
}

impl<'a> CoordCube<'a> {
    pub fn new(transposition_tables: &'a TranspositionTables) -> Self {
        let cube = Cube::new();
        Self {
            edges: cube.edge_coord(),
            centers_corners: cube.center_corner_coord(),
            transposition_tables,
        }
    }

    pub fn u(&mut self) {
        self.edges = self.transposition_tables.u_edges[self.edges as usize];
        self.centers_corners =
            self.transposition_tables.u_centers_corners[self.centers_corners as usize];
    }

    pub fn u2(&mut self) {
        self.edges = self.transposition_tables.u2_edges[self.edges as usize];
        self.centers_corners =
            self.transposition_tables.u2_centers_corners[self.centers_corners as usize];
    }

    pub fn ur(&mut self) {
        self.edges = self.transposition_tables.ur_edges[self.edges as usize];
        self.centers_corners =
            self.transposition_tables.ur_centers_corners[self.centers_corners as usize];
    }

    pub fn r2(&mut self) {
        self.edges = self.transposition_tables.r2_edges[self.edges as usize];
        self.centers_corners =
            self.transposition_tables.r2_centers_corners[self.centers_corners as usize];
    }

    pub fn encode(&self) -> u64 {
        self.edges as u64 * 75600 + self.centers_corners as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.edges = (coord / 75600) as u32;
        self.centers_corners = (coord % 75600) as u32;
    }
}
