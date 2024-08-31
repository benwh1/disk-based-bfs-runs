pub const CP_SIZE: usize = 20160;
pub const CO_SIZE: usize = 2187;
pub const EP_SIZE: usize = 181440;
pub const CORNERS_SIZE: usize = CP_SIZE * CO_SIZE;

#[derive(Debug, PartialEq)]
pub struct Megaminx {
    cp: [u8; 8],
    co: [u8; 8],
    ep: [u8; 9],
}

impl Megaminx {
    pub fn new() -> Self {
        Self {
            cp: [0, 1, 2, 3, 4, 5, 6, 7],
            co: [0; 8],
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8],
        }
    }

    pub fn is_solved(&self) -> bool {
        self == &Self::new()
    }

    pub fn u(&mut self) {
        let a = self.cp[0];
        self.cp[0] = self.cp[4];
        self.cp[4] = self.cp[3];
        self.cp[3] = self.cp[2];
        self.cp[2] = self.cp[1];
        self.cp[1] = a;
        let a = self.co[0];
        self.co[0] = self.co[4];
        self.co[4] = self.co[3];
        self.co[3] = self.co[2];
        self.co[2] = self.co[1];
        self.co[1] = a;
        let a = self.ep[0];
        self.ep[0] = self.ep[4];
        self.ep[4] = self.ep[3];
        self.ep[3] = self.ep[2];
        self.ep[2] = self.ep[1];
        self.ep[1] = a;
    }

    pub fn r(&mut self) {
        let a = self.cp[3];
        self.cp[3] = self.cp[4];
        self.cp[4] = self.cp[5];
        self.cp[5] = self.cp[6];
        self.cp[6] = self.cp[7];
        self.cp[7] = a;
        let a = self.co[3];
        self.co[3] = (self.co[4] + 1) % 3;
        self.co[4] = (self.co[5] + 2) % 3;
        self.co[5] = (self.co[6] + 2) % 3;
        self.co[6] = (self.co[7] + 2) % 3;
        self.co[7] = (a + 2) % 3;
        let a = self.ep[4];
        self.ep[4] = self.ep[5];
        self.ep[5] = self.ep[6];
        self.ep[6] = self.ep[7];
        self.ep[7] = self.ep[8];
        self.ep[8] = a;
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

    pub fn cp_coord(&self) -> u32 {
        combinatorics::indexing::encode_even_permutation(self.cp) as u32
    }

    pub fn co_coord(&self) -> u32 {
        self.co.iter().take(7).fold(0, |acc, &x| acc * 3 + x as u32)
    }

    pub fn ep_coord(&self) -> u32 {
        combinatorics::indexing::encode_even_permutation(self.ep) as u32
    }

    pub fn set_cp_coord(&mut self, coord: u32) {
        self.cp = combinatorics::indexing::decode_even_permutation(coord as u64);
    }

    pub fn set_co_coord(&mut self, mut coord: u32) {
        let mut total = 0;
        for i in (0..7).rev() {
            self.co[i] = (coord % 3) as u8;
            total += self.co[i];
            coord /= 3;
        }
        self.co[7] = (3 - total % 3) as u8;
    }

    pub fn set_ep_coord(&mut self, coord: u32) {
        self.ep = combinatorics::indexing::decode_even_permutation(coord as u64);
    }

    pub fn corners_coord(&self) -> u32 {
        self.cp_coord() * CO_SIZE as u32 + self.co_coord()
    }

    pub fn set_corners_coord(&mut self, coord: u32) {
        self.set_cp_coord(coord / CO_SIZE as u32);
        self.set_co_coord(coord % CO_SIZE as u32);
    }

    pub fn encode(&self) -> u32 {
        self.corners_coord() * EP_SIZE as u32 + self.ep_coord()
    }

    pub fn decode(&mut self, coord: u32) {
        self.set_corners_coord(coord / EP_SIZE as u32);
        self.set_ep_coord(coord % EP_SIZE as u32);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_order() {
        let mut minx = Megaminx::new();
        for i in 0..5 {
            minx.u();
            assert_eq!(minx.is_solved(), i == 4);
        }
        for i in 0..5 {
            minx.r();
            assert_eq!(minx.is_solved(), i == 4);
        }
    }

    #[test]
    fn test_coords() {
        let mut minx = Megaminx::new();

        for i in 0..CORNERS_SIZE as u32 {
            minx.set_corners_coord(i);
            assert_eq!(minx.corners_coord(), i);
        }

        for i in 0..EP_SIZE as u32 {
            minx.set_ep_coord(i);
            assert_eq!(minx.ep_coord(), i);
        }
    }

    #[test]
    fn test_random_scramble() {
        let scramble = "R2 U' R' U2' R' U R' U2' R' U R U2 R' U2' R U2' R U R2' U2' R' U2' R2 U2' \
        R2' U R2 U' R2' U2 R' U R' U2' R U2' R2 U2 R2 U2' R U2' R' U2' R2' U R2 U2 R' U2'";

        let mut minx = Megaminx::new();
        minx.do_alg(scramble);

        assert_eq!(
            minx,
            Megaminx {
                cp: [6, 7, 0, 5, 2, 1, 4, 3],
                co: [1, 2, 2, 0, 1, 2, 2, 2],
                ep: [8, 0, 5, 4, 3, 7, 2, 6, 1],
            }
        );

        let solution = "U' R' U R2 U' R U' R2' U' R' U2' R2' U' R' U2' R' U R2 U' R' U'";

        minx.do_alg(solution);

        assert_eq!(minx.is_solved(), true);
    }
}
