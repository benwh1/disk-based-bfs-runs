#![allow(dead_code)]

use crate::coord_cube::CoordCube;

pub const EP_SIZE: u32 = 18480;
pub const EO_SIZE: u32 = 64;
pub const CP_SIZE: u32 = 1120;
pub const CO_SIZE: u32 = 729;
pub const EDGES_SIZE: u32 = EP_SIZE * EO_SIZE;
pub const CORNERS_SIZE: u32 = CP_SIZE * CO_SIZE;
pub const STATE_SIZE: u64 = EDGES_SIZE as u64 * CORNERS_SIZE as u64;

#[derive(Debug)]
pub struct Cube {
    ep: [u8; 12],
    eo: [u8; 12],
    cp: [u8; 8],
    co: [u8; 8],
}

impl Cube {
    pub fn new() -> Self {
        Self {
            ep: [0, 1, 1, 0, 1, 2, 1, 0, 2, 2, 1, 1],
            eo: [0; 12],
            cp: [0, 1, 0, 2, 3, 1, 0, 1],
            co: [0; 8],
        }
    }

    pub fn is_solved(&self) -> bool {
        if self.ep != [0, 1, 1, 0, 1, 2, 1, 0, 2, 2, 1, 1] || self.cp != [0, 1, 0, 2, 3, 1, 0, 1] {
            return false;
        }

        for i in [1, 2, 4, 6, 10, 11] {
            if self.eo[i] != 0 {
                return false;
            }
        }

        for i in [0, 1, 2, 5, 6, 7] {
            if self.co[i] != 0 {
                return false;
            }
        }

        true
    }

    pub fn u(&mut self) {
        let a = self.ep[0];
        self.ep[0] = self.ep[3];
        self.ep[3] = self.ep[2];
        self.ep[2] = self.ep[1];
        self.ep[1] = a;
        let a = self.eo[0];
        self.eo[0] = self.eo[3];
        self.eo[3] = self.eo[2];
        self.eo[2] = self.eo[1];
        self.eo[1] = a;
        let a = self.cp[0];
        self.cp[0] = self.cp[3];
        self.cp[3] = self.cp[2];
        self.cp[2] = self.cp[1];
        self.cp[1] = a;
        let a = self.co[0];
        self.co[0] = (self.co[3] + 1) % 3;
        self.co[3] = (self.co[2] + 1) % 3;
        self.co[2] = (self.co[1] + 2) % 3;
        self.co[1] = (a + 2) % 3;
    }

    fn x(&mut self) {
        let a = self.ep[0];
        self.ep[0] = self.ep[10];
        self.ep[10] = self.ep[8];
        self.ep[8] = self.ep[2];
        self.ep[2] = a;
        let a = self.ep[1];
        self.ep[1] = self.ep[4];
        self.ep[4] = self.ep[9];
        self.ep[9] = self.ep[5];
        self.ep[5] = a;
        let a = self.ep[3];
        self.ep[3] = self.ep[7];
        self.ep[7] = self.ep[11];
        self.ep[11] = self.ep[6];
        self.ep[6] = a;

        let a = self.eo[0];
        self.eo[0] = self.eo[10];
        self.eo[10] = self.eo[8];
        self.eo[8] = (self.eo[2] + 1) % 2;
        self.eo[2] = (a + 1) % 2;
        let a = self.eo[1];
        self.eo[1] = self.eo[4];
        self.eo[4] = self.eo[9];
        self.eo[9] = self.eo[5];
        self.eo[5] = a;
        let a = self.eo[3];
        self.eo[3] = self.eo[7];
        self.eo[7] = (self.eo[11] + 1) % 2;
        self.eo[11] = self.eo[6];
        self.eo[6] = (a + 1) % 2;

        let a = self.cp[0];
        self.cp[0] = self.cp[5];
        self.cp[5] = self.cp[4];
        self.cp[4] = self.cp[1];
        self.cp[1] = a;
        let a = self.cp[2];
        self.cp[2] = self.cp[3];
        self.cp[3] = self.cp[6];
        self.cp[6] = self.cp[7];
        self.cp[7] = a;

        let a = self.co[0];
        self.co[0] = (self.co[5] + 1) % 3;
        self.co[5] = self.co[4];
        self.co[4] = (self.co[1] + 1) % 3;
        self.co[1] = (a + 1) % 3;
        let a = self.co[2];
        self.co[2] = self.co[3];
        self.co[3] = (self.co[6] + 2) % 3;
        self.co[6] = (self.co[7] + 2) % 3;
        self.co[7] = (a + 2) % 3;
    }

