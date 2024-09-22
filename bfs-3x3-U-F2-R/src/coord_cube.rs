#![allow(dead_code)]

use crate::{
    cube::{Cube, CORNERS_SIZE},
    transposition_tables::TranspositionTables,
};

#[derive(Clone)]
pub struct CoordCube<'a> {
    pub edges: u32,
    pub corners: u32,
    is_even_perm: bool,
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
            edges: cube.ep_coord(),
            corners: cube.corners_coord(),
            is_even_perm: cube.is_even_perm,
            transposition_tables,
        }
    }

    pub fn u(&mut self) {
        self.edges = self.transposition_tables.u_edges[self.edges as usize];
        self.corners = self.transposition_tables.u_corners[self.corners as usize];
        self.is_even_perm = !self.is_even_perm;
    }

    pub fn r(&mut self) {
        self.edges = self.transposition_tables.r_edges[self.edges as usize];
        self.corners = self.transposition_tables.r_corners[self.corners as usize];
        self.is_even_perm = !self.is_even_perm;
    }

    pub fn f2(&mut self) {
        self.edges = if self.is_even_perm {
            self.transposition_tables.f2_edges_even[self.edges as usize]
        } else {
            self.transposition_tables.f2_edges_odd[self.edges as usize]
        };
        self.corners = self.transposition_tables.f2_corners[self.corners as usize];
    }

    pub fn do_move(&mut self, m: u8) {
        match m {
            0 => self.u(),
            1 => self.r(),
            2 => self.f2(),
            _ => panic!("Invalid move"),
        }
    }

    pub fn encode(&self) -> u64 {
        self.edges as u64 * CORNERS_SIZE as u64 + self.corners as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.corners = (coord % CORNERS_SIZE as u64) as u32;
        self.edges = (coord / CORNERS_SIZE as u64) as u32;

        let mut cube = Cube::new();
        cube.set_corners_coord(self.corners);

        self.is_even_perm = cube.is_even_perm;
    }
}
