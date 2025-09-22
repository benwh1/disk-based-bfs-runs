pub const EP_SIZE: usize = 479001600;

#[derive(Debug, PartialEq)]
pub struct Cube {
    ep: [u8; 12],
}

macro_rules! cycle4 {
    ($arr: expr, $a:expr, $b:expr, $c:expr, $d:expr) => {
        let temp = $arr[$a];
        $arr[$a] = $arr[$d];
        $arr[$d] = $arr[$c];
        $arr[$c] = $arr[$b];
        $arr[$b] = temp;
    };
}

impl Cube {
    pub fn new() -> Self {
        Self {
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        }
    }

    pub fn u(&mut self) {
        cycle4!(self.ep, 0, 1, 2, 3);
    }

    pub fn l(&mut self) {
        cycle4!(self.ep, 1, 4, 9, 5);
    }

    pub fn f(&mut self) {
        cycle4!(self.ep, 0, 7, 10, 4);
    }

    pub fn r(&mut self) {
        cycle4!(self.ep, 3, 6, 11, 7);
    }

    pub fn b(&mut self) {
        cycle4!(self.ep, 2, 5, 8, 6);
    }

    pub fn d(&mut self) {
        cycle4!(self.ep, 8, 9, 10, 11);
    }

    pub fn ep_coord(&self) -> u32 {
        combinatorics::indexing::encode_permutation(self.ep) as u32
    }

    pub fn set_ep_coord(&mut self, coord: u32) {
        self.ep = combinatorics::indexing::decode_permutation(coord as u64);
    }
}
