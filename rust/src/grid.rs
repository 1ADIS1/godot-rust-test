use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass, Debug)]
#[inherit(Spatial)]
// TODO: Separate Grid into the default struct, not Godot one
pub struct Grid {
    #[property(path = "base/width")]
    width: u32,
    #[property(path = "base/height")]
    height: u32,
    #[property(path = "base/depth")]
    depth: u32,
    cells: Vec<Cell>,

    #[property(path = "base/offset", default = 1.0)]
    offset: f32,

    // Size of cell in world coordinates
    #[property(path = "base/cell_size_x", default = 2)]
    x_cell_size: u32,
    #[property(path = "base/cell_size_z", default = 2)]
    z_cell_size: u32,
    #[property(path = "base/cell_size_y", default = 2)]
    y_cell_size: u32,
}

impl Grid {
    pub fn create_nav_cube(&self, position: Vector3) -> Ref<MeshInstance, Unique> {
        let cube = MeshInstance::new();
        //TODO: don't create new instances of mesh, but use one already created
        cube.set_mesh(CubeMesh::new());
        cube.set_translation(position);

        return cube;
    }

    pub fn get_cube(&self, position: Vector3) -> Option<String> {
        todo!()
    }

    /*
    Takes coordinates of cube in grid and converts to world coordinates
    */
    // TODO: map into different z
    pub fn map_to_world_coordinates(&self, x: u32, z: u32) -> Vector3 {
        Vector3::new(
            (x * self.x_cell_size) as f32,
            self.y_cell_size as f32,
            (z * self.z_cell_size) as f32,
        )
    }

    /**
     * Converts world cell coordinates to grid coordinates
     */
    // TODO: make for 3d coordinates
    pub fn map_to_grid_coordinates(&self, position: Vector3) -> (u32, u32) {
        (
            (position.x / self.x_cell_size as f32) as u32,
            (position.z / self.z_cell_size as f32) as u32,
        )
    }

    /**
     * Takes grid coordinates and returns id of the object
     */
    pub fn get_id(&self, x: u32, z: u32) -> &str {
        todo!()
    }

    pub fn in_bounds(&self) {}
}

#[methods]
impl Grid {
    fn new(_owner: &Spatial) -> Self {
        Self {
            width: 0,
            height: 0,
            depth: 0,
            cells: Vec::new(),
            offset: 1.0,
            x_cell_size: 2,
            z_cell_size: 2,
            y_cell_size: 2,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] owner: &Spatial) {
        // self.cells.get(index)

        // TODO: Generate cubes on different z
        for x in 0..self.width {
            for z in 0..self.height {
                let cube_position = self.map_to_world_coordinates(x, z);
                let cube = self.create_nav_cube(cube_position);

                // let id = format!("{x}{y}");
                // self.cells.push(Cell::new(id));
                owner.add_child(cube, false);

                godot_print!("Generated cube with id: {x}{z}");
            }
        }
    }
}

#[derive(Debug)]
//TODO: store id of the cube
struct Cell {
    cube_id: Option<String>,
}

impl Cell {
    fn new(cube_id: String) -> Self {
        Self {
            cube_id: Some(cube_id),
        }
    }
}
