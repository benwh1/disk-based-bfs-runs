#![allow(non_snake_case)]

mod coord_minx;
mod minx;
mod transposition_tables;

use std::path::PathBuf;

use disk_based_bfs::{
    builder::BfsBuilder,
    callback::BfsCallback,
    expander::BfsExpander,
    provider::{BfsSettingsProvider, ChunkFilesBehavior, UpdateFilesBehavior},
};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use crate::{coord_minx::CoordMinx, transposition_tables::TranspositionTables};

const EXPANSION_NODES: usize = 8;

#[derive(Clone)]
struct Expander<'a> {
    minx: CoordMinx<'a>,
}

impl BfsExpander<EXPANSION_NODES> for Expander<'_> {
    fn expand(&mut self, node: u64, expanded_nodes: &mut [u64; EXPANSION_NODES]) {
        self.minx.decode(node);
        self.minx.u();
        expanded_nodes[0] = self.minx.encode();
        self.minx.u();
        expanded_nodes[1] = self.minx.encode();
        self.minx.u();
        expanded_nodes[2] = self.minx.encode();
        self.minx.u();
        expanded_nodes[3] = self.minx.encode();
        self.minx.u();
        self.minx.r();
        expanded_nodes[4] = self.minx.encode();
        self.minx.r();
        expanded_nodes[5] = self.minx.encode();
        self.minx.r();
        expanded_nodes[6] = self.minx.encode();
        self.minx.r();
        expanded_nodes[7] = self.minx.encode();
    }
}

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

struct Provider;

impl BfsSettingsProvider for Provider {
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

    BfsBuilder::new()
        .threads(48)
        // 42 * 48 chunks
        .chunk_size_bytes(496011600)
        .update_memory(112 * (1 << 30))
        .num_update_blocks(2 * 42 * 48 * 48)
        .capacity_check_frequency(256)
        .initial_states(&[CoordMinx::new(&transposition_tables).encode()])
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
        .use_compression(true)
        .expander(Expander {
            minx: CoordMinx::new(&transposition_tables),
        })
        .callback(Callback)
        .settings_provider(Provider)
        .run_no_defaults()
        .unwrap();
}
