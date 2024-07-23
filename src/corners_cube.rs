use std::path::PathBuf;

use disk_based_bfs::{bfs::Bfs, callback::BfsCallback, io::LockedIO, settings::BfsSettingsBuilder};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const EP_SIZE: u32 = 18480;
const EO_SIZE: u32 = 64;
const CP_SIZE: u32 = 1120;
const CO_SIZE: u32 = 729;
const EDGES_SIZE: u32 = EP_SIZE * EO_SIZE;
const CORNERS_SIZE: u32 = CP_SIZE * CO_SIZE;
const STATE_SIZE: u64 = EDGES_SIZE as u64 * CORNERS_SIZE as u64;

#[derive(Debug)]
struct Cube {
    ep: [u8; 12],
    eo: [u8; 12],
    cp: [u8; 8],
    co: [u8; 8],
}

impl Cube {
    fn new() -> Self {
        Self {
            ep: [0, 1, 1, 0, 1, 2, 1, 0, 2, 2, 1, 1],
            eo: [0; 12],
            cp: [0, 1, 0, 2, 3, 1, 0, 1],
            co: [0; 8],
        }
    }

    fn is_solved(&self) -> bool {
        if self.ep != [0, 1, 1, 0, 1, 2, 1, 0, 2, 2, 1, 1] || self.cp != [0, 1, 0, 2, 3, 1, 0, 1] {
            return false;
        }

        for i in [1, 2, 4, 6, 10, 11] {
            if self.eo[i] != 0 {
                return false;
            }
        }

        for i in [0, 1, 2, 5, 6, 7] {
            if self.co[i] != 0 {
                return false;
            }
        }

        true
    }

    fn u(&mut self) {
        let a = self.ep[0];
        self.ep[0] = self.ep[3];
        self.ep[3] = self.ep[2];
        self.ep[2] = self.ep[1];
        self.ep[1] = a;
        let a = self.eo[0];
        self.eo[0] = self.eo[3];
        self.eo[3] = self.eo[2];
        self.eo[2] = self.eo[1];
        self.eo[1] = a;
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
    }

    fn x(&mut self) {
        let a = self.ep[0];
        self.ep[0] = self.ep[10];
        self.ep[10] = self.ep[8];
        self.ep[8] = self.ep[2];
        self.ep[2] = a;
        let a = self.ep[1];
        self.ep[1] = self.ep[4];
        self.ep[4] = self.ep[9];
        self.ep[9] = self.ep[5];
        self.ep[5] = a;
        let a = self.ep[3];
        self.ep[3] = self.ep[7];
        self.ep[7] = self.ep[11];
        self.ep[11] = self.ep[6];
        self.ep[6] = a;

        let a = self.eo[0];
        self.eo[0] = (self.eo[10] + 1) % 2;
        self.eo[10] = (self.eo[8] + 1) % 2;
        self.eo[8] = (self.eo[2] + 1) % 2;
        self.eo[2] = (a + 1) % 2;
        let a = self.eo[1];
        self.eo[1] = self.eo[4];
        self.eo[4] = self.eo[9];
        self.eo[9] = self.eo[5];
        self.eo[5] = a;
        let a = self.eo[3];
        self.eo[3] = self.eo[7];
        self.eo[7] = self.eo[11];
        self.eo[11] = self.eo[6];
        self.eo[6] = a;

        let a = self.cp[0];
        self.cp[0] = self.cp[5];
        self.cp[5] = self.cp[4];
        self.cp[4] = self.cp[1];
        self.cp[1] = a;
        let a = self.cp[2];
        self.cp[2] = self.cp[3];
        self.cp[3] = self.cp[6];
        self.cp[6] = self.cp[7];
        self.cp[7] = a;

        let a = self.co[0];
        self.co[0] = (self.co[5] + 1) % 3;
        self.co[5] = (self.co[4] + 2) % 3;
        self.co[4] = (self.co[1] + 1) % 3;
        self.co[1] = (a + 2) % 3;
        let a = self.co[2];
        self.co[2] = (self.co[3] + 1) % 3;
        self.co[3] = (self.co[6] + 2) % 3;
        self.co[6] = (self.co[7] + 1) % 3;
        self.co[7] = (a + 2) % 3;
    }

