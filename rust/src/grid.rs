use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass, Debug)]
#[inherit(Spatial)]
pub struct Grid {
    #[property(path = "base/width")]
    width: u32,
    #[property(path = "base/height")]
    height: u32,
    cells: Vec<Cell>,
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
}

#[methods]
impl Grid {
    fn new(_owner: &Spatial) -> Self {
        Self {
            width: 0,
            height: 0,
            cells: Vec::new(),
        }
    }

    #[method]
    fn _ready(&mut self, #[base] owner: &Spatial) {
        // TODO: Generate cubes on different z
        for x in 0..self.width {
            for y in 0..self.height {
                let cube_position = Vector3::new(x as f32, y as f32, 0.0);
                let cube = self.create_nav_cube(cube_position);

                let id = format!("{x}{y}");
                self.cells.push(Cell::new(id));
                owner.add_child(cube, false);

                godot_print!("Generated cube with id: {x}{y}");
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
