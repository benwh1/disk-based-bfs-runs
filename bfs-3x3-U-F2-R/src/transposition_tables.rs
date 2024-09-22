use crate::cube::{Cube, CORNERS_SIZE, EP_SIZE};

pub struct TranspositionTables {
    pub u_edges: Vec<u32>,
    pub u_corners: Vec<u32>,
    pub r_edges: Vec<u32>,
    pub r_corners: Vec<u32>,
    // We need two sets of tables for f2 because parity determines whether we swap pieces 7 and 8,
    // and those two pieces are on the F face but not in U or R
    pub f2_edges_even: Vec<u32>,
    pub f2_edges_odd: Vec<u32>,
    pub f2_corners: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_edges = vec![0; EP_SIZE as usize];
        let mut u_corners = vec![0; CORNERS_SIZE as usize];
        let mut r_edges = vec![0; EP_SIZE as usize];
        let mut r_corners = vec![0; CORNERS_SIZE as usize];
        let mut f2_edges_even = vec![0; EP_SIZE as usize];
        let mut f2_edges_odd = vec![0; EP_SIZE as usize];
        let mut f2_corners = vec![0; CORNERS_SIZE as usize];

        let mut cube = Cube::new();

        for i in 0..CORNERS_SIZE as usize {
            cube.set_corners_coord(i as u32);
            cube.u();
            u_corners[i] = cube.corners_coord();
            cube.up();
            cube.r();
            r_corners[i] = cube.corners_coord();
            cube.rp();
            cube.f2();
            f2_corners[i] = cube.corners_coord();
        }

        cube.is_even_perm = true;

        for i in 0..EP_SIZE as usize {
            cube.set_ep_coord(i as u32);
            cube.u();
            u_edges[i] = cube.ep_coord();
            cube.up();
            cube.r();
            r_edges[i] = cube.ep_coord();
            cube.rp();
            cube.f2();
            f2_edges_even[i] = cube.ep_coord();
        }

        cube.is_even_perm = false;

        for i in 0..EP_SIZE as usize {
            cube.set_ep_coord(i as u32);
            cube.f2();
            f2_edges_odd[i] = cube.ep_coord();
        }

        Self {
            u_edges,
            u_corners,
            r_edges,
            r_corners,
            f2_edges_even,
            f2_edges_odd,
            f2_corners,
        }
    }
}