    fn y(&mut self) {
        let a = self.ep[0];
        self.ep[0] = self.ep[3];
        self.ep[3] = self.ep[2];
        self.ep[2] = self.ep[1];
        self.ep[1] = a;
        let a = self.ep[4];
        self.ep[4] = self.ep[7];
        self.ep[7] = self.ep[6];
        self.ep[6] = self.ep[5];
        self.ep[5] = a;
        let a = self.ep[8];
        self.ep[8] = self.ep[9];
        self.ep[9] = self.ep[10];
        self.ep[10] = self.ep[11];
        self.ep[11] = a;

        let a = self.eo[0];
        self.eo[0] = self.eo[3];
        self.eo[3] = self.eo[2];
        self.eo[2] = self.eo[1];
        self.eo[1] = a;
        let a = self.eo[4];
        self.eo[4] = (self.eo[7] + 1) % 2;
        self.eo[7] = (self.eo[6] + 1) % 2;
        self.eo[6] = (self.eo[5] + 1) % 2;
        self.eo[5] = (a + 1) % 2;
        let a = self.eo[8];
        self.eo[8] = self.eo[9];
        self.eo[9] = self.eo[10];
        self.eo[10] = self.eo[11];
        self.eo[11] = a;

        let a = self.cp[0];
        self.cp[0] = self.cp[3];
        self.cp[3] = self.cp[2];
        self.cp[2] = self.cp[1];
        self.cp[1] = a;
        let a = self.cp[4];
        self.cp[4] = self.cp[5];
        self.cp[5] = self.cp[6];
        self.cp[6] = self.cp[7];
        self.cp[7] = a;

        let a = self.co[0];
        self.co[0] = self.co[3];
        self.co[3] = self.co[2];
        self.co[2] = self.co[1];
        self.co[1] = a;
        let a = self.co[4];
        self.co[4] = self.co[5];
        self.co[5] = self.co[6];
        self.co[6] = self.co[7];
        self.co[7] = a;
    }

    fn z(&mut self) {
        self.x();
        self.y();
        self.x();
        self.x();
        self.x();
    }

    fn xp(&mut self) {
        self.x();
        self.x();
        self.x();
    }

    fn yp(&mut self) {
        self.y();
        self.y();
        self.y();
    }

    fn zp(&mut self) {
        self.z();
        self.z();
        self.z();
    }

    fn l(&mut self) {
        self.z();
        self.u();
        self.zp();
    }

    fn f(&mut self) {
        self.x();
        self.u();
        self.xp();
    }

    fn r(&mut self) {
        self.zp();
        self.u();
        self.z();
    }

    fn b(&mut self) {
        self.xp();
        self.u();
        self.x();
    }

    fn d(&mut self) {
        self.x();
        self.x();
        self.u();
        self.x();
        self.x();
    }

    fn up(&mut self) {
        self.u();
        self.u();
        self.u();
    }

    fn lp(&mut self) {
        self.l();
        self.l();
        self.l();
    }

    fn fp(&mut self) {
        self.f();
        self.f();
        self.f();
    }

    fn rp(&mut self) {
        self.r();
        self.r();
        self.r();
    }

    fn bp(&mut self) {
        self.b();
        self.b();
        self.b();
    }

    fn dp(&mut self) {
        self.d();
        self.d();
        self.d();
    }

    fn ep_coord(&self) -> u32 {
        combinatorics::indexing::encode_multiset(self.ep, [3, 6, 3]) as u32
    }

    /// Depends on `self.ep`
    fn eo_coord(&self) -> u32 {
        let mut coord = 0;
        for i in 0..12 {
            if self.ep[i] == 1 {
                coord *= 2;
                coord += self.eo[i] as u32;
            }
        }
        coord
    }

    fn cp_coord(&self) -> u32 {
        combinatorics::indexing::encode_multiset(self.cp, [3, 3, 1, 1]) as u32
    }

