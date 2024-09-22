use std::path::PathBuf;

use disk_based_bfs::{
    bfs::Bfs,
    callback::BfsCallback,
    io::LockedIO,
    settings::{BfsSettingsBuilder, BfsSettingsProvider, ChunkFilesBehavior, UpdateFilesBehavior},
};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use crate::bfs_3x3_U_F2_R::{coord_cube::CoordCube, transposition_tables::TranspositionTables};

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

struct SettingsProvider;

impl BfsSettingsProvider for SettingsProvider {
    fn chunk_root_idx(&self, chunk_idx: usize) -> usize {
        [0, 1, 2, 3, 1, 2, 3][chunk_idx % 7]
    }

    fn update_files_behavior(&self, _: usize) -> UpdateFilesBehavior {
        UpdateFilesBehavior::CompressAndKeep
    }

    fn chunk_files_behavior(&self, _: usize) -> ChunkFilesBehavior {
        ChunkFilesBehavior::Keep
    }
}

pub fn main() {
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

    let settings = BfsSettingsBuilder::new()
        .threads(48)
        // 4 * 48 chunks
        .chunk_size_bytes(434010150)
        .update_memory(112 * (1 << 30))
        .num_update_blocks(2 * 4 * 48 * 48)
        .capacity_check_frequency(256)
        .initial_states(&[solved])
        .state_size(666639590400)
        .root_directories(&[
            PathBuf::from("/media/ben/drive1/bfs/3x3-U-R-F2/"),
            PathBuf::from("/media/ben/drive2/bfs/3x3-U-R-F2/"),
            PathBuf::from("/media/ben/drive3/bfs/3x3-U-R-F2/"),
            PathBuf::from("/media/ben/drive4/bfs/3x3-U-R-F2/"),
        ])
        .initial_memory_limit(1 << 28)
        .available_disk_space_limit(256 * (1 << 30))
        .update_array_threshold(434010150)
        .use_locked_io(false)
        .sync_filesystem(true)
        .compute_checksums(true)
        .compress_bit_arrays(true)
        .settings_provider(SettingsProvider)
        .build()
        .unwrap();

    let mut cube = CoordCube::new(&transposition_tables);
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
            PathBuf::from("/media/ben/drive1/bfs/3x3-U-R-F2/"),
            PathBuf::from("/media/ben/drive2/bfs/3x3-U-R-F2/"),
            PathBuf::from("/media/ben/drive3/bfs/3x3-U-R-F2/"),
            PathBuf::from("/media/ben/drive4/bfs/3x3-U-R-F2/"),
        ],
    );

    let bfs = Bfs::new(&settings, &locked_io, expander, callback);
    bfs.run();
}
