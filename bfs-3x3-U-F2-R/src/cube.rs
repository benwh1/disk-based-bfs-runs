#![allow(dead_code)]

use crate::coord_cube::CoordCube;

pub const EP_SIZE: u32 = 181440;
pub const CP_SIZE: u32 = 5040;
pub const CO_SIZE: u32 = 729;
pub const CORNERS_SIZE: u32 = CP_SIZE * CO_SIZE;
pub const STATE_SIZE: u64 = EP_SIZE as u64 * CORNERS_SIZE as u64;

#[derive(Debug, PartialEq)]
pub struct Cube {
    ep: [u8; 9],
    cp: [u8; 7],
    co: [u8; 7],
    pub(super) is_even_perm: bool,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            cp: [0, 1, 2, 3, 4, 5, 6],
            co: [0; 7],
            is_even_perm: true,
        }
    }

    pub fn u(&mut self) {
        let a = self.ep[0];
        self.ep[0] = self.ep[3];
        self.ep[3] = self.ep[2];
        self.ep[2] = self.ep[1];
        self.ep[1] = a;
        let a = self.cp[0];
        self.cp[0] = self.cp[3];
        self.cp[3] = self.cp[2];
        self.cp[2] = self.cp[1];
        self.cp[1] = a;
        let a = self.co[0];
        self.co[0] = self.co[3];
        self.co[3] = self.co[2];
        self.co[2] = self.co[1];
        self.co[1] = a;
        self.is_even_perm = !self.is_even_perm;
    }

    pub fn r(&mut self) {
        let a = self.ep[3];
        self.ep[3] = self.ep[4];
        self.ep[4] = self.ep[5];
        self.ep[5] = self.ep[6];
        self.ep[6] = a;
        let a = self.cp[2];
        self.cp[2] = self.cp[3];
        self.cp[3] = self.cp[4];
        self.cp[4] = self.cp[5];
        self.cp[5] = a;
        let a = self.co[2];
        self.co[2] = (self.co[3] + 1) % 3;
        self.co[3] = (self.co[4] + 2) % 3;
        self.co[4] = (self.co[5] + 1) % 3;
        self.co[5] = (a + 2) % 3;
        self.is_even_perm = !self.is_even_perm;
    }

    pub fn f2(&mut self) {
        self.ep.swap(0, 7);
        self.ep.swap(4, 8);
        self.cp.swap(0, 4);
        self.cp.swap(3, 6);
        self.co.swap(0, 4);
        self.co.swap(3, 6);
    }

    pub fn do_move(&mut self, m: u8) {
        match m {
            0 => self.u(),
            1 => self.r(),
            2 => self.f2(),
            _ => panic!("Invalid move"),
        }
    }

    pub fn up(&mut self) {
        self.u();
        self.u();
        self.u();
    }

    pub fn rp(&mut self) {
        self.r();
        self.r();
        self.r();
    }

    pub fn ep_coord(&self) -> u32 {
        combinatorics::indexing::encode_even_permutation(self.ep) as u32
    }

    fn cp_coord(&self) -> u32 {
        combinatorics::indexing::encode_permutation(self.cp) as u32
    }

    fn co_coord(&self) -> u32 {
        self.co.iter().take(6).fold(0, |acc, &x| 3 * acc + x as u32)
    }

    pub fn set_ep_coord(&mut self, coord: u32) {
        self.ep = combinatorics::indexing::decode_even_permutation(coord as u64);
        if !self.is_even_perm {
            self.ep.swap(7, 8);
        }
    }

    fn set_cp_coord(&mut self, coord: u32) {
        self.cp = combinatorics::indexing::decode_permutation(coord as u64);
    }

    fn set_co_coord(&mut self, mut coord: u32) {
        let mut tot = 0;
        for i in (0..6).rev() {
            let a = (coord % 3) as u8;
            self.co[i] = a;
            tot += a;
            coord /= 3;
        }
        self.co[6] = (3 - tot % 3) % 3;
    }

    pub fn corners_coord(&self) -> u32 {
        self.cp_coord() * CO_SIZE + self.co_coord()
    }

    pub fn set_corners_coord(&mut self, coord: u32) {
        self.set_cp_coord(coord / CO_SIZE);
        self.set_co_coord(coord % CO_SIZE);
        self.is_even_perm = combinatorics::sign::is_even(self.cp);
    }

    fn encode(&self) -> u64 {
        self.ep_coord() as u64 * CORNERS_SIZE as u64 + self.corners_coord() as u64
    }

    fn decode(&mut self, state: u64) {
        // Must set corners first because it sets parity, which is used by ep
        self.set_corners_coord((state % CORNERS_SIZE as u64) as u32);
        self.set_ep_coord((state / CORNERS_SIZE as u64) as u32);
    }
}

