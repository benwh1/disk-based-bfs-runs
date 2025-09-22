#[derive(Debug, Clone, PartialEq)]
pub struct Kilominx {
    corners: [u8; 15],
}

impl Kilominx {
    pub fn new() -> Self {
        Self {
            corners: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
        }
    }

    pub fn is_solved(&self) -> bool {
        self.corners == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
    }

    pub fn u(&mut self) {
        let a = self.corners[0];
        self.corners[0] = self.corners[4];
        self.corners[4] = self.corners[3];
        self.corners[3] = self.corners[2];
        self.corners[2] = self.corners[1];
        self.corners[1] = a;
    }

    pub fn l(&mut self) {
        let a = self.corners[0];
        self.corners[0] = self.corners[1];
        self.corners[1] = self.corners[6];
        self.corners[6] = self.corners[11];
        self.corners[11] = self.corners[5];
        self.corners[5] = a;
    }

    pub fn f(&mut self) {
        let a = self.corners[4];
        self.corners[4] = self.corners[0];
        self.corners[0] = self.corners[5];
        self.corners[5] = self.corners[12];
        self.corners[12] = self.corners[9];
        self.corners[9] = a;
    }

    pub fn r(&mut self) {
        let a = self.corners[3];
        self.corners[3] = self.corners[4];
        self.corners[4] = self.corners[9];
        self.corners[9] = self.corners[13];
        self.corners[13] = self.corners[8];
        self.corners[8] = a;
    }

    pub fn br(&mut self) {
        let a = self.corners[2];
        self.corners[2] = self.corners[3];
        self.corners[3] = self.corners[8];
        self.corners[8] = self.corners[14];
        self.corners[14] = self.corners[7];
        self.corners[7] = a;
    }

    pub fn bl(&mut self) {
        let a = self.corners[1];
        self.corners[1] = self.corners[2];
        self.corners[2] = self.corners[7];
        self.corners[7] = self.corners[10];
        self.corners[10] = self.corners[6];
        self.corners[6] = a;
    }

    pub fn do_move(&mut self, mv: &str) {
        match mv {
            "U" => self.u(),
            "L" => self.l(),
            "F" => self.f(),
            "R" => self.r(),
            "BR" => self.br(),
            "BL" => self.bl(),
            _ => panic!("Invalid move"),
        }
    }

    pub fn do_alg(&mut self, alg: &str) {
        alg.split_whitespace().for_each(|mv| self.do_move(mv));
    }

    pub fn encode(&self) -> u64 {
        combinatorics::indexing::encode_even_permutation(self.corners)
    }

    pub fn decode(&mut self, coord: u64) {
        self.corners = combinatorics::indexing::decode_even_permutation(coord);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_order() {
        let mut minx = Kilominx::new();
        for i in 0..5 {
            minx.u();
            assert_eq!(minx.is_solved(), i == 4);
        }
        for i in 0..5 {
            minx.l();
            assert_eq!(minx.is_solved(), i == 4);
        }
        for i in 0..5 {
            minx.f();
            assert_eq!(minx.is_solved(), i == 4);
        }
        for i in 0..5 {
            minx.r();
            assert_eq!(minx.is_solved(), i == 4);
        }
        for i in 0..5 {
            minx.br();
            assert_eq!(minx.is_solved(), i == 4);
        }
        for i in 0..5 {
            minx.bl();
            assert_eq!(minx.is_solved(), i == 4);
        }
    }

    #[test]
    fn test_t_perm() {
        let scramble =
            "R U R R R R U U U U R R R R F R R U U U U R R R R U U U U R U R R R R F F F F U";

        let mut minx = Kilominx::new();
        minx.do_alg(scramble);

        assert_eq!(
            minx.corners,
            [1, 0, 3, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
        );
    }
}
