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

const EXPANSION_NODES_HTM: usize = 6;
const EXPANSION_NODES_QTM: usize = 4;
const EXPANSION_NODES_UTM: usize = 2;

#[derive(Clone)]
struct ExpanderHtm<'a> {
    cube: CoordCube<'a>,
}

impl BfsExpander<EXPANSION_NODES_HTM> for ExpanderHtm<'_> {
    fn expand(&mut self, node: u64, expanded_nodes: &mut [u64; EXPANSION_NODES_HTM]) {
        self.cube.decode(node);
        self.cube.u();
        expanded_nodes[0] = self.cube.encode();
        self.cube.u();
        expanded_nodes[1] = self.cube.encode();
        self.cube.u();
        expanded_nodes[2] = self.cube.encode();
        self.cube.ur();
        expanded_nodes[3] = self.cube.encode();
        self.cube.r();
        expanded_nodes[4] = self.cube.encode();
        self.cube.r();
        expanded_nodes[5] = self.cube.encode();
    }
}

#[derive(Clone)]
struct ExpanderQtm<'a> {
    cube: CoordCube<'a>,
}

impl BfsExpander<EXPANSION_NODES_QTM> for ExpanderQtm<'_> {
    fn expand(&mut self, node: u64, expanded_nodes: &mut [u64; EXPANSION_NODES_QTM]) {
        self.cube.decode(node);
        self.cube.u();
        expanded_nodes[0] = self.cube.encode();
        self.cube.u2();
        expanded_nodes[1] = self.cube.encode();
        self.cube.ur();
        expanded_nodes[2] = self.cube.encode();
        self.cube.r2();
        expanded_nodes[3] = self.cube.encode();
    }
}

#[derive(Clone)]
struct ExpanderUtm<'a> {
    cube: CoordCube<'a>,
}

impl BfsExpander<EXPANSION_NODES_UTM> for ExpanderUtm<'_> {
    fn expand(&mut self, node: u64, expanded_nodes: &mut [u64; EXPANSION_NODES_UTM]) {
        self.cube.decode(node);
        self.cube.up();
        expanded_nodes[0] = self.cube.encode();
        self.cube.urp();
        expanded_nodes[1] = self.cube.encode();
    }
}

macro_rules! define_callback {
    ($name:ident, $depth:expr) => {
        #[derive(Clone)]
        struct $name;

        impl BfsCallback for $name {
            fn new_state(&mut self, depth: usize, state: u64) {
                if depth > $depth {
                    tracing::info!("depth {depth} state {state}");
                }
            }

            fn end_of_chunk(&self, _: usize, _: usize) {}
        }
    };
}

define_callback!(CallbackHtm, 26);
define_callback!(CallbackQtm, 33);
define_callback!(CallbackUtm, 47);

struct Provider;

impl BfsSettingsProvider for Provider {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    Htm,
    Qtm,
    Utm,
}

pub fn run(metric: Metric) {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "disk_based_bfs=trace,bfs_4x4_U_2R=trace".into()),
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

    macro_rules! run {
        ($expander:ident, $callback:ident) => {
            BfsBuilder::new()
                .threads(48)
                // 2 * 48 chunks
                .chunk_size_bytes(357210000)
                .update_memory(112 * (1 << 30))
                .num_update_blocks(2 * 2 * 48 * 48)
                .capacity_check_frequency(256)
                .initial_states(&[CoordCube::new(&transposition_tables).encode()])
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
                .use_compression(true)
                .expander($expander {
                    cube: CoordCube::new(&transposition_tables),
                })
                .callback($callback)
                .settings_provider(Provider)
                .run_no_defaults()
                .unwrap()
        };
    }

    match metric {
        Metric::Htm => run!(ExpanderHtm, CallbackHtm),
        Metric::Qtm => run!(ExpanderQtm, CallbackQtm),
        Metric::Utm => run!(ExpanderUtm, CallbackUtm),
    }
}