impl From<CoordCube<'_>> for Cube {
    fn from(value: CoordCube<'_>) -> Self {
        let mut cube = Cube::new();
        cube.set_corners_coord(value.corners);
        cube.set_ep_coord(value.edges);
        cube
    }
}

#[cfg(test)]
mod tests {
    use crate::transposition_tables::TranspositionTables;

    use super::*;

    #[test]
    fn test_cube_decode_encode() {
        let mut x = 0u64;
        let mut cube = Cube::new();
        for _ in 0..65536 {
            x = x
                .wrapping_mul(450349535401847371)
                .wrapping_add(380506838312516788);
            let coord = x % STATE_SIZE;
            cube.decode(coord);
            assert_eq!(cube.encode(), coord);
            assert_eq!(combinatorics::sign::is_even(cube.cp), cube.is_even_perm);
            assert_eq!(combinatorics::sign::is_even(cube.ep), cube.is_even_perm);
        }
    }

    #[test]
    fn test_coord_cube_decode_encode() {
        let transposition_tables = TranspositionTables::new();
        let mut cube = CoordCube::new(&transposition_tables);

        let mut x = 0u64;
        for _ in 0..65536 {
            x = x
                .wrapping_mul(450349535401847371)
                .wrapping_add(380506838312516788);
            let coord = x % STATE_SIZE;
            cube.decode(coord);
            assert_eq!(cube.encode(), coord);
        }
    }

    #[test]
    fn test_cube_matches_coord_cube() {
        let transposition_tables = TranspositionTables::new();

        let mut cube = Cube::new();
        let mut coord_cube = CoordCube::new(&transposition_tables);

        let mut x = 0u64;
        for _ in 0..65536 {
            x = x
                .wrapping_mul(450349535401847371)
                .wrapping_add(380506838312516788);
            let mv = (x % 3) as u8;

            cube.do_move(mv);
            coord_cube.do_move(mv);

            assert_eq!(cube.encode(), coord_cube.encode());
        }
    }

    #[test]
    fn test_cube_matches_coord_cube_2() {
        let transposition_tables = TranspositionTables::new();

        let mut cube = Cube::new();
        let mut coord_cube = CoordCube::new(&transposition_tables);

        let mut x = 0u64;
        for _ in 0..65536 {
            x = x
                .wrapping_mul(450349535401847371)
                .wrapping_add(380506838312516788);
            let enc = x % STATE_SIZE;

            cube.decode(enc);
            coord_cube.decode(enc);
            let cube2 = Cube::from(coord_cube.clone());

            assert_eq!(cube, cube2);
            assert_eq!(cube.encode(), coord_cube.encode());
        }
    }

    #[test]
    fn test_random_scramble() {
        // Scramble: U R U F2 R F2 F2 R U U U U U R U F2 U R U U U U U F2 R F2 U R F2 R U U F2 R R
        // F2 F2 U U U F2 U R F2 R U R R F2 R U F2 F2 U U F2 F2 R F2 F2 R F2 U R U U U F2 U R R R U
        // F2 U F2 R U R U R U U U R F2 F2 F2 R R U R U U U R F2 F2 U R
        let moves = [
            0, 1, 0, 2, 1, 2, 2, 1, 0, 0, 0, 0, 0, 1, 0, 2, 0, 1, 0, 0, 0, 0, 0, 2, 1, 2, 0, 1, 2,
            1, 0, 0, 2, 1, 1, 2, 2, 0, 0, 0, 2, 0, 1, 2, 1, 0, 1, 1, 2, 1, 0, 2, 2, 0, 0, 2, 2, 1,
            2, 2, 1, 2, 0, 1, 0, 0, 0, 2, 0, 1, 1, 1, 0, 2, 0, 2, 1, 0, 1, 0, 1, 0, 0, 0, 1, 2, 2,
            2, 1, 1, 0, 1, 0, 0, 0, 1, 2, 2, 0, 1,
        ];

        let mut cube = Cube::new();
        for mv in moves {
            cube.do_move(mv);
        }

        // Solution: F2 R U F2 U2 R F2 U' R2 U' F2 R2 U R2 U2 R2 U2 F2 R2
        let solution = [
            2, 1, 0, 2, 0, 0, 1, 2, 0, 0, 0, 1, 1, 0, 0, 0, 2, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 2,
            1, 1,
        ];

        for mv in solution {
            cube.do_move(mv);
        }

        let solved_cube = Cube::new();
        assert_eq!(cube.encode(), solved_cube.encode());
    }
}
