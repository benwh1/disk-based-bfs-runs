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

const EXPANSION_NODES_HTM: usize = 18;
const EXPANSION_NODES_QTM: usize = 12;
const EXPANSION_NODES_UTM: usize = 6;

const CALLBACK_BOUND_HTM: usize = 11;
const CALLBACK_BOUND_QTM: usize = usize::MAX;
const CALLBACK_BOUND_UTM: usize = usize::MAX;

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
        self.cube.l();
        expanded_nodes[3] = self.cube.encode();
        self.cube.l();
        expanded_nodes[4] = self.cube.encode();
        self.cube.l();
        expanded_nodes[5] = self.cube.encode();
        self.cube.l();
        self.cube.f();
        expanded_nodes[6] = self.cube.encode();
        self.cube.f();
        expanded_nodes[7] = self.cube.encode();
        self.cube.f();
        expanded_nodes[8] = self.cube.encode();
        self.cube.f();
        self.cube.r();
        expanded_nodes[9] = self.cube.encode();
        self.cube.r();
        expanded_nodes[10] = self.cube.encode();
        self.cube.r();
        expanded_nodes[11] = self.cube.encode();
        self.cube.r();
        self.cube.b();
        expanded_nodes[12] = self.cube.encode();
        self.cube.b();
        expanded_nodes[13] = self.cube.encode();
        self.cube.b();
        expanded_nodes[14] = self.cube.encode();
        self.cube.b();
        self.cube.d();
        expanded_nodes[15] = self.cube.encode();
        self.cube.d();
        expanded_nodes[16] = self.cube.encode();
        self.cube.d();
        expanded_nodes[17] = self.cube.encode();
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
        self.cube.l();
        expanded_nodes[2] = self.cube.encode();
        self.cube.l();
        self.cube.l();
        expanded_nodes[3] = self.cube.encode();
        self.cube.l();
        self.cube.f();
        expanded_nodes[4] = self.cube.encode();
        self.cube.f();
        self.cube.f();
        expanded_nodes[5] = self.cube.encode();
        self.cube.f();
        self.cube.r();
        expanded_nodes[6] = self.cube.encode();
        self.cube.r();
        self.cube.r();
        expanded_nodes[7] = self.cube.encode();
        self.cube.r();
        self.cube.b();
        expanded_nodes[8] = self.cube.encode();
        self.cube.b();
        self.cube.b();
        expanded_nodes[9] = self.cube.encode();
        self.cube.b();
        self.cube.d();
        expanded_nodes[10] = self.cube.encode();
        self.cube.d();
        self.cube.d();
        expanded_nodes[11] = self.cube.encode();
    }
}

#[derive(Clone)]
struct ExpanderUtm<'a> {
    cube: CoordCube<'a>,
}

impl BfsExpander<EXPANSION_NODES_UTM> for ExpanderUtm<'_> {
    fn expand(&mut self, node: u64, expanded_nodes: &mut [u64; EXPANSION_NODES_UTM]) {
        self.cube.decode(node);
        self.cube.u();
        self.cube.u();
        self.cube.u();
        expanded_nodes[0] = self.cube.encode();
        self.cube.u();
        self.cube.l();
        self.cube.l();
        self.cube.l();
        expanded_nodes[1] = self.cube.encode();
        self.cube.l();
        self.cube.f();
        self.cube.f();
        self.cube.f();
        expanded_nodes[2] = self.cube.encode();
        self.cube.f();
        self.cube.r();
        self.cube.r();
        self.cube.r();
        expanded_nodes[3] = self.cube.encode();
        self.cube.r();
        self.cube.b();
        self.cube.b();
        self.cube.b();
        expanded_nodes[4] = self.cube.encode();
        self.cube.b();
        self.cube.d();
        self.cube.d();
        self.cube.d();
        expanded_nodes[5] = self.cube.encode();
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

struct Provider;

impl BfsSettingsProvider for Provider {
    fn chunk_root_idx(&self, _: usize) -> usize {
        0
    }

    fn update_files_behavior(&self, _: usize) -> UpdateFilesBehavior {
        UpdateFilesBehavior::DontMerge
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
                .unwrap_or_else(|_| "disk_based_bfs=trace,bfs_3x3_ep=trace".into()),
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
        ($expander:ident, $callback_bound:expr) => {
            BfsBuilder::new()
                .threads(1)
                // 16 chunks
                .chunk_size_bytes(3742200)
                .update_memory(2 * (1 << 30))
                .num_update_blocks(2 * 16)
                .capacity_check_frequency(256)
                .initial_states(&[CoordCube::new(&transposition_tables).encode()])
                .state_size(479001600)
                .root_directories(&[PathBuf::from(
                    "/home/ben/programs/rust/disk-based-bfs-runs/runs/bfs-3x3-ep/htm/",
                )])
                .initial_memory_limit(1 << 24)
                .available_disk_space_limit(4 * (1 << 30))
                .update_array_threshold(3742200)
                .use_locked_io(false)
                .sync_filesystem(false)
                .compute_checksums(true)
                .use_compression(true)
                .expander($expander {
                    cube: CoordCube::new(&transposition_tables),
                })
                .callback(Callback($callback_bound))
                .settings_provider(Provider)
                .run_no_defaults()
                .unwrap()
        };
    }

    match metric {
        Metric::Htm => run!(ExpanderHtm, CALLBACK_BOUND_HTM),
        Metric::Qtm => run!(ExpanderQtm, CALLBACK_BOUND_QTM),
        Metric::Utm => run!(ExpanderUtm, CALLBACK_BOUND_UTM),
    }
}
