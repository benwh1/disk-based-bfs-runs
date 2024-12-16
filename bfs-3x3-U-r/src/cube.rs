#![allow(dead_code)]

use crate::coord_cube::CoordCube;

static CP_ALL: [[u8; 6]; 120] = [
    [0, 1, 2, 3, 4, 5],
    [0, 1, 3, 4, 5, 2],
    [0, 1, 4, 5, 2, 3],
    [0, 1, 5, 2, 3, 4],
    [0, 2, 1, 4, 3, 5],
    [0, 2, 3, 5, 1, 4],
    [0, 2, 4, 3, 5, 1],
    [0, 2, 5, 1, 4, 3],
    [0, 3, 1, 5, 4, 2],
    [0, 3, 2, 1, 5, 4],
    [0, 3, 4, 2, 1, 5],
    [0, 3, 5, 4, 2, 1],
    [0, 4, 1, 2, 5, 3],
    [0, 4, 2, 5, 3, 1],
    [0, 4, 3, 1, 2, 5],
    [0, 4, 5, 3, 1, 2],
    [0, 5, 1, 3, 2, 4],
    [0, 5, 2, 4, 1, 3],
    [0, 5, 3, 2, 4, 1],
    [0, 5, 4, 1, 3, 2],
    [1, 0, 2, 5, 4, 3],
    [1, 0, 3, 2, 5, 4],
    [1, 0, 4, 3, 2, 5],
    [1, 0, 5, 4, 3, 2],
    [1, 2, 0, 4, 5, 3],
    [1, 2, 3, 0, 4, 5],
    [1, 2, 4, 5, 3, 0],
    [1, 2, 5, 3, 0, 4],
    [1, 3, 0, 5, 2, 4],
    [1, 3, 2, 4, 0, 5],
    [1, 3, 4, 0, 5, 2],
    [1, 3, 5, 2, 4, 0],
    [1, 4, 0, 2, 3, 5],
    [1, 4, 2, 3, 5, 0],
    [1, 4, 3, 5, 0, 2],
    [1, 4, 5, 0, 2, 3],
    [1, 5, 0, 3, 4, 2],
    [1, 5, 2, 0, 3, 4],
    [1, 5, 3, 4, 2, 0],
    [1, 5, 4, 2, 0, 3],
    [2, 0, 1, 5, 3, 4],
    [2, 0, 3, 4, 1, 5],
    [2, 0, 4, 1, 5, 3],
    [2, 0, 5, 3, 4, 1],
    [2, 1, 0, 3, 5, 4],
    [2, 1, 3, 5, 4, 0],
    [2, 1, 4, 0, 3, 5],
    [2, 1, 5, 4, 0, 3],
    [2, 3, 0, 1, 4, 5],
    [2, 3, 1, 4, 5, 0],
    [2, 3, 4, 5, 0, 1],
    [2, 3, 5, 0, 1, 4],
    [2, 4, 0, 5, 1, 3],
    [2, 4, 1, 3, 0, 5],
    [2, 4, 3, 0, 5, 1],
    [2, 4, 5, 1, 3, 0],
    [2, 5, 0, 4, 3, 1],
    [2, 5, 1, 0, 4, 3],
    [2, 5, 3, 1, 0, 4],
    [2, 5, 4, 3, 1, 0],
    [3, 0, 1, 2, 4, 5],
    [3, 0, 2, 4, 5, 1],
    [3, 0, 4, 5, 1, 2],
    [3, 0, 5, 1, 2, 4],
    [3, 1, 0, 4, 2, 5],
    [3, 1, 2, 5, 0, 4],
    [3, 1, 4, 2, 5, 0],
    [3, 1, 5, 0, 4, 2],
    [3, 2, 0, 5, 4, 1],
    [3, 2, 1, 0, 5, 4],
    [3, 2, 4, 1, 0, 5],
    [3, 2, 5, 4, 1, 0],
    [3, 4, 0, 1, 5, 2],
    [3, 4, 1, 5, 2, 0],
    [3, 4, 2, 0, 1, 5],
    [3, 4, 5, 2, 0, 1],
    [3, 5, 0, 2, 1, 4],
    [3, 5, 1, 4, 0, 2],
    [3, 5, 2, 1, 4, 0],
    [3, 5, 4, 0, 2, 1],
    [4, 0, 1, 3, 5, 2],
    [4, 0, 2, 1, 3, 5],
    [4, 0, 3, 5, 2, 1],
    [4, 0, 5, 2, 1, 3],
    [4, 1, 0, 5, 3, 2],
    [4, 1, 2, 0, 5, 3],
    [4, 1, 3, 2, 0, 5],
    [4, 1, 5, 3, 2, 0],
    [4, 2, 0, 3, 1, 5],
    [4, 2, 1, 5, 0, 3],
    [4, 2, 3, 1, 5, 0],
    [4, 2, 5, 0, 3, 1],
    [4, 3, 0, 2, 5, 1],
    [4, 3, 1, 0, 2, 5],
    [4, 3, 2, 5, 1, 0],
    [4, 3, 5, 1, 0, 2],
    [4, 5, 0, 1, 2, 3],
    [4, 5, 1, 2, 3, 0],
    [4, 5, 2, 3, 0, 1],
    [4, 5, 3, 0, 1, 2],
    [5, 0, 1, 4, 2, 3],
    [5, 0, 2, 3, 1, 4],
    [5, 0, 3, 1, 4, 2],
    [5, 0, 4, 2, 3, 1],
    [5, 1, 0, 2, 4, 3],
    [5, 1, 2, 4, 3, 0],
    [5, 1, 3, 0, 2, 4],
    [5, 1, 4, 3, 0, 2],
    [5, 2, 0, 1, 3, 4],
    [5, 2, 1, 3, 4, 0],
    [5, 2, 3, 4, 0, 1],
    [5, 2, 4, 0, 1, 3],
    [5, 3, 0, 4, 1, 2],
    [5, 3, 1, 2, 0, 4],
    [5, 3, 2, 0, 4, 1],
    [5, 3, 4, 1, 2, 0],
    [5, 4, 0, 3, 2, 1],
    [5, 4, 1, 0, 3, 2],
    [5, 4, 2, 1, 0, 3],
    [5, 4, 3, 2, 1, 0],
];