    /// Depends on `self.cp`
    fn co_coord(&self) -> u32 {
        let mut coord = 0;
        for i in 0..8 {
            if self.cp[i] <= 1 {
                coord *= 3;
                coord += self.co[i] as u32;
            }
        }
        coord
    }

    fn set_ep_coord(&mut self, coord: u32) {
        self.ep = combinatorics::indexing::decode_multiset(coord as u128, [3, 6, 3]);
    }

    /// Depends on `self.ep`
    fn set_eo_coord(&mut self, mut coord: u32) {
        for i in (0..12).rev() {
            if self.ep[i] == 1 {
                self.eo[i] = (coord % 2) as u8;
                coord /= 2;
            }
        }
    }

    fn set_cp_coord(&mut self, coord: u32) {
        self.cp = combinatorics::indexing::decode_multiset(coord as u128, [3, 3, 1, 1]);
    }

    /// Depends on `self.cp`
    fn set_co_coord(&mut self, mut coord: u32) {
        for i in (0..8).rev() {
            if self.cp[i] <= 1 {
                self.co[i] = (coord % 3) as u8;
                coord /= 3;
            }
        }
    }

    fn edges_coord(&self) -> u32 {
        self.ep_coord() * EO_SIZE + self.eo_coord()
    }

    fn corners_coord(&self) -> u32 {
        self.cp_coord() * CO_SIZE + self.co_coord()
    }

    fn set_edges_coord(&mut self, coord: u32) {
        // Must set EP before EO
        self.set_ep_coord(coord / EO_SIZE);
        self.set_eo_coord(coord % EO_SIZE);
    }

    fn set_corners_coord(&mut self, coord: u32) {
        // Must set CP before CO
        self.set_cp_coord(coord / CO_SIZE);
        self.set_co_coord(coord % CO_SIZE);
    }

    fn encode(&self) -> u64 {
        self.edges_coord() as u64 * CORNERS_SIZE as u64 + self.corners_coord() as u64
    }

    fn decode(&mut self, coord: u64) {
        self.set_edges_coord((coord / CORNERS_SIZE as u64) as u32);
        self.set_corners_coord((coord % CORNERS_SIZE as u64) as u32);
    }
}

struct TranspositionTables {
    u_edges: Vec<u32>,
    u_corners: Vec<u32>,
    l_edges: Vec<u32>,
    l_corners: Vec<u32>,
    f_edges: Vec<u32>,
    f_corners: Vec<u32>,
    r_edges: Vec<u32>,
    r_corners: Vec<u32>,
    b_edges: Vec<u32>,
    b_corners: Vec<u32>,
    d_edges: Vec<u32>,
    d_corners: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_edges = vec![0; EDGES_SIZE as usize];
        let mut u_corners = vec![0; CORNERS_SIZE as usize];
        let mut l_edges = vec![0; EDGES_SIZE as usize];
        let mut l_corners = vec![0; CORNERS_SIZE as usize];
        let mut f_edges = vec![0; EDGES_SIZE as usize];
        let mut f_corners = vec![0; CORNERS_SIZE as usize];
        let mut r_edges = vec![0; EDGES_SIZE as usize];
        let mut r_corners = vec![0; CORNERS_SIZE as usize];
        let mut b_edges = vec![0; EDGES_SIZE as usize];
        let mut b_corners = vec![0; CORNERS_SIZE as usize];
        let mut d_edges = vec![0; EDGES_SIZE as usize];
        let mut d_corners = vec![0; CORNERS_SIZE as usize];

        let mut cube = Cube::new();

        for i in 0..EDGES_SIZE as usize {
            cube.set_edges_coord(i as u32);
            cube.u();
            u_edges[i] = cube.edges_coord();
            cube.up();
            cube.l();
            l_edges[i] = cube.edges_coord();
            cube.lp();
            cube.f();
            f_edges[i] = cube.edges_coord();
            cube.fp();
            cube.r();
            r_edges[i] = cube.edges_coord();
            cube.rp();
            cube.b();
            b_edges[i] = cube.edges_coord();
            cube.bp();
            cube.d();
            d_edges[i] = cube.edges_coord();
        }

