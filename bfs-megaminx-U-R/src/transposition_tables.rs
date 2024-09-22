use crate::minx::{Megaminx, CORNERS_SIZE, EP_SIZE};

#[derive(Debug, PartialEq)]
pub struct TranspositionTables {
    pub u_corners: Vec<u32>,
    pub u_edges: Vec<u32>,
    pub r_corners: Vec<u32>,
    pub r_edges: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_corners = vec![0; CORNERS_SIZE];
        let mut u_edges = vec![0; EP_SIZE];
        let mut r_corners = vec![0; CORNERS_SIZE];
        let mut r_edges = vec![0; EP_SIZE];

        let mut minx = Megaminx::new();

        for i in 0..CORNERS_SIZE {
            minx.set_corners_coord(i as u32);
            minx.u();
            u_corners[i] = minx.corners_coord();
            (0..4).for_each(|_| minx.u());
            minx.r();
            r_corners[i] = minx.corners_coord();
        }

        for i in 0..EP_SIZE {
            minx.set_ep_coord(i as u32);
            minx.u();
            u_edges[i] = minx.ep_coord();
            (0..4).for_each(|_| minx.u());
            minx.r();
            r_edges[i] = minx.ep_coord();
        }

        Self {
            u_corners,
            u_edges,
            r_corners,
            r_edges,
        }
    }
}