    fn y(&mut self) {
        let a = self.ep[0];
        self.ep[0] = self.ep[3];
        self.ep[3] = self.ep[2];
        self.ep[2] = self.ep[1];
        self.ep[1] = a;
        let a = self.ep[4];
        self.ep[4] = self.ep[7];
        self.ep[7] = self.ep[6];
        self.ep[6] = self.ep[5];
        self.ep[5] = a;
        let a = self.ep[8];
        self.ep[8] = self.ep[9];
        self.ep[9] = self.ep[10];
        self.ep[10] = self.ep[11];
        self.ep[11] = a;

        let a = self.eo[0];
        self.eo[0] = self.eo[3];
        self.eo[3] = self.eo[2];
        self.eo[2] = self.eo[1];
        self.eo[1] = a;
        let a = self.eo[4];
        self.eo[4] = (self.eo[7] + 1) % 2;
        self.eo[7] = self.eo[6];
        self.eo[6] = self.eo[5];
        self.eo[5] = (a + 1) % 2;
        let a = self.eo[8];
        self.eo[8] = self.eo[9];
        self.eo[9] = (self.eo[10] + 1) % 2;
        self.eo[10] = self.eo[11];
        self.eo[11] = (a + 1) % 2;

        let a = self.cp[0];
        self.cp[0] = self.cp[3];
        self.cp[3] = self.cp[2];
        self.cp[2] = self.cp[1];
        self.cp[1] = a;
        let a = self.cp[4];
        self.cp[4] = self.cp[5];
        self.cp[5] = self.cp[6];
        self.cp[6] = self.cp[7];
        self.cp[7] = a;

        let a = self.co[0];
        self.co[0] = (self.co[3] + 1) % 3;
        self.co[3] = (self.co[2] + 1) % 3;
        self.co[2] = (self.co[1] + 2) % 3;
        self.co[1] = (a + 2) % 3;
        let a = self.co[4];
        self.co[4] = (self.co[5] + 2) % 3;
        self.co[5] = (self.co[6] + 1) % 3;
        self.co[6] = (self.co[7] + 1) % 3;
        self.co[7] = (a + 2) % 3;
    }

    fn z(&mut self) {
        self.x();
        self.y();
        self.x();
        self.x();
        self.x();
    }

    fn xp(&mut self) {
        self.x();
        self.x();
        self.x();
    }

    fn yp(&mut self) {
        self.y();
        self.y();
        self.y();
    }

    fn zp(&mut self) {
        self.z();
        self.z();
        self.z();
    }

    pub fn l(&mut self) {
        self.z();
        self.u();
        self.zp();
    }

    pub fn f(&mut self) {
        self.x();
        self.u();
        self.xp();
    }

    pub fn r(&mut self) {
        self.zp();
        self.u();
        self.z();
    }

    pub fn b(&mut self) {
        self.xp();
        self.u();
        self.x();
    }

    pub fn d(&mut self) {
        self.x();
        self.x();
        self.u();
        self.x();
        self.x();
    }

    pub fn up(&mut self) {
        self.u();
        self.u();
        self.u();
    }

    pub fn lp(&mut self) {
        self.l();
        self.l();
        self.l();
    }

    pub fn fp(&mut self) {
        self.f();
        self.f();
        self.f();
    }

    pub fn rp(&mut self) {
        self.r();
        self.r();
        self.r();
    }

    pub fn bp(&mut self) {
        self.b();
        self.b();
        self.b();
    }

    pub fn dp(&mut self) {
        self.d();
        self.d();
        self.d();
    }

    fn ep_coord(&self) -> u32 {
        combinatorics::indexing::encode_multiset(self.ep, [3, 6, 3]) as u32
    }

    /// Depends on `self.ep`
    fn eo_coord(&self) -> u32 {
        let mut coord = 0;
        for i in 0..12 {
            if self.ep[i] == 1 {
                coord *= 2;
                coord += self.eo[i] as u32;
            }
        }
        coord
    }

    fn cp_coord(&self) -> u32 {
        combinatorics::indexing::encode_multiset(self.cp, [3, 3, 1, 1]) as u32
    }