        for i in 0..CORNERS_SIZE as usize {
            cube.set_corners_coord(i as u32);
            cube.u();
            u_corners[i] = cube.corners_coord();
            cube.up();
            cube.l();
            l_corners[i] = cube.corners_coord();
            cube.lp();
            cube.f();
            f_corners[i] = cube.corners_coord();
            cube.fp();
            cube.r();
            r_corners[i] = cube.corners_coord();
            cube.rp();
            cube.b();
            b_corners[i] = cube.corners_coord();
            cube.bp();
            cube.d();
            d_corners[i] = cube.corners_coord();
        }

        Self {
            u_edges,
            u_corners,
            l_edges,
            l_corners,
            f_edges,
            f_corners,
            r_edges,
            r_corners,
            b_edges,
            b_corners,
            d_edges,
            d_corners,
        }
    }
}

#[derive(Clone)]
struct CoordCube<'a> {
    edges: u32,
    corners: u32,
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
            edges: cube.edges_coord(),
            corners: cube.corners_coord(),
            transposition_tables,
        }
    }

    pub fn u(&mut self) {
        self.edges = self.transposition_tables.u_edges[self.edges as usize];
        self.corners = self.transposition_tables.u_corners[self.corners as usize];
    }

    pub fn l(&mut self) {
        self.edges = self.transposition_tables.l_edges[self.edges as usize];
        self.corners = self.transposition_tables.l_corners[self.corners as usize];
    }

    pub fn f(&mut self) {
        self.edges = self.transposition_tables.f_edges[self.edges as usize];
        self.corners = self.transposition_tables.f_corners[self.corners as usize];
    }

    pub fn r(&mut self) {
        self.edges = self.transposition_tables.r_edges[self.edges as usize];
        self.corners = self.transposition_tables.r_corners[self.corners as usize];
    }

    pub fn b(&mut self) {
        self.edges = self.transposition_tables.b_edges[self.edges as usize];
        self.corners = self.transposition_tables.b_corners[self.corners as usize];
    }

    pub fn d(&mut self) {
        self.edges = self.transposition_tables.d_edges[self.edges as usize];
        self.corners = self.transposition_tables.d_corners[self.corners as usize];
    }

    pub fn encode(&self) -> u64 {
        self.edges as u64 * CORNERS_SIZE as u64 + self.corners as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.edges = (coord / CORNERS_SIZE as u64) as u32;
        self.corners = (coord % CORNERS_SIZE as u64) as u32;
    }
}

impl From<CoordCube<'_>> for Cube {
    fn from(value: CoordCube<'_>) -> Self {
        let mut cube = Cube::new();
        cube.set_edges_coord(value.edges as u32);
        cube.set_corners_coord(value.corners as u32);
        cube
    }
}

#[derive(Clone)]
struct Callback;

impl BfsCallback for Callback {
    fn new_state(&mut self, depth: usize, state: u64) {
        if depth >= 13 {
            tracing::info!("depth {depth} state {state}");
        }
    }

    fn end_of_chunk(&self, _: usize, _: usize) {}
}

