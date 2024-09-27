#![allow(non_snake_case)]

mod coord_cube;
mod cube;
mod transposition_tables;

use std::path::PathBuf;

use disk_based_bfs::{
    builder::BfsBuilder,
    callback::BfsCallback,
    expander::BfsExpander,
    provider::{BfsSettingsProvider, ChunkFilesBehavior, UpdateFilesBehavior},
};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use crate::{coord_cube::CoordCube, transposition_tables::TranspositionTables};

const EXPANSION_NODES: usize = 7;

#[derive(Clone)]
struct Expander<'a> {
    cube: CoordCube<'a>,
}

impl BfsExpander<EXPANSION_NODES> for Expander<'_> {
    fn expand(&mut self, node: u64, expanded_nodes: &mut [u64; EXPANSION_NODES]) {
        self.cube.decode(node);
        self.cube.u();
        expanded_nodes[0] = self.cube.encode();
        self.cube.u();
        expanded_nodes[1] = self.cube.encode();
        self.cube.u();
        expanded_nodes[2] = self.cube.encode();
        self.cube.u();
        self.cube.r();
        expanded_nodes[3] = self.cube.encode();
        self.cube.r();
        expanded_nodes[4] = self.cube.encode();
        self.cube.r();
        expanded_nodes[5] = self.cube.encode();
        self.cube.r();
        self.cube.f2();
        expanded_nodes[6] = self.cube.encode();
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

struct Provider;

impl BfsSettingsProvider for Provider {
    fn chunk_root_idx(&self, chunk_idx: usize) -> usize {
        chunk_idx % 4
    }

    fn update_files_behavior(&self, depth: usize) -> UpdateFilesBehavior {
        if depth >= 12 {
            UpdateFilesBehavior::CompressAndKeep
        } else {
            UpdateFilesBehavior::DontCompress
        }
    }

    fn chunk_files_behavior(&self, depth: usize) -> ChunkFilesBehavior {
        if depth >= 12 {
            ChunkFilesBehavior::Keep
        } else {
            ChunkFilesBehavior::Delete
        }
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

    BfsBuilder::new()
        .threads(48)
        // 4 * 48 chunks
        .chunk_size_bytes(434010150)
        .update_memory(112 * (1 << 30))
        .num_update_blocks(2 * 4 * 48 * 48)
        .capacity_check_frequency(256)
        .initial_states(&[CoordCube::new(&transposition_tables).encode()])
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
        .use_compression(true)
        .expander(Expander {
            cube: CoordCube::new(&transposition_tables),
        })
        .callback(Callback)
        .settings_provider(Provider)
        .run_no_defaults()
        .unwrap();
}
