use std::path::PathBuf;

use disk_based_bfs::{bfs::Bfs, callback::BfsCallback, io::LockedIO, settings::BfsSettingsBuilder};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const EP_SIZE: u32 = 181440;
const CP_SIZE: u32 = 5040;
const CO_SIZE: u32 = 729;
const CORNERS_SIZE: u32 = CP_SIZE * CO_SIZE;
const STATE_SIZE: u64 = EP_SIZE as u64 * CORNERS_SIZE as u64;

#[derive(Debug, PartialEq)]
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

    pub fn do_move(&mut self, m: u8) {
        match m {
            0 => self.u(),
            1 => self.r(),
            2 => self.f2(),
            _ => panic!("Invalid move"),
        }
    }

    fn up(&mut self) {
        self.u();
        self.u();
        self.u();
    }

    fn rp(&mut self) {
        self.r();
        self.r();
        self.r();
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

struct TranspositionTables {
    u_edges: Vec<u32>,
    u_corners: Vec<u32>,
    r_edges: Vec<u32>,
    r_corners: Vec<u32>,
    // We need two sets of tables for f2 because parity determines whether we swap pieces 7 and 8,
    // and those two pieces are on the F face but not in U or R
    f2_edges_even: Vec<u32>,
    f2_edges_odd: Vec<u32>,
    f2_corners: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_edges = vec![0; EP_SIZE as usize];
        let mut u_corners = vec![0; CORNERS_SIZE as usize];
        let mut r_edges = vec![0; EP_SIZE as usize];
        let mut r_corners = vec![0; CORNERS_SIZE as usize];
        let mut f2_edges_even = vec![0; EP_SIZE as usize];
        let mut f2_edges_odd = vec![0; EP_SIZE as usize];
        let mut f2_corners = vec![0; CORNERS_SIZE as usize];

        let mut cube = Cube::new();

        for i in 0..CORNERS_SIZE as usize {
            cube.set_corners_coord(i as u32);
            cube.u();
            u_corners[i] = cube.corners_coord();
            cube.up();
            cube.r();
            r_corners[i] = cube.corners_coord();
            cube.rp();
            cube.f2();
            f2_corners[i] = cube.corners_coord();
        }

        cube.is_even_perm = true;

        for i in 0..EP_SIZE as usize {
            cube.set_ep_coord(i as u32);
            cube.u();
            u_edges[i] = cube.ep_coord();
            cube.up();
            cube.r();
            r_edges[i] = cube.ep_coord();
            cube.rp();
            cube.f2();
            f2_edges_even[i] = cube.ep_coord();
        }

        cube.is_even_perm = false;

        for i in 0..EP_SIZE as usize {
            cube.set_ep_coord(i as u32);
            cube.f2();
            f2_edges_odd[i] = cube.ep_coord();
        }

        Self {
            u_edges,
            u_corners,
            r_edges,
            r_corners,
            f2_edges_even,
            f2_edges_odd,
            f2_corners,
        }
    }
}

#[derive(Clone)]
struct CoordCube<'a> {
    edges: u32,
    corners: u32,
    is_even_perm: bool,
    transposition_tables: &'a TranspositionTables,
}

impl<'a> std::fmt::Debug for CoordCube<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoordCube")
            .field("edges", &self.edges)
            .field("corners", &self.corners)
            .finish()
    }
}

impl<'a> CoordCube<'a> {
    pub fn new(transposition_tables: &'a TranspositionTables) -> Self {
        let cube = Cube::new();
        Self {
            edges: cube.ep_coord(),
            corners: cube.corners_coord(),
            is_even_perm: cube.is_even_perm,
            transposition_tables,
        }
    }

    pub fn u(&mut self) {
        self.edges = self.transposition_tables.u_edges[self.edges as usize];
        self.corners = self.transposition_tables.u_corners[self.corners as usize];
        self.is_even_perm = !self.is_even_perm;
    }

    pub fn r(&mut self) {
        self.edges = self.transposition_tables.r_edges[self.edges as usize];
        self.corners = self.transposition_tables.r_corners[self.corners as usize];
        self.is_even_perm = !self.is_even_perm;
    }

    pub fn f2(&mut self) {
        self.edges = if self.is_even_perm {
            self.transposition_tables.f2_edges_even[self.edges as usize]
        } else {
            self.transposition_tables.f2_edges_odd[self.edges as usize]
        };
        self.corners = self.transposition_tables.f2_corners[self.corners as usize];
    }

    pub fn do_move(&mut self, m: u8) {
        match m {
            0 => self.u(),
            1 => self.r(),
            2 => self.f2(),
            _ => panic!("Invalid move"),
        }
    }

    pub fn encode(&self) -> u64 {
        self.edges as u64 * CORNERS_SIZE as u64 + self.corners as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.corners = (coord % CORNERS_SIZE as u64) as u32;
        self.edges = (coord / CORNERS_SIZE as u64) as u32;

        let mut cube = Cube::new();
        cube.set_corners_coord(self.corners);

        self.is_even_perm = cube.is_even_perm;
    }
}

impl From<CoordCube<'_>> for Cube {
    fn from(value: CoordCube<'_>) -> Self {
        let mut cube = Cube::new();
        cube.set_corners_coord(value.corners as u32);
        cube.set_ep_coord(value.edges as u32);
        cube
    }
}

#[derive(Clone)]
struct Callback;

impl BfsCallback for Callback {
    fn new_state(&mut self, depth: usize, state: u64) {
        if depth >= 21 {
            tracing::info!("depth {depth} state {state}");
        }
    }

    fn end_of_chunk(&self, _: usize, _: usize) {}
}

fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "disk_based_bfs=trace,bfs_3x3_U_R_F2=trace".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(false)
                .with_thread_names(true)
                .with_line_number(true),
        )
        .init();

    let transposition_tables = TranspositionTables::new();
    let solved = CoordCube::new(&transposition_tables).encode() as u64;

    let mut cube = CoordCube::new(&transposition_tables);
    let settings = BfsSettingsBuilder::new()
        .threads(48)
        // 4 * 48 chunks
        .chunk_size_bytes(434010150)
        .update_memory(1 << 37)
        .capacity_check_frequency(256)
        .initial_states(&[solved])
        .state_size(666639590400)
        .root_directories(&[
            PathBuf::from("/media/ben/drive2/bfs/3x3-U-R-F2/"),
            PathBuf::from("/media/ben/drive3/bfs/3x3-U-R-F2/"),
            PathBuf::from("/media/ben/drive4/bfs/3x3-U-R-F2/"),
        ])
        .initial_memory_limit(1 << 28)
        .update_files_compression_threshold(3 * (1 << 32))
        .buf_io_capacity(1 << 23)
        .use_locked_io(true)
        .sync_filesystem(true)
        .compress_update_files_at_end_of_iter(true)
        .build()
        .unwrap();

    let expander = move |enc, arr: &mut [_; 7]| {
        cube.decode(enc);
        cube.u();
        arr[0] = cube.encode();
        cube.u();
        arr[1] = cube.encode();
        cube.u();
        arr[2] = cube.encode();
        cube.u();
        cube.r();
        arr[3] = cube.encode();
        cube.r();
        arr[4] = cube.encode();
        cube.r();
        arr[5] = cube.encode();
        cube.r();
        cube.f2();
        arr[6] = cube.encode();
    };
    let callback = Callback;

    let locked_io = LockedIO::new(
        &settings,
        vec![
            PathBuf::from("/media/ben/drive2/bfs/3x3-U-R-F2/"),
            PathBuf::from("/media/ben/drive3/bfs/3x3-U-R-F2/"),
            PathBuf::from("/media/ben/drive4/bfs/3x3-U-R-F2/"),
        ],
    );

    let bfs = Bfs::new(&settings, &locked_io, expander, callback);
    bfs.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_decode_encode() {
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

    #[test]
    fn test_coord_cube_decode_encode() {
        let transposition_tables = TranspositionTables::new();
        let mut cube = CoordCube::new(&transposition_tables);

        let mut x = 0u64;
        for _ in 0..65536 {
            x = x
                .wrapping_mul(450349535401847371)
                .wrapping_add(380506838312516788);
            let coord = x % STATE_SIZE;
            cube.decode(coord);
            assert_eq!(cube.encode(), coord);
        }
    }

    #[test]
    fn test_cube_matches_coord_cube() {
        let transposition_tables = TranspositionTables::new();

        let mut cube = Cube::new();
        let mut coord_cube = CoordCube::new(&transposition_tables);

        let mut x = 0u64;
        for _ in 0..65536 {
            x = x
                .wrapping_mul(450349535401847371)
                .wrapping_add(380506838312516788);
            let mv = (x % 3) as u8;

            cube.do_move(mv);
            coord_cube.do_move(mv);

            assert_eq!(cube.encode(), coord_cube.encode());
        }
    }

    #[test]
    fn test_cube_matches_coord_cube_2() {
        let transposition_tables = TranspositionTables::new();

        let mut cube = Cube::new();
        let mut coord_cube = CoordCube::new(&transposition_tables);

        let mut x = 0u64;
        for _ in 0..65536 {
            x = x
                .wrapping_mul(450349535401847371)
                .wrapping_add(380506838312516788);
            let enc = x % STATE_SIZE;

            cube.decode(enc);
            coord_cube.decode(enc);
            let cube2 = Cube::from(coord_cube.clone());

            assert_eq!(cube, cube2);
            assert_eq!(cube.encode(), coord_cube.encode());
        }
    }

    #[test]
    fn test_random_scramble() {
        // Scramble: U R U F2 R F2 F2 R U U U U U R U F2 U R U U U U U F2 R F2 U R F2 R U U F2 R R
        // F2 F2 U U U F2 U R F2 R U R R F2 R U F2 F2 U U F2 F2 R F2 F2 R F2 U R U U U F2 U R R R U
        // F2 U F2 R U R U R U U U R F2 F2 F2 R R U R U U U R F2 F2 U R
        let moves = [
            0, 1, 0, 2, 1, 2, 2, 1, 0, 0, 0, 0, 0, 1, 0, 2, 0, 1, 0, 0, 0, 0, 0, 2, 1, 2, 0, 1, 2,
            1, 0, 0, 2, 1, 1, 2, 2, 0, 0, 0, 2, 0, 1, 2, 1, 0, 1, 1, 2, 1, 0, 2, 2, 0, 0, 2, 2, 1,
            2, 2, 1, 2, 0, 1, 0, 0, 0, 2, 0, 1, 1, 1, 0, 2, 0, 2, 1, 0, 1, 0, 1, 0, 0, 0, 1, 2, 2,
            2, 1, 1, 0, 1, 0, 0, 0, 1, 2, 2, 0, 1,
        ];

        let mut cube = Cube::new();
        for mv in moves {
            cube.do_move(mv);
        }

        // Solution: F2 R U F2 U2 R F2 U' R2 U' F2 R2 U R2 U2 R2 U2 F2 R2
        let solution = [
            2, 1, 0, 2, 0, 0, 1, 2, 0, 0, 0, 1, 1, 0, 0, 0, 2, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 2,
            1, 1,
        ];

        for mv in solution {
            cube.do_move(mv);
        }

        let solved_cube = Cube::new();
        assert_eq!(cube.encode(), solved_cube.encode());
    }
}