pub fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "disk_based_bfs=trace,bfs_3x3_2_color=trace".into()),
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
        // 16 * 48 chunks
        .chunk_size_bytes(157172400)
        .update_memory(1 << 37)
        .capacity_check_frequency(256)
        .initial_states(&[solved])
        .state_size(965667225600)
        .root_directories(&[
            PathBuf::from("/media/ben/drive2/bfs/3x3-2-color-corners/"),
            PathBuf::from("/media/ben/drive3/bfs/3x3-2-color-corners/"),
            PathBuf::from("/media/ben/drive4/bfs/3x3-2-color-corners/"),
        ])
        .initial_memory_limit(1 << 28)
        .update_files_compression_threshold(1 << 34)
        .buf_io_capacity(1 << 23)
        .use_locked_io(true)
        .sync_filesystem(true)
        .compress_update_files_at_end_of_iter(true)
        .build()
        .unwrap();

    let expander = move |enc, arr: &mut [_; 18]| {
        cube.decode(enc);
        cube.u();
        arr[0] = cube.encode() as u64;
        cube.u();
        arr[1] = cube.encode();
        cube.u();
        arr[2] = cube.encode();
        cube.u();
        cube.l();
        arr[3] = cube.encode();
        cube.l();
        arr[4] = cube.encode();
        cube.l();
        arr[5] = cube.encode();
        cube.l();
        cube.f();
        arr[6] = cube.encode();
        cube.f();
        arr[7] = cube.encode();
        cube.f();
        arr[8] = cube.encode();
        cube.f();
        cube.r();
        arr[9] = cube.encode();
        cube.r();
        arr[10] = cube.encode();
        cube.r();
        arr[11] = cube.encode();
        cube.r();
        cube.b();
        arr[12] = cube.encode();
        cube.b();
        arr[13] = cube.encode();
        cube.b();
        arr[14] = cube.encode();
        cube.b();
        cube.d();
        arr[15] = cube.encode();
        cube.d();
        arr[16] = cube.encode();
        cube.d();
        arr[17] = cube.encode();
    };
    let callback = Callback;

    let locked_io = LockedIO::new(
        &settings,
        vec![
            PathBuf::from("/media/ben/drive2/bfs/3x3-2-color-corners/"),
            PathBuf::from("/media/ben/drive3/bfs/3x3-2-color-corners/"),
            PathBuf::from("/media/ben/drive4/bfs/3x3-2-color-corners/"),
        ],
    );

    let bfs = Bfs::new(&settings, &locked_io, expander, callback);
    bfs.run();
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_cube() {
        let mut cube = Cube::new();
        cube.u();
        assert!(!cube.is_solved());
        cube.u();
        assert!(!cube.is_solved());
        cube.d();
        assert!(!cube.is_solved());
        cube.d();
        assert!(!cube.is_solved());
        cube.u();
        cube.u();
        cube.d();
        cube.d();
        cube.l();
        assert!(!cube.is_solved());
        cube.l();
        assert!(!cube.is_solved());
        cube.l();
        assert!(!cube.is_solved());
        cube.l();
        assert!(cube.is_solved());

        // R U R' U' R' F R2 U' R' U' R U R' F'
        for _ in 0..2 {
            cube.r();
            cube.u();
            cube.rp();
            cube.up();
            cube.rp();
            cube.f();
            cube.r();
            cube.r();
            cube.up();
            cube.rp();
            cube.up();
            cube.r();
            cube.u();
            cube.rp();
            cube.fp();
        }
        assert!(cube.is_solved());
    }

    #[test]
    fn test_eo_coord_1() {
        let mut cube = Cube::new();
        cube.u();
        assert_eq!(cube.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        cube.up();
        cube.l();
        assert_eq!(cube.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        cube.lp();
        cube.f();
        assert_eq!(cube.eo, [1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0]);
        cube.fp();
        cube.r();
        assert_eq!(cube.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        cube.rp();
        cube.b();
        assert_eq!(cube.eo, [0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0]);
        cube.bp();
        cube.d();
        assert_eq!(cube.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_eo_coord_2() {
        let mut cube = Cube::new();
        let eo = cube.eo_coord();

        // flip 4 edges that don't have orientation, and check eo coord
        cube.u();
        cube.l();
        cube.l();
        cube.d();
        cube.f();

        assert_eq!(cube.eo_coord(), eo);
    }

    #[test]
    fn test_edges_coord() {
        let mut cube = Cube::new();
        for i in 0..EDGES_SIZE {
            cube.set_edges_coord(i as u32);
            assert_eq!(cube.edges_coord(), i as u32);
        }
    }

    #[test]
    fn test_corners_coord() {
        let mut cube = Cube::new();
        for i in 0..CORNERS_SIZE {
            cube.set_corners_coord(i as u32);
            assert_eq!(cube.corners_coord(), i as u32);
        }
    }

    #[test]
    fn test_encode() {
        let mut x = 0u64;
        let mut cube = Cube::new();
        for _ in 0..65536 {
            x = x.wrapping_mul(450349535401847371);
            x = x.wrapping_add(380506838312516788);
            let coord = x % STATE_SIZE;
            cube.decode(coord);
            assert_eq!(cube.encode(), coord);
        }
    }

    #[test]
    fn test_depth_1() {
        let mut cube = Cube::new();
        let solved = cube.encode();
        let mut arr = [0; 18];
        println!("{cube:?}");
        cube.u();
        println!("u {cube:?}");
        arr[0] = cube.encode();
        cube.u();
        println!("u {cube:?}");
        arr[1] = cube.encode();
        cube.u();
        println!("u {cube:?}");
        arr[2] = cube.encode();
        cube.u();
        println!("u {cube:?}");
        cube.l();
        println!("l {cube:?}");
        arr[3] = cube.encode();
        cube.l();
        println!("l {cube:?}");
        arr[4] = cube.encode();
        cube.l();
        println!("l {cube:?}");
        arr[5] = cube.encode();
        cube.l();
        println!("l {cube:?}");
        cube.f();
        println!("f {cube:?}");
        arr[6] = cube.encode();
        cube.f();
        println!("f {cube:?}");
        arr[7] = cube.encode();
        cube.f();
        println!("f {cube:?}");
        arr[8] = cube.encode();
        cube.f();
        println!("f {cube:?}");
        cube.r();
        println!("r {cube:?}");
        arr[9] = cube.encode();
        cube.r();
        println!("r {cube:?}");
        arr[10] = cube.encode();
        cube.r();
        println!("r {cube:?}");
        arr[11] = cube.encode();
        cube.r();
        println!("r {cube:?}");
        cube.b();
        println!("b {cube:?}");
        arr[12] = cube.encode();
        cube.b();
        println!("b {cube:?}");
        arr[13] = cube.encode();
        cube.b();
        println!("b {cube:?}");
        arr[14] = cube.encode();
        cube.b();
        println!("b {cube:?}");
        cube.d();
        println!("d {cube:?}");
        arr[15] = cube.encode();
        cube.d();
        println!("d {cube:?}");
        arr[16] = cube.encode();
        cube.d();
        println!("d {cube:?}");
        arr[17] = cube.encode();
        cube.d();
        println!("d {cube:?}");

        let set = arr.iter().copied().collect::<HashSet<_>>();
        assert_eq!(set.len(), 18);
    }

    #[test]
    fn test_coord_cube() {
        let transposition_tables = TranspositionTables::new();
        let mut coord_cube = CoordCube::new(&transposition_tables);
        let mut cube = Cube::new();

        let mut x = 0u64;
        for _ in 0..65536 {
            x = x.wrapping_mul(450349535401847371);
            x = x.wrapping_add(380506838312516788);

            let coord = x % STATE_SIZE as u64;

            cube.decode(coord);
            coord_cube.decode(coord);

            println!("coord = {coord}");
            println!("cube = {cube:?}");
            for i in 0..4 {
                coord_cube.u();
                cube.u();
                println!("u{}", i + 1);
                println!("cube = {cube:?}");
                assert_eq!(cube.encode(), coord_cube.encode());
            }
            for i in 0..4 {
                coord_cube.l();
                cube.l();
                println!("l{}", i + 1);
                println!("cube = {cube:?}");
                assert_eq!(cube.encode(), coord_cube.encode());
            }
            for i in 0..4 {
                coord_cube.f();
                cube.f();
                println!("f{}", i + 1);
                assert_eq!(cube.encode(), coord_cube.encode());
            }
            for i in 0..4 {
                coord_cube.r();
                cube.r();
                println!("r{}", i + 1);
                assert_eq!(cube.encode(), coord_cube.encode());
            }
            for i in 0..4 {
                coord_cube.b();
                cube.b();
                println!("b{}", i + 1);
                assert_eq!(cube.encode(), coord_cube.encode());
            }
            for i in 0..4 {
                coord_cube.d();
                cube.d();
                println!("d{}", i + 1);
                assert_eq!(cube.encode(), coord_cube.encode());
            }
        }
    }
}
