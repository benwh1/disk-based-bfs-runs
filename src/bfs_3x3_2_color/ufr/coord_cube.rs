use crate::bfs_3x3_2_color::ufr::{
    cube::{Cube, CORNERS_SIZE},
    transposition_tables::TranspositionTables,
};

#[derive(Clone)]
pub struct CoordCube<'a> {
    pub edges: u32,
    pub corners: u32,
    transposition_tables: &'a TranspositionTables,
}

impl<'a> std::fmt::Debug for CoordCube<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoordCube")
            .field("edges", &self.edges)
            .field("corners", &self.corners)
            .finish()
    }
}

impl<'a> CoordCube<'a> {
    pub fn new(transposition_tables: &'a TranspositionTables) -> Self {
        let cube = Cube::new();
        Self {
            edges: cube.edges_coord(),
            corners: cube.corners_coord(),
            transposition_tables,
        }
    }

    pub fn u(&mut self) {
        self.edges = self.transposition_tables.u_edges[self.edges as usize];
        self.corners = self.transposition_tables.u_corners[self.corners as usize];
    }

    pub fn l(&mut self) {
        self.edges = self.transposition_tables.l_edges[self.edges as usize];
        self.corners = self.transposition_tables.l_corners[self.corners as usize];
    }

    pub fn f(&mut self) {
        self.edges = self.transposition_tables.f_edges[self.edges as usize];
        self.corners = self.transposition_tables.f_corners[self.corners as usize];
    }

    pub fn r(&mut self) {
        self.edges = self.transposition_tables.r_edges[self.edges as usize];
        self.corners = self.transposition_tables.r_corners[self.corners as usize];
    }

    pub fn b(&mut self) {
        self.edges = self.transposition_tables.b_edges[self.edges as usize];
        self.corners = self.transposition_tables.b_corners[self.corners as usize];
    }

    pub fn d(&mut self) {
        self.edges = self.transposition_tables.d_edges[self.edges as usize];
        self.corners = self.transposition_tables.d_corners[self.corners as usize];
    }

    pub fn encode(&self) -> u64 {
        self.edges as u64 * CORNERS_SIZE as u64 + self.corners as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.edges = (coord / CORNERS_SIZE as u64) as u32;
        self.corners = (coord % CORNERS_SIZE as u64) as u32;
    }
}

#[cfg(test)]
mod tests {

    use crate::bfs_3x3_2_color::ufr::cube::STATE_SIZE;

    use super::*;

    #[test]
    fn test_coord_cube() {
        let transposition_tables = TranspositionTables::new();
        let mut coord_cube = CoordCube::new(&transposition_tables);
        let mut cube = Cube::new();

        let mut x = 0u64;
        for _ in 0..65536 {
            x = x.wrapping_mul(450349535401847371);
            x = x.wrapping_add(380506838312516788);

            let coord = x % STATE_SIZE as u64;

            cube.decode(coord);
            coord_cube.decode(coord);

            println!("coord = {coord}");
            println!("cube = {cube:?}");
            for i in 0..4 {
                coord_cube.u();
                cube.u();
                println!("u{}", i + 1);
                println!("cube = {cube:?}");
                assert_eq!(cube.encode(), coord_cube.encode());
            }
            for i in 0..4 {
                coord_cube.l();
                cube.l();
                println!("l{}", i + 1);
                println!("cube = {cube:?}");
                assert_eq!(cube.encode(), coord_cube.encode());
            }
            for i in 0..4 {
                coord_cube.f();
                cube.f();
                println!("f{}", i + 1);
                assert_eq!(cube.encode(), coord_cube.encode());
            }
            for i in 0..4 {
                coord_cube.r();
                cube.r();
                println!("r{}", i + 1);
                assert_eq!(cube.encode(), coord_cube.encode());
            }
            for i in 0..4 {
                coord_cube.b();
                cube.b();
                println!("b{}", i + 1);
                assert_eq!(cube.encode(), coord_cube.encode());
            }
            for i in 0..4 {
                coord_cube.d();
                cube.d();
                println!("d{}", i + 1);
                assert_eq!(cube.encode(), coord_cube.encode());
            }
        }
    }
}
