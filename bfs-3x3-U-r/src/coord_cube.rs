#![allow(dead_code)]

use crate::{cube::Cube, transposition_tables::TranspositionTables};

#[derive(Clone)]
pub struct CoordCube<'a> {
    pub perm: u32,
    pub ori: u32,
    transposition_tables: &'a TranspositionTables,
}

impl<'a> std::fmt::Debug for CoordCube<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoordCube")
            .field("perm", &self.perm)
            .field("ori", &self.ori)
            .finish()
    }
}

impl<'a> CoordCube<'a> {
    pub fn new(transposition_tables: &'a TranspositionTables) -> Self {
        let cube = Cube::new();
        Self {
            perm: cube.perm_coord(),
            ori: cube.ori_coord(),
            transposition_tables,
        }
    }

    pub fn u(&mut self) {
        self.perm = self.transposition_tables.u_perm[self.perm as usize];
        self.ori = self.transposition_tables.u_ori[self.ori as usize];
    }

    pub fn u2(&mut self) {
        self.u();
        self.u();
    }

    pub fn u_inv(&mut self) {
        self.u();
        self.u();
        self.u();
    }

    pub fn rw(&mut self) {
        self.perm = self.transposition_tables.rw_perm[self.perm as usize];
        self.ori = self.transposition_tables.rw_ori[self.ori as usize];
    }

    pub fn rw2(&mut self) {
        self.rw();
        self.rw();
    }

    pub fn rw_inv(&mut self) {
        self.rw();
        self.rw();
        self.rw();
    }

    pub fn do_move(&mut self, mv: &str) {
        match mv {
            "U" => self.u(),
            "U2" => self.u2(),
            "U'" => self.u_inv(),
            "r" => self.rw(),
            "r2" => self.rw2(),
            "r'" => self.rw_inv(),
            _ => panic!("Invalid move {mv}"),
        }
    }

    pub fn encode(&self) -> u64 {
        self.perm as u64 * 62208 + self.ori as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.perm = (coord / 62208) as u32;
        self.ori = (coord % 62208) as u32;
    }
}
