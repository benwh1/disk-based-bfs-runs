use std::path::PathBuf;

use disk_based_bfs::{
    bfs::Bfs,
    callback::BfsCallback,
    io::LockedIO,
    settings::{BfsSettingsBuilder, BfsSettingsProvider, ChunkFilesBehavior, UpdateFilesBehavior},
};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

struct Cube {
    corners: u8,
    edges: [u8; 10],
    centers: [u8; 10],
}

impl Cube {
    pub fn new() -> Self {
        Self {
            corners: 0,
            edges: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            centers: [0, 0, 0, 0, 1, 1, 2, 2, 3, 3],
        }
    }

    pub fn u(&mut self) {
        self.corners = (self.corners + 1) % 4;
        self.edges[0..8].rotate_right(2);
        self.centers[0..4].rotate_right(1);
    }

    pub fn uinv(&mut self) {
        self.corners = (self.corners + 3) % 4;
        self.edges[0..8].rotate_left(2);
        self.centers[0..4].rotate_left(1);
    }

    pub fn r(&mut self) {
        let x = self.edges[0];
        self.edges[0] = self.edges[8];
        self.edges[8] = self.edges[9];
        self.edges[9] = self.edges[5];
        self.edges[5] = x;
        self.centers[2..10].rotate_left(2);
    }

    pub fn rinv(&mut self) {
        let x = self.edges[0];
        self.edges[0] = self.edges[5];
        self.edges[5] = self.edges[9];
        self.edges[9] = self.edges[8];
        self.edges[8] = x;
        self.centers[2..10].rotate_right(2);
    }

    pub fn edge_coord(&self) -> u32 {
        combinatorics::indexing::encode_permutation(self.edges) as u32
    }

    pub fn center_coord(&self) -> u32 {
        combinatorics::indexing::encode_multiset(self.centers, [4, 2, 2, 2]) as u32
    }

    pub fn center_corner_coord(&self) -> u32 {
        self.center_coord() * 4 + self.corners as u32
    }

    pub fn set_edge_coord(&mut self, coord: u32) {
        self.edges = combinatorics::indexing::decode_permutation(coord as u64);
    }

    pub fn set_center_coord(&mut self, coord: u32) {
        self.centers = combinatorics::indexing::decode_multiset(coord as u128, [4, 2, 2, 2]);
    }

    pub fn set_center_corner_coord(&mut self, coord: u32) {
        self.corners = (coord % 4) as u8;
        self.set_center_coord(coord / 4);
    }
}

struct TranspositionTables {
    u_edges: Vec<u32>,
    u_centers_corners: Vec<u32>,
    u2_edges: Vec<u32>,
    u2_centers_corners: Vec<u32>,
    ur_edges: Vec<u32>,
    ur_centers_corners: Vec<u32>,
    r2_edges: Vec<u32>,
    r2_centers_corners: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_edges = vec![0; 3628800];
        let mut u_centers_corners = vec![0; 75600];
        let mut u2_edges = vec![0; 3628800];
        let mut u2_centers_corners = vec![0; 75600];
        let mut ur_edges = vec![0; 3628800];
        let mut ur_centers_corners = vec![0; 75600];
        let mut r2_edges = vec![0; 3628800];
        let mut r2_centers_corners = vec![0; 75600];

        let mut cube = Cube::new();

        for i in 0..3628800 {
            cube.set_edge_coord(i);
            let i = i as usize;
            cube.u();
            u_edges[i] = cube.edge_coord();
            cube.u();
            u2_edges[i] = cube.edge_coord();
            cube.uinv();
            cube.r();
            ur_edges[i] = cube.edge_coord();
            cube.rinv();
            cube.uinv();
            cube.r();
            cube.r();
            r2_edges[i] = cube.edge_coord();
        }

        for i in 0..75600 {
            cube.set_center_corner_coord(i);
            let i = i as usize;
            cube.u();
            u_centers_corners[i] = cube.center_corner_coord();
            cube.u();
            u2_centers_corners[i] = cube.center_corner_coord();
            cube.uinv();
            cube.r();
            ur_centers_corners[i] = cube.center_corner_coord();
            cube.rinv();
            cube.uinv();
            cube.r();
            cube.r();
            r2_centers_corners[i] = cube.center_corner_coord();
        }

        Self {
            u_edges,
            u_centers_corners,
            u2_edges,
            u2_centers_corners,
            ur_edges,
            ur_centers_corners,
            r2_edges,
            r2_centers_corners,
        }
    }
}

#[derive(Clone)]
struct CoordCube<'a> {
    edges: u32,
    centers_corners: u32,
    transposition_tables: &'a TranspositionTables,
}

