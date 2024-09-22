mod coord_cube;
mod cube;
mod transposition_tables;

use std::path::PathBuf;

use disk_based_bfs::{
    bfs::Bfs,
    callback::BfsCallback,
    io::LockedIO,
    settings::{BfsSettingsBuilder, BfsSettingsProvider, ChunkFilesBehavior, UpdateFilesBehavior},
};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use crate::{coord_cube::CoordCube, transposition_tables::TranspositionTables};

#[derive(Clone)]
struct Callback;

impl BfsCallback for Callback {
    fn new_state(&mut self, depth: usize, state: u64) {
        if depth > 12 {
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
        UpdateFilesBehavior::CompressAndKeep
    }

    fn chunk_files_behavior(&self, depth: usize) -> ChunkFilesBehavior {
        if depth > 6 {
            ChunkFilesBehavior::Keep
        } else {
            ChunkFilesBehavior::Delete
        }
    }
}

pub fn run() {
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

    let settings = BfsSettingsBuilder::new()
        .threads(48)
        // 48 chunks
        .chunk_size_bytes(303118200)
        .update_memory(112 * (1 << 30))
        .num_update_blocks(2 * 48 * 48)
        .capacity_check_frequency(256)
        .initial_states(&[solved])
        .state_size(116397388800)
        .root_directories(&[
            PathBuf::from("/media/ben/drive1/bfs/3x3-2-color-tennis-ball/"),
            PathBuf::from("/media/ben/drive2/bfs/3x3-2-color-tennis-ball/"),
            PathBuf::from("/media/ben/drive3/bfs/3x3-2-color-tennis-ball/"),
            PathBuf::from("/media/ben/drive4/bfs/3x3-2-color-tennis-ball/"),
        ])
        .initial_memory_limit(1 << 28)
        .available_disk_space_limit(256 * (1 << 30))
        .update_array_threshold(303118200)
        .use_locked_io(false)
        .sync_filesystem(true)
        .compute_checksums(true)
        .compress_bit_arrays(true)
        .settings_provider(SettingsProvider)
        .build()
        .unwrap();

    let mut cube = CoordCube::new(&transposition_tables);
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
            PathBuf::from("/media/ben/drive1/bfs/3x3-2-color-tennis-ball/"),
            PathBuf::from("/media/ben/drive2/bfs/3x3-2-color-tennis-ball/"),
            PathBuf::from("/media/ben/drive3/bfs/3x3-2-color-tennis-ball/"),
            PathBuf::from("/media/ben/drive4/bfs/3x3-2-color-tennis-ball/"),
        ],
    );

    let bfs = Bfs::new(&settings, &locked_io, expander, callback);
    bfs.run();
}
