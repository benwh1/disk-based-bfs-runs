use crate::cube::{Cube, CORNERS_SIZE, EDGES_SIZE};

pub struct TranspositionTables {
    pub u_edges: Vec<u32>,
    pub u_corners: Vec<u32>,
    pub l_edges: Vec<u32>,
    pub l_corners: Vec<u32>,
    pub f_edges: Vec<u32>,
    pub f_corners: Vec<u32>,
    pub r_edges: Vec<u32>,
    pub r_corners: Vec<u32>,
    pub b_edges: Vec<u32>,
    pub b_corners: Vec<u32>,
    pub d_edges: Vec<u32>,
    pub d_corners: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_edges = vec![0; EDGES_SIZE as usize];
        let mut u_corners = vec![0; CORNERS_SIZE as usize];
        let mut l_edges = vec![0; EDGES_SIZE as usize];
        let mut l_corners = vec![0; CORNERS_SIZE as usize];
        let mut f_edges = vec![0; EDGES_SIZE as usize];
        let mut f_corners = vec![0; CORNERS_SIZE as usize];
        let mut r_edges = vec![0; EDGES_SIZE as usize];
        let mut r_corners = vec![0; CORNERS_SIZE as usize];
        let mut b_edges = vec![0; EDGES_SIZE as usize];
        let mut b_corners = vec![0; CORNERS_SIZE as usize];
        let mut d_edges = vec![0; EDGES_SIZE as usize];
        let mut d_corners = vec![0; CORNERS_SIZE as usize];

        let mut cube = Cube::new();

        for i in 0..EDGES_SIZE as usize {
            cube.set_edges_coord(i as u32);
            cube.u();
            u_edges[i] = cube.edges_coord();
            cube.up();
            cube.l();
            l_edges[i] = cube.edges_coord();
            cube.lp();
            cube.f();
            f_edges[i] = cube.edges_coord();
            cube.fp();
            cube.r();
            r_edges[i] = cube.edges_coord();
            cube.rp();
            cube.b();
            b_edges[i] = cube.edges_coord();
            cube.bp();
            cube.d();
            d_edges[i] = cube.edges_coord();
        }

        for i in 0..CORNERS_SIZE as usize {
            cube.set_corners_coord(i as u32);
            cube.u();
            u_corners[i] = cube.corners_coord();
            cube.up();
            cube.l();
            l_corners[i] = cube.corners_coord();
            cube.lp();
            cube.f();
            f_corners[i] = cube.corners_coord();
            cube.fp();
            cube.r();
            r_corners[i] = cube.corners_coord();
            cube.rp();
            cube.b();
            b_corners[i] = cube.corners_coord();
            cube.bp();
            cube.d();
            d_corners[i] = cube.corners_coord();
        }

        Self {
            u_edges,
            u_corners,
            l_edges,
            l_corners,
            f_edges,
            f_corners,
            r_edges,
            r_corners,
            b_edges,
            b_corners,
            d_edges,
            d_corners,
        }
    }
}
