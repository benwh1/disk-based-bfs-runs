use crate::cube::Cube;

pub struct TranspositionTables {
    pub u_perm: Vec<u32>,
    pub u_ori: Vec<u32>,
    pub u2_perm: Vec<u32>,
    pub u2_ori: Vec<u32>,
    pub ur_perm: Vec<u32>,
    pub ur_ori: Vec<u32>,
    pub r_perm: Vec<u32>,
    pub r_ori: Vec<u32>,
    pub r2_perm: Vec<u32>,
    pub r2_ori: Vec<u32>,
    pub rw_perm: Vec<u32>,
    pub rw_ori: Vec<u32>,
    pub rw2_perm: Vec<u32>,
    pub rw2_ori: Vec<u32>,
    pub m_perm: Vec<u32>,
    pub m_ori: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_perm = vec![0; 87091200];
        let mut u_ori = vec![0; 62208];
        let mut u2_perm = vec![0; 87091200];
        let mut u2_ori = vec![0; 62208];
        let mut ur_perm = vec![0; 87091200];
        let mut ur_ori = vec![0; 62208];
        let mut r_perm = vec![0; 87091200];
        let mut r_ori = vec![0; 62208];
        let mut r2_perm = vec![0; 87091200];
        let mut r2_ori = vec![0; 62208];
        let mut rw_perm = vec![0; 87091200];
        let mut rw_ori = vec![0; 62208];
        let mut rw2_perm = vec![0; 87091200];
        let mut rw2_ori = vec![0; 62208];
        let mut m_perm = vec![0; 87091200];
        let mut m_ori = vec![0; 62208];

        let mut cube = Cube::new();

        for i in 0..87091200 {
            cube.set_perm_coord(i);
            let i = i as usize;
            cube.u();
            u_perm[i] = cube.perm_coord();
            cube.u();
            u2_perm[i] = cube.perm_coord();
            cube.u_inv();
            cube.r();
            ur_perm[i] = cube.perm_coord();
            cube.r_inv();
            cube.u_inv();
            cube.r();
            r_perm[i] = cube.perm_coord();
            cube.r();
            r2_perm[i] = cube.perm_coord();
            cube.r2();
            cube.rw();
            rw_perm[i] = cube.perm_coord();
            cube.rw();
            rw2_perm[i] = cube.perm_coord();
            cube.rw2();
            cube.m();
            m_perm[i] = cube.perm_coord();
        }

        for i in 0..62208 {
            cube.set_ori_coord(i);
            let i = i as usize;
            cube.u();
            u_ori[i] = cube.ori_coord();
            cube.u();
            u2_ori[i] = cube.ori_coord();
            cube.u_inv();
            cube.r();
            ur_ori[i] = cube.ori_coord();
            cube.r_inv();
            cube.u_inv();
            cube.r();
            r_ori[i] = cube.ori_coord();
            cube.r();
            r2_ori[i] = cube.ori_coord();
            cube.r2();
            cube.rw();
            rw_ori[i] = cube.ori_coord();
            cube.rw();
            rw2_ori[i] = cube.ori_coord();
            cube.rw2();
            cube.m();
            m_ori[i] = cube.ori_coord();
        }

        Self {
            u_perm,
            u_ori,
            u2_perm,
            u2_ori,
            ur_perm,
            ur_ori,
            r_perm,
            r_ori,
            r2_perm,
            r2_ori,
            rw_perm,
            rw_ori,
            rw2_perm,
            rw2_ori,
            m_perm,
            m_ori,
        }
    }
}
