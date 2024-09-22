#![allow(non_snake_case)]

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
        if depth > 33 {
            tracing::info!("depth {depth} state {state}");
        }
    }

    fn end_of_chunk(&self, _: usize, _: usize) {}
}

struct SettingsProvider;

impl BfsSettingsProvider for SettingsProvider {
    fn chunk_root_idx(&self, chunk_idx: usize) -> usize {
        chunk_idx % 4
    }

    fn update_files_behavior(&self, _: usize) -> UpdateFilesBehavior {
        UpdateFilesBehavior::DontCompress
    }

    fn chunk_files_behavior(&self, _: usize) -> ChunkFilesBehavior {
        ChunkFilesBehavior::Delete
    }
}

pub fn run() {
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
        // 2 * 48 chunks
        .chunk_size_bytes(357210000)
        .update_memory(112 * (1 << 30))
        .num_update_blocks(2 * 2 * 48 * 48)
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
        .update_array_threshold(357210000)
        .use_locked_io(false)
        .sync_filesystem(false)
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
