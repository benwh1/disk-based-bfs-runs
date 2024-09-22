use crate::cube::Cube;

pub struct TranspositionTables {
    pub u_perm: Vec<u32>,
    pub u_ori: Vec<u32>,
    pub rw_perm: Vec<u32>,
    pub rw_ori: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_perm = vec![0; 87091200];
        let mut u_ori = vec![0; 62208];
        let mut rw_perm = vec![0; 87091200];
        let mut rw_ori = vec![0; 62208];

        let mut cube = Cube::new();

        for i in 0..87091200 {
            cube.set_perm_coord(i);
            let i = i as usize;
            cube.u();
            u_perm[i] = cube.perm_coord();
            cube.u_inv();
            cube.rw();
            rw_perm[i] = cube.perm_coord();
        }

        for i in 0..62208 {
            cube.set_ori_coord(i);
            let i = i as usize;
            cube.u();
            u_ori[i] = cube.ori_coord();
            cube.u_inv();
            cube.rw();
            rw_ori[i] = cube.ori_coord();
        }

        Self {
            u_perm,
            u_ori,
            rw_perm,
            rw_ori,
        }
    }
}
