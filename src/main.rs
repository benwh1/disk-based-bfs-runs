const EP_SIZE: u32 = 181440;
const CP_SIZE: u32 = 5040;
const CO_SIZE: u32 = 729;
const CORNERS_SIZE: u32 = CP_SIZE * CO_SIZE;
const STATE_SIZE: u64 = EP_SIZE as u64 * CORNERS_SIZE as u64;

#[derive(Debug)]
struct Cube {
    ep: [u8; 9],
    cp: [u8; 7],
    co: [u8; 7],
    is_even_perm: bool,
}

impl Cube {
    fn new() -> Cube {
        Cube {
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            cp: [0, 1, 2, 3, 4, 5, 6],
            co: [0; 7],
            is_even_perm: true,
        }
    }

    fn u(&mut self) {
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

    fn r(&mut self) {
        let a = self.ep[3];
        self.ep[3] = self.ep[4];
        self.ep[4] = self.ep[5];
        self.ep[5] = self.ep[6];
        self.ep[6] = a;
        let a = self.cp[3];
        self.cp[3] = self.cp[4];
        self.cp[4] = self.cp[5];
        self.cp[5] = self.cp[6];
        self.cp[6] = a;
        let a = self.co[3];
        self.co[3] = (self.co[4] + 1) % 3;
        self.co[4] = (self.co[5] + 2) % 3;
        self.co[5] = (self.co[6] + 1) % 3;
        self.co[6] = (a + 2) % 3;
        self.is_even_perm = !self.is_even_perm;
    }

    fn f2(&mut self) {
        self.ep.swap(0, 7);
        self.ep.swap(4, 8);
        self.cp.swap(0, 4);
        self.cp.swap(3, 6);
        self.co.swap(0, 4);
        self.co.swap(3, 6);
    }

    fn ep_coord(&self) -> u32 {
        combinatorics::indexing::encode_even_permutation(self.ep) as u32
    }

    fn cp_coord(&self) -> u32 {
        combinatorics::indexing::encode_permutation(self.cp) as u32
    }

    fn co_coord(&self) -> u32 {
        self.co.iter().take(6).fold(0, |acc, &x| 3 * acc + x as u32)
    }

    fn set_ep_coord(&mut self, coord: u32) {
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

    fn corners_coord(&self) -> u32 {
        self.cp_coord() * CO_SIZE + self.co_coord()
    }

    fn set_corners_coord(&mut self, coord: u32) {
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

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
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
}
