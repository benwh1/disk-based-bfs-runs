use crate::minx::{Megaminx, CORNERS_SIZE, EP_SIZE};

#[derive(Debug, PartialEq)]
pub struct TranspositionTables {
    pub u_corners: Vec<u32>,
    pub u_edges: Vec<u32>,
    pub u2p_corners: Vec<u32>,
    pub u2p_edges: Vec<u32>,
    pub r_corners: Vec<u32>,
    pub r_edges: Vec<u32>,
    pub r2p_corners: Vec<u32>,
    pub r2p_edges: Vec<u32>,
    pub ur_corners: Vec<u32>,
    pub ur_edges: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_corners = vec![0; CORNERS_SIZE];
        let mut u_edges = vec![0; EP_SIZE];
        let mut u2p_corners = vec![0; CORNERS_SIZE];
        let mut u2p_edges = vec![0; EP_SIZE];
        let mut r_corners = vec![0; CORNERS_SIZE];
        let mut r_edges = vec![0; EP_SIZE];
        let mut r2p_corners = vec![0; CORNERS_SIZE];
        let mut r2p_edges = vec![0; EP_SIZE];
        let mut ur_corners = vec![0; CORNERS_SIZE];
        let mut ur_edges = vec![0; EP_SIZE];

        let mut minx = Megaminx::new();

        for i in 0..CORNERS_SIZE {
            minx.set_corners_coord(i as u32);
            minx.u();
            u_corners[i] = minx.corners_coord();
            minx.r();
            ur_corners[i] = minx.corners_coord();
            minx.r();
            minx.r();
            minx.r();
            minx.r();
            minx.u();
            minx.u();
            u2p_corners[i] = minx.corners_coord();
            minx.u();
            minx.u();
            minx.r();
            r_corners[i] = minx.corners_coord();
            minx.r();
            minx.r();
            r2p_corners[i] = minx.corners_coord();
        }

        for i in 0..EP_SIZE {
            minx.set_ep_coord(i as u32);
            minx.u();
            u_edges[i] = minx.ep_coord();
            minx.r();
            ur_edges[i] = minx.ep_coord();
            minx.r();
            minx.r();
            minx.r();
            minx.r();
            minx.u();
            minx.u();
            u2p_edges[i] = minx.ep_coord();
            minx.u();
            minx.u();
            minx.r();
            r_edges[i] = minx.ep_coord();
            minx.r();
            minx.r();
            r2p_edges[i] = minx.ep_coord();
        }

        Self {
            u_corners,
            u_edges,
            u2p_corners,
            u2p_edges,
            r_corners,
            r_edges,
            r2p_corners,
            r2p_edges,
            ur_corners,
            ur_edges,
        }
    }
}
