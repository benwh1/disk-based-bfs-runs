use std::path::PathBuf;

use disk_based_bfs::{callback::BfsCallback, one_bit::BfsBuilder};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

struct Cube {
    cp: [u8; 6],
    co: [u8; 6],
    ep: [u8; 9],
    eo: [u8; 9],
    centers: u8,
}

impl Cube {
    fn new() -> Self {
        Self {
            cp: [0, 1, 2, 3, 4, 5],
            co: [0; 6],
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            eo: [0; 9],
            centers: 0,
        }
    }

    fn u(&mut self) {
        self.ep[0..4].rotate_right(1);
        self.cp[0..4].rotate_right(1);
    }

    fn u_inv(&mut self) {
        self.ep[0..4].rotate_left(1);
        self.cp[0..4].rotate_left(1);
    }

    fn r(&mut self) {
        self.cp[2..6].rotate_left(1);
        self.ep[3..7].rotate_left(1);
        for i in 2..6 {
            self.co[self.cp[i] as usize] = (self.co[self.cp[i] as usize] + 1 + i as u8 % 2) % 3;
        }
    }

    fn r_inv(&mut self) {
        self.cp[2..6].rotate_right(1);
        self.ep[3..7].rotate_right(1);
        for i in 2..6 {
            self.co[self.cp[i] as usize] = (self.co[self.cp[i] as usize] + 1 + i as u8 % 2) % 3;
        }
    }

    fn m(&mut self) {
        let x = self.ep[0];
        self.ep[0] = self.ep[2];
        self.ep[2] = self.ep[8];
        self.ep[8] = self.ep[7];
        self.ep[7] = x;
        for i in [0, 2, 7, 8] {
            self.eo[self.ep[i] as usize] = (self.eo[self.ep[i] as usize] + 1) % 2;
        }
        self.centers = (self.centers + 3) % 4;
    }

    fn m_inv(&mut self) {
        let x = self.ep[0];
        self.ep[0] = self.ep[7];
        self.ep[7] = self.ep[8];
        self.ep[8] = self.ep[2];
        self.ep[2] = x;
        for i in [0, 2, 7, 8] {
            self.eo[self.ep[i] as usize] = (self.eo[self.ep[i] as usize] + 1) % 2;
        }
        self.centers = (self.centers + 1) % 4;
    }

    fn rw(&mut self) {
        self.r();
        self.m_inv();
    }

    fn rw_inv(&mut self) {
        self.r_inv();
        self.m();
    }

    fn cp_coord(&self) -> u32 {
        // Lazy
        let all = include!("cp_all.rs");
        all.iter().position(|a| a == &self.cp).unwrap() as u32
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

    fn perm_coord(&self) -> u32 {
        self.cp_coord() * 362880 * 2 + self.ep_coord() * 2 + self.centers as u32
    }

    fn ori_coord(&self) -> u32 {
        self.co_coord() * 256 + self.eo_coord()
    }

    fn set_cp_coord(&mut self, coord: u32) {
        let all = include!("cp_all.rs");
        self.cp = all[coord as usize];
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

    fn set_perm_coord(&mut self, coord: u32) {
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

    fn set_ori_coord(&mut self, coord: u32) {
        self.set_eo_coord(coord % 256);
        self.set_co_coord(coord / 256);
    }
}

struct TranspositionTables {
    u_perm: Vec<u32>,
    u_ori: Vec<u32>,
    urw_perm: Vec<u32>,
    urw_ori: Vec<u32>,
    rw_perm: Vec<u32>,
    rw_ori: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_perm = vec![0; 87091200];
        let mut u_ori = vec![0; 62208];
        let mut urw_perm = vec![0; 87091200];
        let mut urw_ori = vec![0; 62208];
        let mut rw_perm = vec![0; 87091200];
        let mut rw_ori = vec![0; 62208];

        let mut cube = Cube::new();

        for i in 0..87091200 {
            cube.set_perm_coord(i);
            let i = i as usize;
            cube.rw();
            rw_perm[i] = cube.perm_coord();
            cube.rw_inv();
            cube.u();
            u_perm[i] = cube.perm_coord();
            cube.rw();
            urw_perm[i] = cube.perm_coord();
        }

        for i in 0..62208 {
            cube.set_ori_coord(i);
            let i = i as usize;
            cube.rw();
            rw_ori[i] = cube.ori_coord();
            cube.rw_inv();
            cube.u();
            u_ori[i] = cube.ori_coord();
            cube.rw();
            urw_ori[i] = cube.ori_coord();
        }

        Self {
            u_perm,
            u_ori,
            urw_perm,
            urw_ori,
            rw_perm,
            rw_ori,
        }
    }
}

#[derive(Clone)]
struct CoordCube<'a> {
    perm: u32,
    ori: u32,
    transposition_tables: &'a TranspositionTables,
}

impl<'a> std::fmt::Debug for CoordCube<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoordCube")
            .field("perm", &self.perm)
            .field("ori", &self.ori)
            .finish()
    }
}

impl<'a> CoordCube<'a> {
    pub fn new(transposition_tables: &'a TranspositionTables) -> Self {
        let cube = Cube::new();
        Self {
            perm: cube.perm_coord(),
            ori: cube.ori_coord(),
            transposition_tables,
        }
    }

    pub fn u(&mut self) {
        self.perm = self.transposition_tables.u_perm[self.perm as usize];
        self.ori = self.transposition_tables.u_ori[self.ori as usize];
    }

    pub fn urw(&mut self) {
        self.perm = self.transposition_tables.urw_perm[self.perm as usize];
        self.ori = self.transposition_tables.urw_ori[self.ori as usize];
    }

    pub fn rw(&mut self) {
        self.perm = self.transposition_tables.rw_perm[self.perm as usize];
        self.ori = self.transposition_tables.rw_ori[self.ori as usize];
    }

    pub fn encode(&self) -> u64 {
        self.perm as u64 * 62208 + self.ori as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.perm = (coord / 62208) as u32;
        self.ori = (coord % 62208) as u32;
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

#[derive(Clone)]
struct Callback;

impl BfsCallback for Callback {
    fn new_state(&mut self, depth: usize, state: u64) {
        if depth >= 30 {
            tracing::info!("depth {depth} state {state}");
        }
    }

    fn end_of_chunk(&self, _: usize, _: usize) {}
}

fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "disk_based_bfs=info,bfs_3x3_U_r=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().compact().with_ansi(false))
        .init();

    let transposition_tables = TranspositionTables::new();
    let solved = CoordCube::new(&transposition_tables).encode();

    let mut cube = CoordCube::new(&transposition_tables);
    BfsBuilder::new()
        .expander(move |enc, arr: &mut [_; 6]| {
            cube.decode(enc);
            cube.u();
            arr[0] = cube.encode();
            cube.u();
            arr[1] = cube.encode();
            cube.u();
            arr[2] = cube.encode();
            cube.urw();
            arr[3] = cube.encode();
            cube.rw();
            arr[4] = cube.encode();
            cube.rw();
            arr[5] = cube.encode();
        })
        .callback(Callback)
        .threads(48)
        .chunk_size_bytes(529079040)
        .update_set_capacity(1 << 22)
        .capacity_check_frequency(256)
        .initial_states(&[solved])
        .state_size(5417769369600)
        .root_directories(&[PathBuf::from("run")])
        .initial_memory_limit(1 << 32)
        .update_files_compression_threshold(1 << 30)
        .build()
        .unwrap()
        .run();
}
