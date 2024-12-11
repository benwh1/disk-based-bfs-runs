#![allow(non_snake_case)]

use std::path::PathBuf;

use disk_based_bfs::{
    builder::BfsBuilder,
    callback::BfsCallback,
    expander::BfsExpander,
    provider::{BfsSettingsProvider, ChunkFilesBehavior, UpdateFilesBehavior},
};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use crate::{coord_cube::CoordCube, transposition_tables::TranspositionTables};

mod coord_cube;
mod cube;
mod transposition_tables;

const EXPANSION_NODES_HTM: usize = 6;
const EXPANSION_NODES_QTM: usize = 4;
const EXPANSION_NODES_U_R_RW_HTM: usize = 9;

const CALLBACK_BOUND_HTM: usize = 30;
const CALLBACK_BOUND_QTM: usize = 37;
const CALLBACK_BOUND_U_R_RW_HTM: usize = 22;

const PROVIDER_BOUND_HTM: usize = 20;
const PROVIDER_BOUND_QTM: usize = 25;
const PROVIDER_BOUND_U_R_RW_HTM: usize = 16;

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
        self.cube.u();
        self.cube.rw();
        expanded_nodes[3] = self.cube.encode();
        self.cube.rw();
        expanded_nodes[4] = self.cube.encode();
        self.cube.rw();
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
        self.cube.u();
        self.cube.u();
        expanded_nodes[1] = self.cube.encode();
        self.cube.u();
        self.cube.rw();
        expanded_nodes[2] = self.cube.encode();
        self.cube.rw();
        self.cube.rw();
        expanded_nodes[3] = self.cube.encode();
    }
}

#[derive(Clone)]
struct ExpanderURRwHtm<'a> {
    cube: CoordCube<'a>,
}

impl BfsExpander<EXPANSION_NODES_U_R_RW_HTM> for ExpanderURRwHtm<'_> {
    fn expand(&mut self, node: u64, expanded_nodes: &mut [u64; EXPANSION_NODES_U_R_RW_HTM]) {
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
        self.cube.rw();
        expanded_nodes[6] = self.cube.encode();
        self.cube.rw();
        expanded_nodes[7] = self.cube.encode();
        self.cube.rw();
        expanded_nodes[8] = self.cube.encode();
    }
}

#[derive(Clone)]
struct Callback(usize);

impl BfsCallback for Callback {
    fn new_state(&mut self, depth: usize, state: u64) {
        if depth >= self.0 {
            tracing::info!("depth {depth} state {state}");
        }
    }

    fn end_of_chunk(&self, _: usize, _: usize) {}
}

struct Provider(usize);

impl BfsSettingsProvider for Provider {
    fn chunk_root_idx(&self, chunk_idx: usize) -> usize {
        [0, 1, 2, 3, 1, 2, 3][chunk_idx % 7]
    }

    fn update_files_behavior(&self, depth: usize) -> UpdateFilesBehavior {
        if depth >= self.0 {
            UpdateFilesBehavior::MergeAndKeep
        } else {
            UpdateFilesBehavior::DontMerge
        }
    }

    fn chunk_files_behavior(&self, depth: usize) -> ChunkFilesBehavior {
        if depth >= self.0 {
            ChunkFilesBehavior::Keep
        } else {
            ChunkFilesBehavior::Delete
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    Htm,
    Qtm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Generators {
    UR,
    URRw,
}

pub fn run(metric: Metric, generators: Generators) {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "disk_based_bfs=trace,bfs_3x3_U_r=trace".into()),
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
        ($expander:ident, $callback_bound:expr, $provider_bound:expr) => {
            BfsBuilder::new()
                .threads(48)
                .chunk_size_bytes(529079040)
                .update_memory(112 * (1 << 30))
                .num_update_blocks(2 * 48 * 1280)
                .capacity_check_frequency(256)
                .initial_states(&[CoordCube::new(&transposition_tables).encode()])
                .state_size(5417769369600)
                .root_directories(&[
                    PathBuf::from("/media/ben/drive1/bfs/3x3-U-r/"),
                    PathBuf::from("/media/ben/drive2/bfs/3x3-U-r/"),
                    PathBuf::from("/media/ben/drive3/bfs/3x3-U-r/"),
                    PathBuf::from("/media/ben/drive4/bfs/3x3-U-r/"),
                ])
                .initial_memory_limit(1 << 34)
                .available_disk_space_limit(256 * (1 << 30))
                .update_array_threshold(529079040)
                .use_locked_io(false)
                .sync_filesystem(true)
                .compute_checksums(true)
                .use_compression(true)
                .expander($expander {
                    cube: CoordCube::new(&transposition_tables),
                })
                .callback(Callback($callback_bound))
                .settings_provider(Provider($provider_bound))
                .run_no_defaults()
                .unwrap()
        };
    }

    match (metric, generators) {
        (Metric::Htm, Generators::UR) => run!(ExpanderHtm, CALLBACK_BOUND_HTM, PROVIDER_BOUND_HTM),
        (Metric::Qtm, Generators::UR) => run!(ExpanderQtm, CALLBACK_BOUND_QTM, PROVIDER_BOUND_QTM),
        (Metric::Htm, Generators::URRw) => {
            run!(
                ExpanderURRwHtm,
                CALLBACK_BOUND_U_R_RW_HTM,
                PROVIDER_BOUND_U_R_RW_HTM
            );
        }
        (Metric::Qtm, Generators::URRw) => todo!(),
    }
}