impl<'a> CoordCube<'a> {
    pub fn new(transposition_tables: &'a TranspositionTables) -> Self {
        let cube = Cube::new();
        Self {
            edges: cube.edge_coord(),
            centers_corners: cube.center_corner_coord(),
            transposition_tables,
        }
    }

    pub fn u(&mut self) {
        self.edges = self.transposition_tables.u_edges[self.edges as usize];
        self.centers_corners =
            self.transposition_tables.u_centers_corners[self.centers_corners as usize];
    }

    pub fn u2(&mut self) {
        self.edges = self.transposition_tables.u2_edges[self.edges as usize];
        self.centers_corners =
            self.transposition_tables.u2_centers_corners[self.centers_corners as usize];
    }

    pub fn ur(&mut self) {
        self.edges = self.transposition_tables.ur_edges[self.edges as usize];
        self.centers_corners =
            self.transposition_tables.ur_centers_corners[self.centers_corners as usize];
    }

    pub fn r2(&mut self) {
        self.edges = self.transposition_tables.r2_edges[self.edges as usize];
        self.centers_corners =
            self.transposition_tables.r2_centers_corners[self.centers_corners as usize];
    }

    pub fn encode(&self) -> u64 {
        self.edges as u64 * 75600 + self.centers_corners as u64
    }

    pub fn decode(&mut self, coord: u64) {
        self.edges = (coord / 75600) as u32;
        self.centers_corners = (coord % 75600) as u32;
    }
}

#[derive(Clone)]
struct Callback;

impl BfsCallback for Callback {
    fn new_state(&mut self, depth: usize, state: u64) {
        if depth > 33 {
            tracing::info!("depth {depth} state {state}");
        }
    }

    fn end_of_chunk(&self, _: usize, _: usize) {}
}

struct SettingsProvider;

impl BfsSettingsProvider for SettingsProvider {
    fn chunk_root_idx(&self, chunk_idx: usize) -> usize {
        [0, 1, 2, 3, 1, 2, 3][chunk_idx % 7]
    }

    fn update_files_behavior(&self, _: usize) -> UpdateFilesBehavior {
        UpdateFilesBehavior::DontCompress
    }

    fn chunk_files_behavior(&self, _: usize) -> ChunkFilesBehavior {
        ChunkFilesBehavior::Delete
    }
}

fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "disk_based_bfs=trace,bfs_4x4_U_2R=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().compact().with_ansi(false))
        .init();

    let transposition_tables = TranspositionTables::new();
    let solved = CoordCube::new(&transposition_tables).encode();

    let settings = BfsSettingsBuilder::new()
        .threads(48)
        // 12 * 48 chunks
        .chunk_size_bytes(476280000)
        .update_memory(112 * (1 << 30))
        .num_update_blocks(2 * 12 * 48 * 48)
        .capacity_check_frequency(256)
        .initial_states(&[solved])
        .state_size(274337280000)
        .root_directories(&[
            PathBuf::from("/media/ben/drive1/bfs/4x4-U-2R/"),
            PathBuf::from("/media/ben/drive2/bfs/4x4-U-2R/"),
            PathBuf::from("/media/ben/drive3/bfs/4x4-U-2R/"),
            PathBuf::from("/media/ben/drive4/bfs/4x4-U-2R/"),
        ])
        .initial_memory_limit(1 << 32)
        .available_disk_space_limit(256 * (1 << 30))
        .update_array_threshold(476280000)
        .use_locked_io(false)
        .sync_filesystem(true)
        .compute_checksums(true)
        .compress_bit_arrays(true)
        .settings_provider(SettingsProvider)
        .build()
        .unwrap();

    let mut cube = CoordCube::new(&transposition_tables);
    let expander = move |enc, arr: &mut [_; 4]| {
        cube.decode(enc);
        cube.u();
        arr[0] = cube.encode();
        cube.u2();
        arr[1] = cube.encode();
        cube.ur();
        arr[2] = cube.encode();
        cube.r2();
        arr[3] = cube.encode();
    };
    let callback = Callback;

    let locked_io = LockedIO::new(
        &settings,
        vec![
            PathBuf::from("/media/ben/drive1/bfs/4x4-U-2R/"),
            PathBuf::from("/media/ben/drive2/bfs/4x4-U-2R/"),
            PathBuf::from("/media/ben/drive3/bfs/4x4-U-2R/"),
            PathBuf::from("/media/ben/drive4/bfs/4x4-U-2R/"),
        ],
    );

    let bfs = Bfs::new(&settings, &locked_io, expander, callback);
    bfs.run();
}
