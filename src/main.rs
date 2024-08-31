use std::path::PathBuf;

use disk_based_bfs::{
    bfs::Bfs, callback::BfsCallback, chunk_allocator::ChunkAllocator, io::LockedIO,
    settings::BfsSettingsBuilder,
};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use crate::minx::{CoordMinx, TranspositionTables};

pub mod minx;

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

struct ChunkAlloc;

impl ChunkAllocator for ChunkAlloc {
    fn chunk_root_idx(&self, chunk_idx: usize) -> usize {
        [0, 1, 2, 3, 1, 2, 3][chunk_idx % 7]
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
    let solved = CoordMinx::new(&transposition_tables).encode() as u64;

    let mut minx = CoordMinx::new(&transposition_tables);
    let settings = BfsSettingsBuilder::new()
        .threads(48)
        // 42 * 48 chunks
        .chunk_size_bytes(496011600)
        .update_memory(1 << 37)
        .capacity_check_frequency(256)
        .initial_states(&[solved])
        .state_size(7999675084800)
        .root_directories(&[
            PathBuf::from("/media/ben/drive1/bfs/megaminx-U-R/"),
            PathBuf::from("/media/ben/drive2/bfs/megaminx-U-R/"),
            PathBuf::from("/media/ben/drive3/bfs/megaminx-U-R/"),
            PathBuf::from("/media/ben/drive4/bfs/megaminx-U-R/"),
        ])
        .chunk_allocator(ChunkAlloc)
        .initial_memory_limit(1 << 28)
        .update_files_compression_threshold(3 * (1 << 32))
        .buf_io_capacity(1 << 23)
        .use_locked_io(true)
        .sync_filesystem(true)
        .compress_update_files_from_depth(Some(19))
        .build()
        .unwrap();

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

fn main() {
    run();
}
