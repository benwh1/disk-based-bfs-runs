use crate::bfs_4x4_U_2R::cube::Cube;

pub struct TranspositionTables {
    pub u_edges: Vec<u32>,
    pub u_centers_corners: Vec<u32>,
    pub u2_edges: Vec<u32>,
    pub u2_centers_corners: Vec<u32>,
    pub ur_edges: Vec<u32>,
    pub ur_centers_corners: Vec<u32>,
    pub r2_edges: Vec<u32>,
    pub r2_centers_corners: Vec<u32>,
}

impl TranspositionTables {
    pub fn new() -> Self {
        let mut u_edges = vec![0; 3628800];
        let mut u_centers_corners = vec![0; 75600];
        let mut u2_edges = vec![0; 3628800];
        let mut u2_centers_corners = vec![0; 75600];
        let mut ur_edges = vec![0; 3628800];
        let mut ur_centers_corners = vec![0; 75600];
        let mut r2_edges = vec![0; 3628800];
        let mut r2_centers_corners = vec![0; 75600];

        let mut cube = Cube::new();

        for i in 0..3628800 {
            cube.set_edge_coord(i);
            let i = i as usize;
            cube.u();
            u_edges[i] = cube.edge_coord();
            cube.u();
            u2_edges[i] = cube.edge_coord();
            cube.uinv();
            cube.r();
            ur_edges[i] = cube.edge_coord();
            cube.rinv();
            cube.uinv();
            cube.r();
            cube.r();
            r2_edges[i] = cube.edge_coord();
        }

        for i in 0..75600 {
            cube.set_center_corner_coord(i);
            let i = i as usize;
            cube.u();
            u_centers_corners[i] = cube.center_corner_coord();
            cube.u();
            u2_centers_corners[i] = cube.center_corner_coord();
            cube.uinv();
            cube.r();
            ur_centers_corners[i] = cube.center_corner_coord();
            cube.rinv();
            cube.uinv();
            cube.r();
            cube.r();
            r2_centers_corners[i] = cube.center_corner_coord();
        }

        Self {
            u_edges,
            u_centers_corners,
            u2_edges,
            u2_centers_corners,
            ur_edges,
            ur_centers_corners,
            r2_edges,
            r2_centers_corners,
        }
    }
}
