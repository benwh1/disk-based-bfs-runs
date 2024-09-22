#![allow(non_snake_case)]

mod coord_minx;
mod minx;
mod transposition_tables;

use std::path::PathBuf;

use disk_based_bfs::{
    bfs::Bfs,
    callback::BfsCallback,
    io::LockedIO,
    settings::{BfsSettingsBuilder, BfsSettingsProvider, ChunkFilesBehavior, UpdateFilesBehavior},
};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use crate::{coord_minx::CoordMinx, transposition_tables::TranspositionTables};

#[derive(Clone)]
struct Callback;

impl BfsCallback for Callback {
    fn new_state(&mut self, depth: usize, state: u64) {
        if depth >= 25 {
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

    fn update_files_behavior(&self, depth: usize) -> UpdateFilesBehavior {
        if (21..25).contains(&depth) {
            UpdateFilesBehavior::CompressAndKeep
        } else {
            UpdateFilesBehavior::DontCompress
        }
    }

    fn chunk_files_behavior(&self, depth: usize) -> ChunkFilesBehavior {
        if depth >= 15 {
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
                .unwrap_or_else(|_| "disk_based_bfs=trace,bfs_megaminx_U_R=trace".into()),
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
    let solved = CoordMinx::new(&transposition_tables).encode();

    let settings = BfsSettingsBuilder::new()
        .threads(48)
        // 42 * 48 chunks
        .chunk_size_bytes(496011600)
        .update_memory(112 * (1 << 30))
        .num_update_blocks(2 * 42 * 48 * 48)
        .capacity_check_frequency(256)
        .initial_states(&[solved])
        .state_size(7999675084800)
        .root_directories(&[
            PathBuf::from("/media/ben/drive1/bfs/megaminx-U-R/"),
            PathBuf::from("/media/ben/drive2/bfs/megaminx-U-R/"),
            PathBuf::from("/media/ben/drive3/bfs/megaminx-U-R/"),
            PathBuf::from("/media/ben/drive4/bfs/megaminx-U-R/"),
        ])
        .initial_memory_limit(1 << 34)
        .available_disk_space_limit(256 * (1 << 30))
        .update_array_threshold(496011600)
        .use_locked_io(false)
        .sync_filesystem(true)
        .compute_checksums(true)
        .compress_bit_arrays(true)
        .settings_provider(SettingsProvider)
        .build()
        .unwrap();

    let mut minx = CoordMinx::new(&transposition_tables);
    let expander = move |enc, arr: &mut [_; 8]| {
        minx.decode(enc);
        minx.u();
        arr[0] = minx.encode();
        minx.u();
        arr[1] = minx.encode();
        minx.u();
        arr[2] = minx.encode();
        minx.u();
        arr[3] = minx.encode();
        minx.u();
        minx.r();
        arr[4] = minx.encode();
        minx.r();
        arr[5] = minx.encode();
        minx.r();
        arr[6] = minx.encode();
        minx.r();
        arr[7] = minx.encode();
    };

    let callback = Callback;

    let locked_io = LockedIO::new(
        &settings,
        vec![
            PathBuf::from("/media/ben/drive1/bfs/megaminx-U-R/"),
            PathBuf::from("/media/ben/drive2/bfs/megaminx-U-R/"),
            PathBuf::from("/media/ben/drive3/bfs/megaminx-U-R/"),
            PathBuf::from("/media/ben/drive4/bfs/megaminx-U-R/"),
        ],
    );

    let bfs = Bfs::new(&settings, &locked_io, expander, callback);
    bfs.run();
}