/// Corners: UFL ULB UBR URF DFR DRB
/// Edges: UF UL UB UR FR BR DF DR DB
#[derive(Debug, PartialEq)]
pub struct Cube {
    cp: [u8; 6],
    co: [u8; 6],
    ep: [u8; 9],
    eo: [u8; 9],
    centers: u8,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            cp: [0, 1, 2, 3, 4, 5],
            co: [0; 6],
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            eo: [0; 9],
            centers: 0,
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == Cube::new()
    }

    pub fn u(&mut self) {
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

    pub fn r(&mut self) {
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
        let a = self.ep[3];
        self.ep[3] = self.ep[4];
        self.ep[4] = self.ep[7];
        self.ep[7] = self.ep[5];
        self.ep[5] = a;
        let a = self.eo[3];
        self.eo[3] = self.eo[4];
        self.eo[4] = self.eo[7];
        self.eo[7] = self.eo[5];
        self.eo[5] = a;
    }

    pub fn r2(&mut self) {
        self.r();
        self.r();
    }

    pub fn r_inv(&mut self) {
        self.r();
        self.r();
        self.r();
    }

    pub fn m(&mut self) {
        let a = self.ep[0];
        self.ep[0] = self.ep[2];
        self.ep[2] = self.ep[8];
        self.ep[8] = self.ep[6];
        self.ep[6] = a;
        let a = self.eo[0];
        self.eo[0] = (self.eo[2] + 1) % 2;
        self.eo[2] = (self.eo[8] + 1) % 2;
        self.eo[8] = (self.eo[6] + 1) % 2;
        self.eo[6] = (a + 1) % 2;
        self.centers = (self.centers + 1) % 4;
    }

    pub fn m2(&mut self) {
        self.m();
        self.m();
    }

    pub fn m_inv(&mut self) {
        self.m();
        self.m();
        self.m();
    }

    pub fn rw(&mut self) {
        self.r();
        self.m_inv();
    }

    pub fn rw2(&mut self) {
        self.r2();
        self.m2();
    }

    pub fn rw_inv(&mut self) {
        self.r_inv();
        self.m();
    }

    fn do_move(&mut self, mv: &str) {
        match mv {
            "U" => self.u(),
            "U2" => self.u2(),
            "U'" => self.u_inv(),
            "R" => self.r(),
            "R2" => self.r2(),
            "R'" => self.r_inv(),
            "M" => self.m(),
            "M2" => self.m2(),
            "M'" => self.m_inv(),
            "r" => self.rw(),
            "r2" => self.rw2(),
            "r'" => self.rw_inv(),
            _ => panic!("Invalid move {mv}"),
        }
    }

    fn cp_coord(&self) -> u32 {
        CP_ALL.iter().position(|a| a == &self.cp).unwrap() as u32
    }

    fn co_coord(&self) -> u32 {
        self.co.iter().take(5).fold(0, |acc, &x| acc * 3 + x as u32)
    }

    fn ep_coord(&self) -> u32 {
        combinatorics::indexing::encode_permutation(self.ep) as u32
    }

    fn eo_coord(&self) -> u32 {
        self.eo.iter().take(8).fold(0, |acc, &x| acc * 2 + x as u32)
    }

    pub fn perm_coord(&self) -> u32 {
        self.cp_coord() * 362880 * 2 + self.ep_coord() * 2 + self.centers as u32 / 2
    }

    pub fn ori_coord(&self) -> u32 {
        self.co_coord() * 256 + self.eo_coord()
    }

    fn set_cp_coord(&mut self, coord: u32) {
        self.cp = CP_ALL[coord as usize];
    }

    fn set_co_coord(&mut self, coord: u32) {
        let mut coord = coord;
        let mut total = 0;
        for i in (0..5).rev() {
            self.co[i] = (coord % 3) as u8;
            total += self.co[i];
            coord /= 3;
        }
        self.co[5] = (15 - total) % 3;
    }

    fn set_ep_coord(&mut self, coord: u32) {
        self.ep = combinatorics::indexing::decode_permutation(coord as u64);
    }

    fn set_eo_coord(&mut self, coord: u32) {
        let mut coord = coord;
        let mut total = 0;
        for i in (0..8).rev() {
            self.eo[i] = (coord % 2) as u8;
            total += self.eo[i];
            coord /= 2;
        }
        self.eo[8] = total % 2;
    }

    pub fn set_perm_coord(&mut self, coord: u32) {
        // There is a 2 dimensional parity space of parity constraints.
        // For all solvable states, the sum of the parities of corners, edges, centers must be even
        // i.e. the (corner, edge, center) parity must be one of (0,0,0), (1,1,0), (1,0,1), (0,1,1)
        // We store the EP and CP states completely, so their parities are known.
        // We only store one bit of information about the centers, so we have to use the parity of
        // the edges and corners to determine the other bit.

        let centers_half = coord % 2;
        let ep_coord = (coord / 2) % 362880;
        let cp_coord = (coord / 2) / 362880;

        self.set_cp_coord(cp_coord);
        self.set_ep_coord(ep_coord);

        let cp_parity = combinatorics::sign::parity(self.cp);
        let ep_parity = combinatorics::sign::parity(self.ep);

        let centers_parity = (cp_parity + ep_parity) % 2;

        let centers = centers_half as u8 * 2 + centers_parity;
        self.centers = centers;
    }

    pub fn set_ori_coord(&mut self, coord: u32) {
        self.set_eo_coord(coord % 256);
        self.set_co_coord(coord / 256);
    }

    pub fn encode(&self) -> u64 {
        self.perm_coord() as u64 * 62208 + self.ori_coord() as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.set_perm_coord((coord / 62208) as u32);
        self.set_ori_coord((coord % 62208) as u32);
    }

    pub fn to_ksolve_string(&self) -> String {
        let mut s = format!("corners\n");
        for c in self.cp {
            s.push_str(&(c + 1).to_string());
            s.push(' ');
        }
        s.pop();
        s.push('\n');
        for c in self.co {
            s.push_str(&c.to_string());
            s.push(' ');
        }
        s.pop();
        s.push_str("\nedges\n");
        for e in self.ep {
            s.push_str(&(e + 1).to_string());
            s.push(' ');
        }
        s.pop();
        s.push('\n');
        for e in self.eo {
            s.push_str(&e.to_string());
            s.push(' ');
        }
        s.pop();
        s.push_str("\ncenters\n1\n");
        s.push_str(&self.centers.to_string());
        s
    }
}

