pub mod minx;

use std::path::PathBuf;

use disk_based_bfs::{
    builder::BfsBuilder,
    callback::BfsCallback,
    expander::BfsExpander,
    provider::{BfsSettingsProvider, ChunkFilesBehavior, UpdateFilesBehavior},
};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::minx::Kilominx;

const EXPANSION_NODES_HTM: usize = 24;
const CALLBACK_BOUND_HTM: usize = 12;
const PROVIDER_BOUND_HTM: usize = 13;

#[derive(Clone)]
struct ExpanderHtm {
    minx: Kilominx,
}

impl BfsExpander<EXPANSION_NODES_HTM> for ExpanderHtm {
    fn expand(&mut self, node: u64, expanded_nodes: &mut [u64; EXPANSION_NODES_HTM]) {
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
        self.minx.l();
        expanded_nodes[4] = self.minx.encode();
        self.minx.l();
        expanded_nodes[5] = self.minx.encode();
        self.minx.l();
        expanded_nodes[6] = self.minx.encode();
        self.minx.l();
        expanded_nodes[7] = self.minx.encode();
        self.minx.l();
        self.minx.f();
        expanded_nodes[8] = self.minx.encode();
        self.minx.f();
        expanded_nodes[9] = self.minx.encode();
        self.minx.f();
        expanded_nodes[10] = self.minx.encode();
        self.minx.f();
        expanded_nodes[11] = self.minx.encode();
        self.minx.f();
        self.minx.r();
        expanded_nodes[12] = self.minx.encode();
        self.minx.r();
        expanded_nodes[13] = self.minx.encode();
        self.minx.r();
        expanded_nodes[14] = self.minx.encode();
        self.minx.r();
        expanded_nodes[15] = self.minx.encode();
        self.minx.r();
        self.minx.br();
        expanded_nodes[16] = self.minx.encode();
        self.minx.br();
        expanded_nodes[17] = self.minx.encode();
        self.minx.br();
        expanded_nodes[18] = self.minx.encode();
        self.minx.br();
        expanded_nodes[19] = self.minx.encode();
        self.minx.br();
        self.minx.bl();
        expanded_nodes[20] = self.minx.encode();
        self.minx.bl();
        expanded_nodes[21] = self.minx.encode();
        self.minx.bl();
        expanded_nodes[22] = self.minx.encode();
        self.minx.bl();
        expanded_nodes[23] = self.minx.encode();
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
        [0, 1, 2][chunk_idx % 3]
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

pub fn run(metric: Metric) {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "disk_based_bfs=trace,bfs_kilominx_6gen_permutation=trace".into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(false)
                .with_thread_names(true)
                .with_line_number(true),
        )
        .init();

    macro_rules! run {
        ($expander:ident, $callback_bound:expr, $provider_bound:expr) => {
            BfsBuilder::new()
                .threads(48)
                // 4 * 48 chunks
                .chunk_size_bytes(425675250)
                .update_memory(80 * (1 << 30))
                .num_update_blocks(2 * 4 * 48 * 48)
                .capacity_check_frequency(256)
                .initial_states(&[Kilominx::new().encode()])
                .state_size(653837184000)
                .root_directories(&[
                    PathBuf::from("/media/ben/drive2/bfs/megaminx-U-R/"),
                    PathBuf::from("/media/ben/drive3/bfs/megaminx-U-R/"),
                    PathBuf::from("/media/ben/drive4/bfs/megaminx-U-R/"),
                ])
                .initial_memory_limit(1 << 30)
                .available_disk_space_limit(256 * (1 << 30))
                .update_array_threshold(425675250)
                .use_locked_io(false)
                .sync_filesystem(true)
                .compute_checksums(true)
                .use_compression(true)
                .expander($expander {
                    minx: Kilominx::new(),
                })
                .callback(Callback($callback_bound))
                .settings_provider(Provider($provider_bound))
                .run_no_defaults()
                .unwrap()
        };
    }

    match metric {
        Metric::Htm => run!(ExpanderHtm, CALLBACK_BOUND_HTM, PROVIDER_BOUND_HTM),
        Metric::Qtm => todo!(),
    }
}
