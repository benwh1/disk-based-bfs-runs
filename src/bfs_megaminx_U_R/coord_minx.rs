use crate::bfs_megaminx_U_R::{
    minx::{Megaminx, EP_SIZE},
    transposition_tables::TranspositionTables,
};

#[derive(Clone)]
pub struct CoordMinx<'a> {
    pub(super) corners: u32,
    pub(super) edges: u32,
    transposition_tables: &'a TranspositionTables,
}

impl<'a> std::fmt::Debug for CoordMinx<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoordMinx")
            .field("corners", &self.corners)
            .field("edges", &self.edges)
            .finish()
    }
}

impl<'a> CoordMinx<'a> {
    pub fn new(transposition_tables: &'a TranspositionTables) -> Self {
        let minx = Megaminx::new();
        Self {
            corners: minx.corners_coord(),
            edges: minx.ep_coord(),
            transposition_tables,
        }
    }

    pub fn is_solved(&self) -> bool {
        Megaminx::from(self).is_solved()
    }

    pub fn u(&mut self) {
        self.corners = self.transposition_tables.u_corners[self.corners as usize];
        self.edges = self.transposition_tables.u_edges[self.edges as usize];
    }

    pub fn r(&mut self) {
        self.corners = self.transposition_tables.r_corners[self.corners as usize];
        self.edges = self.transposition_tables.r_edges[self.edges as usize];
    }

    pub fn do_move(&mut self, mv: &str) {
        match mv {
            "U" => self.u(),
            "U2" => (0..2).for_each(|_| self.u()),
            "U2'" => (0..3).for_each(|_| self.u()),
            "U'" => (0..4).for_each(|_| self.u()),
            "R" => self.r(),
            "R2" => (0..2).for_each(|_| self.r()),
            "R2'" => (0..3).for_each(|_| self.r()),
            "R'" => (0..4).for_each(|_| self.r()),
            _ => panic!("Invalid move"),
        }
    }

    pub fn do_alg(&mut self, alg: &str) {
        alg.split_whitespace().for_each(|mv| self.do_move(mv));
    }

    pub fn encode(&self) -> u64 {
        self.corners as u64 * EP_SIZE as u64 + self.edges as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.corners = (coord / EP_SIZE as u64) as u32;
        self.edges = (coord % EP_SIZE as u64) as u32;
    }
}
