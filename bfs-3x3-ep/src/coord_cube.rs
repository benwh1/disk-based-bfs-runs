use crate::{cube::Cube, transposition_tables::TranspositionTables};

#[derive(Clone)]
pub struct CoordCube<'a> {
    pub ep: u32,
    transposition_tables: &'a TranspositionTables,
}

impl<'a> std::fmt::Debug for CoordCube<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoordCube").field("ep", &self.ep).finish()
    }
}

impl<'a> CoordCube<'a> {
    pub fn new(transposition_tables: &'a TranspositionTables) -> Self {
        let cube = Cube::new();
        Self {
            ep: cube.ep_coord(),
            transposition_tables,
        }
    }

    pub fn u(&mut self) {
        self.ep = self.transposition_tables.u[self.ep as usize];
    }

    pub fn l(&mut self) {
        self.ep = self.transposition_tables.l[self.ep as usize];
    }

    pub fn f(&mut self) {
        self.ep = self.transposition_tables.f[self.ep as usize];
    }

    pub fn r(&mut self) {
        self.ep = self.transposition_tables.r[self.ep as usize];
    }

    pub fn b(&mut self) {
        self.ep = self.transposition_tables.b[self.ep as usize];
    }

    pub fn d(&mut self) {
        self.ep = self.transposition_tables.d[self.ep as usize];
    }

    pub fn encode(&self) -> u64 {
        self.ep as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.ep = coord as u32;
    }
}