    /// Depends on `self.cp`
    fn co_coord(&self) -> u32 {
        let mut coord = 0;
        for i in 0..8 {
            if self.cp[i] <= 1 {
                coord *= 3;
                coord += self.co[i] as u32;
            }
        }
        coord
    }

    fn set_ep_coord(&mut self, coord: u32) {
        self.ep = combinatorics::indexing::decode_multiset(coord as u128, [3, 6, 3]);
    }

    /// Depends on `self.ep`
    fn set_eo_coord(&mut self, mut coord: u32) {
        for i in (0..12).rev() {
            if self.ep[i] == 1 {
                self.eo[i] = (coord % 2) as u8;
                coord /= 2;
            }
        }
    }

    fn set_cp_coord(&mut self, coord: u32) {
        self.cp = combinatorics::indexing::decode_multiset(coord as u128, [3, 3, 1, 1]);
    }

    /// Depends on `self.cp`
    fn set_co_coord(&mut self, mut coord: u32) {
        for i in (0..8).rev() {
            if self.cp[i] <= 1 {
                self.co[i] = (coord % 3) as u8;
                coord /= 3;
            }
        }
    }

    pub fn edges_coord(&self) -> u32 {
        self.ep_coord() * EO_SIZE + self.eo_coord()
    }

    pub fn corners_coord(&self) -> u32 {
        self.cp_coord() * CO_SIZE + self.co_coord()
    }

    pub fn set_edges_coord(&mut self, coord: u32) {
        // Must set EP before EO
        self.set_ep_coord(coord / EO_SIZE);
        self.set_eo_coord(coord % EO_SIZE);
    }

    pub fn set_corners_coord(&mut self, coord: u32) {
        // Must set CP before CO
        self.set_cp_coord(coord / CO_SIZE);
        self.set_co_coord(coord % CO_SIZE);
    }

    pub fn encode(&self) -> u64 {
        self.edges_coord() as u64 * CORNERS_SIZE as u64 + self.corners_coord() as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.set_edges_coord((coord / CORNERS_SIZE as u64) as u32);
        self.set_corners_coord((coord % CORNERS_SIZE as u64) as u32);
    }
}