impl From<CoordCube<'_>> for Cube {
    fn from(value: CoordCube<'_>) -> Self {
        let mut cube = Cube::new();
        cube.set_perm_coord(value.perm);
        cube.set_ori_coord(value.ori);
        cube
    }
}

#[cfg(test)]
mod tests {
    use crate::transposition_tables::TranspositionTables;

    use super::*;

    #[test]
    fn test_move_order() {
        let mut cube = Cube::new();
        for i in 0..4 {
            cube.u();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.r();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.m();
            assert_eq!(cube.is_solved(), i == 3);
        }
        for i in 0..4 {
            cube.rw();
            assert_eq!(cube.is_solved(), i == 3);
        }
    }

    #[test]
    fn test_perm_coord() {
        let mut cube = Cube::new();

        for i in 0..87091200 {
            cube.set_perm_coord(i);
            assert_eq!(cube.perm_coord(), i);
        }

        for i in 0..62208 {
            cube.set_ori_coord(i);
            assert_eq!(cube.ori_coord(), i);
        }
    }

    #[test]
    fn test_cube_random_scramble() {
        let scramble = "r' U' r U' r U' r' U' r' U' r2 U2 r U' r U2 r U' r' U r2 U2 r2 U' r2 \
        U' r2 U2 r' U r2 U' r U' r U' r' U' r2 U2 r' U r2 U' r U' r2 U r U";

        let transposition_tables = TranspositionTables::new();

        let mut cube = Cube::new();
        let mut coord_cube = CoordCube::new(&transposition_tables);

        for (i, mv) in scramble.split_whitespace().enumerate() {
            cube.do_move(mv);
            coord_cube.do_move(mv);

            assert_eq!(
                cube.perm_coord(),
                coord_cube.perm,
                "perm differs after move {i} = {mv}"
            );
            assert_eq!(
                cube.ori_coord(),
                coord_cube.ori,
                "ori differs after move {i} = {mv}"
            );
            assert_eq!(
                cube,
                Cube::from(coord_cube.clone()),
                "cube differs after move {i} = {mv}"
            );
        }

        assert_eq!(
            cube,
            Cube {
                cp: [1, 4, 0, 2, 3, 5],
                co: [0, 2, 1, 1, 1, 1],
                ep: [8, 6, 4, 0, 7, 2, 5, 3, 1],
                eo: [0, 0, 0, 1, 0, 1, 1, 1, 0],
                centers: 0,
            },
        );

        let solution = "U' r' U' r2 U r2 U r2 U' r' U r' U r' U2 r' U2 r U2 r2 U r U' r' U'";

        for mv in solution.split_whitespace() {
            cube.do_move(mv);
            coord_cube.do_move(mv);

            assert_eq!(cube.perm_coord(), coord_cube.perm);
            assert_eq!(cube.ori_coord(), coord_cube.ori);
            assert_eq!(cube, Cube::from(coord_cube.clone()));
        }

        assert_eq!(cube, Cube::new());
    }
}
