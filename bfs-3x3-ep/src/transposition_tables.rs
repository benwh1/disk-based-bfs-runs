use crate::cube::{Cube, EP_SIZE};

#[derive(Debug, PartialEq)]
pub struct TranspositionTables {
    pub u: Vec<u32>,
    pub l: Vec<u32>,
    pub f: Vec<u32>,
    pub r: Vec<u32>,
    pub b: Vec<u32>,
    pub d: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u = vec![0; EP_SIZE];
        let mut l = vec![0; EP_SIZE];
        let mut f = vec![0; EP_SIZE];
        let mut r = vec![0; EP_SIZE];
        let mut b = vec![0; EP_SIZE];
        let mut d = vec![0; EP_SIZE];

        let mut cube = Cube::new();

        for i in 0..EP_SIZE {
            cube.set_ep_coord(i as u32);
            cube.u();
            u[i] = cube.ep_coord();
            cube.up();
            cube.r();
            r[i] = cube.ep_coord();
            cube.rp();
            cube.f();
            f[i] = cube.ep_coord();
            cube.fp();
            cube.l();
            l[i] = cube.ep_coord();
            cube.lp();
            cube.b();
            b[i] = cube.ep_coord();
            cube.bp();
            cube.d();
            d[i] = cube.ep_coord();
        }

        Self { u, l, f, r, b, d }
    }
}