impl From<CoordCube<'_>> for Cube {
    fn from(value: CoordCube<'_>) -> Self {
        let mut cube = Cube::new();
        cube.set_edges_coord(value.edges);
        cube.set_corners_coord(value.corners);
        cube
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_cube() {
        let mut cube = Cube::new();
        cube.u();
        assert!(!cube.is_solved());
        cube.u();
        assert!(!cube.is_solved());
        cube.d();
        assert!(!cube.is_solved());
        cube.d();
        assert!(!cube.is_solved());
        cube.u();
        cube.u();
        cube.d();
        cube.d();
        cube.l();
        assert!(!cube.is_solved());
        cube.l();
        assert!(!cube.is_solved());
        cube.l();
        assert!(!cube.is_solved());
        cube.l();
        assert!(cube.is_solved());

        // R U R' U' R' F R2 U' R' U' R U R' F'
        for _ in 0..2 {
            cube.r();
            cube.u();
            cube.rp();
            cube.up();
            cube.rp();
            cube.f();
            cube.r();
            cube.r();
            cube.up();
            cube.rp();
            cube.up();
            cube.r();
            cube.u();
            cube.rp();
            cube.fp();
        }
        assert!(cube.is_solved());
    }

    #[test]
    fn test_moves_order() {
        let mut cube = Cube::new();
        for i in 0..4 {
            cube.u();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.x();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.y();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.l();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.f();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.r();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.b();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.d();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.z();
            assert_eq!(cube.is_solved(), i == 3);
        }
    }

    #[test]
    fn test_eo() {
        let mut cube = Cube::new();
        cube.u();
        assert_eq!(cube.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        cube.up();
        cube.l();
        assert_eq!(cube.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        cube.lp();
        cube.f();
        assert_eq!(cube.eo, [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0]);
        cube.fp();
        cube.r();
        assert_eq!(cube.eo, [0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]);
        cube.rp();
        cube.b();
        assert_eq!(cube.eo, [0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0]);
        cube.bp();
        cube.d();
        assert_eq!(cube.eo, [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0]);
    }

    #[test]
    fn test_eo_coord() {
        let mut cube = Cube::new();
        let eo = cube.eo_coord();

        // Flip 2 edges that don't have orientation, and check eo coord
        // R U' R2 U2 R L F R' F' L' U2 R U
        cube.r();
        cube.up();
        cube.r();
        cube.r();
        cube.u();
        cube.u();
        cube.r();
        cube.l();
        cube.f();
        cube.rp();
        cube.fp();
        cube.lp();
        cube.u();
        cube.u();
        cube.r();
        cube.u();

        assert_eq!(cube.eo_coord(), eo);
    }

    #[test]
    fn test_random_scramble() {
        let mut cube = Cube::new();

        // Scramble: D B2 D L2 D U R2 B2 L2 R2 U R B' D' U' F R' U2 B D R2
        cube.d();
        cube.b();
        cube.b();
        cube.d();
        cube.l();
        cube.l();
        cube.d();
        cube.u();
        cube.r();
        cube.r();
        cube.b();
        cube.b();
        cube.l();
        cube.l();
        cube.r();
        cube.r();
        cube.u();
        cube.r();
        cube.bp();
        cube.dp();
        cube.up();
        cube.f();
        cube.rp();
        cube.u();
        cube.u();
        cube.b();
        cube.d();
        cube.r();
        cube.r();

        // Solution: R' B2 R2 U R F' D2 R F R2 L
        cube.rp();
        cube.b();
        cube.b();
        cube.r();
        cube.r();
        cube.u();
        cube.r();
        cube.fp();
        cube.d();
        cube.d();
        cube.r();
        cube.f();
        cube.r();
        cube.r();
        cube.l();

        assert!(cube.is_solved());
    }

    #[test]
    fn test_edges_coord() {
        let mut cube = Cube::new();
        for i in 0..EDGES_SIZE {
            cube.set_edges_coord(i as u32);
            assert_eq!(cube.edges_coord(), i as u32);
        }
    }

    #[test]
    fn test_corners_coord() {
        let mut cube = Cube::new();
        for i in 0..CORNERS_SIZE {
            cube.set_corners_coord(i as u32);
            assert_eq!(cube.corners_coord(), i as u32);
        }
    }

    #[test]
    fn test_encode() {
        let mut x = 0u64;
        let mut cube = Cube::new();
        for _ in 0..65536 {
            x = x.wrapping_mul(450349535401847371);
            x = x.wrapping_add(380506838312516788);
            let coord = x % STATE_SIZE;
            cube.decode(coord);
            assert_eq!(cube.encode(), coord);
        }
    }

    #[test]
    fn test_depth_1() {
        let mut cube = Cube::new();
        let mut arr = [0; 18];
        println!("{cube:?}");
        cube.u();
        println!("u {cube:?}");
        arr[0] = cube.encode();
        cube.u();
        println!("u {cube:?}");
        arr[1] = cube.encode();
        cube.u();
        println!("u {cube:?}");
        arr[2] = cube.encode();
        cube.u();
        println!("u {cube:?}");
        cube.l();
        println!("l {cube:?}");
        arr[3] = cube.encode();
        cube.l();
        println!("l {cube:?}");
        arr[4] = cube.encode();
        cube.l();
        println!("l {cube:?}");
        arr[5] = cube.encode();
        cube.l();
        println!("l {cube:?}");
        cube.f();
        println!("f {cube:?}");
        arr[6] = cube.encode();
        cube.f();
        println!("f {cube:?}");
        arr[7] = cube.encode();
        cube.f();
        println!("f {cube:?}");
        arr[8] = cube.encode();
        cube.f();
        println!("f {cube:?}");
        cube.r();
        println!("r {cube:?}");
        arr[9] = cube.encode();
        cube.r();
        println!("r {cube:?}");
        arr[10] = cube.encode();
        cube.r();
        println!("r {cube:?}");
        arr[11] = cube.encode();
        cube.r();
        println!("r {cube:?}");
        cube.b();
        println!("b {cube:?}");
        arr[12] = cube.encode();
        cube.b();
        println!("b {cube:?}");
        arr[13] = cube.encode();
        cube.b();
        println!("b {cube:?}");
        arr[14] = cube.encode();
        cube.b();
        println!("b {cube:?}");
        cube.d();
        println!("d {cube:?}");
        arr[15] = cube.encode();
        cube.d();
        println!("d {cube:?}");
        arr[16] = cube.encode();
        cube.d();
        println!("d {cube:?}");
        arr[17] = cube.encode();
        cube.d();
        println!("d {cube:?}");

        let set = arr.iter().copied().collect::<HashSet<_>>();
        assert_eq!(set.len(), 18);
    }
}
