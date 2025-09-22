use rayon::iter::{
    IndexedParallelIterator as _, IntoParallelRefMutIterator as _, ParallelIterator,
};

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

        macro_rules! par_build_table {
            ($table:ident) => {
                $table.par_iter_mut().enumerate().for_each(|(i, val)| {
                    let mut cube = Cube::new();
                    cube.set_ep_coord(i as u32);
                    cube.$table();
                    *val = cube.ep_coord();
                });
            };
        }

        par_build_table!(u);
        par_build_table!(l);
        par_build_table!(f);
        par_build_table!(r);
        par_build_table!(b);
        par_build_table!(d);

        Self { u, l, f, r, b, d }
    }
}
