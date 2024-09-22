use crate::cube::Cube;

pub struct TranspositionTables {
    pub u_edges: Vec<u32>,
    pub u_centers_corners: Vec<u32>,
    pub u2_edges: Vec<u32>,
    pub u2_centers_corners: Vec<u32>,
    pub up_edges: Vec<u32>,
    pub up_centers_corners: Vec<u32>,
    pub ur_edges: Vec<u32>,
    pub ur_centers_corners: Vec<u32>,
    pub urp_edges: Vec<u32>,
    pub urp_centers_corners: Vec<u32>,
    pub r_edges: Vec<u32>,
    pub r_centers_corners: Vec<u32>,
    pub r2_edges: Vec<u32>,
    pub r2_centers_corners: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_edges = vec![0; 3628800];
        let mut u_centers_corners = vec![0; 75600];
        let mut u2_edges = vec![0; 3628800];
        let mut u2_centers_corners = vec![0; 75600];
        let mut up_edges = vec![0; 3628800];
        let mut up_centers_corners = vec![0; 75600];
        let mut ur_edges = vec![0; 3628800];
        let mut ur_centers_corners = vec![0; 75600];
        let mut urp_edges = vec![0; 3628800];
        let mut urp_centers_corners = vec![0; 75600];
        let mut r_edges = vec![0; 3628800];
        let mut r_centers_corners = vec![0; 75600];
        let mut r2_edges = vec![0; 3628800];
        let mut r2_centers_corners = vec![0; 75600];

        let mut cube = Cube::new();

        for i in 0..3628800 {
            cube.set_edge_coord(i);
            let i = i as usize;
            cube.u();
            u_edges[i] = cube.edge_coord();
            cube.r();
            ur_edges[i] = cube.edge_coord();
            cube.r();
            cube.r();
            urp_edges[i] = cube.edge_coord();
            cube.r();
            cube.u();
            u2_edges[i] = cube.edge_coord();
            cube.u();
            up_edges[i] = cube.edge_coord();
            cube.u();
            cube.r();
            r_edges[i] = cube.edge_coord();
            cube.r();
            r2_edges[i] = cube.edge_coord();
        }

        for i in 0..75600 {
            cube.set_center_corner_coord(i);
            let i = i as usize;
            cube.u();
            u_centers_corners[i] = cube.center_corner_coord();
            cube.r();
            ur_centers_corners[i] = cube.center_corner_coord();
            cube.r();
            cube.r();
            urp_centers_corners[i] = cube.center_corner_coord();
            cube.r();
            cube.u();
            u2_centers_corners[i] = cube.center_corner_coord();
            cube.u();
            up_centers_corners[i] = cube.center_corner_coord();
            cube.u();
            cube.r();
            r_centers_corners[i] = cube.center_corner_coord();
            cube.r();
            r2_centers_corners[i] = cube.center_corner_coord();
        }

        Self {
            u_edges,
            u_centers_corners,
            u2_edges,
            u2_centers_corners,
            up_edges,
            up_centers_corners,
            ur_edges,
            ur_centers_corners,
            urp_edges,
            urp_centers_corners,
            r_edges,
            r_centers_corners,
            r2_edges,
            r2_centers_corners,
        }
    }
}
