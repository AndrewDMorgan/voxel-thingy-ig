use crate::{CELL_SIZE, MAXIMUM_WINDOW_HEIGHT, MAXIMUM_WINDOW_WIDTH, WINDOW_START_HEIGHT, WINDOW_START_WIDTH};
use crate::shader_handling::{Float2, Float4, Pipeline, ShaderHandler, Uint4, Vertex};
use std::f32::consts::PI;

pub struct Mesh {
    pub mutated: bool,
    pub vertices_original: Vec<Vertex>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Uint4>,
    pub normals: Vec<Float4>,
    pub binned_indices: Vec<u32>,
}

pub struct Float3x3 {
    table: [f32; 9],
}

impl Float3x3 {
    pub fn new(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32,
    ) -> Self {
        Float3x3 {
            table: [
                m00, m01, m02,
                m10, m11, m12,
                m20, m21, m22,
            ],
        }
    }
    
    pub fn mult_3x3(&self, other: &Float3x3) -> Float3x3 {
        Float3x3::new(
            (self.table[0] * other.table[0] + self.table[1] * other.table[3] + self.table[2] * other.table[6]),
            (self.table[0] * other.table[1] + self.table[1] * other.table[4] + self.table[2] * other.table[7]),
            (self.table[0] * other.table[2] + self.table[1] * other.table[5] + self.table[2] * other.table[8]),
            
            (self.table[3] * other.table[0] + self.table[4] * other.table[3] + self.table[5] * other.table[6]),
            (self.table[3] * other.table[1] + self.table[4] * other.table[4] + self.table[5] * other.table[7]),
            (self.table[3] * other.table[2] + self.table[4] * other.table[5] + self.table[5] * other.table[8]),
            
            (self.table[6] * other.table[0] + self.table[7] * other.table[3] + self.table[8] * other.table[6]),
            (self.table[6] * other.table[1] + self.table[7] * other.table[4] + self.table[8] * other.table[7]),
            (self.table[6] * other.table[2] + self.table[7] * other.table[5] + self.table[8] * other.table[8]),
        )
    }
    
    pub fn mult_3x1(&self, vec: &Float4) -> Float4 {
        Float4 {
            x: self.table[0] * vec.x + self.table[1] * vec.y + self.table[2] * vec.z,
            y: self.table[3] * vec.x + self.table[4] * vec.y + self.table[5] * vec.z,
            z: self.table[6] * vec.x + self.table[7] * vec.y + self.table[8] * vec.z,
            w: 0.0,
        }
    }
}

pub fn rotate(vertex: Float4, rotation: &Float4) -> Float4 {
    let sin_x = f32::sin(rotation.x);
    let sin_y = f32::sin(rotation.y);
    let sin_z = f32::sin(rotation.z);
    let cos_x = f32::cos(rotation.x);
    let cos_y = f32::cos(rotation.y);
    let cos_z = f32::cos(rotation.z);
    // rotation matrices
    let rx = Float3x3::new(
        1.0, 0.0   ,  0.0   ,
        0.0, cos_x , -sin_x ,
        0.0, sin_x ,  cos_x
    );
    let ry = Float3x3::new(
        cos_y , 0.0, sin_y,
        0.0   , 1.0, 0.0  ,
        -sin_y , 0.0, cos_y
    );
    let rz = Float3x3::new(
        cos_z , -sin_z , 0.0,
        sin_z ,  cos_z , 0.0,
        0.0   ,  0.0   , 1.0
    );
    let r = rx.mult_3x3(&ry).mult_3x3(&rz);
    r.mult_3x1(&vertex)
}

impl Mesh {
    pub fn check_remesh(&mut self, shader_handler: &mut ShaderHandler, window_size: (u32, u32), camera_position: Float4, camera_rotation: Float4) {
        if !self.mutated { return; }
        self.mutated = false;
        
        let start = std::time::Instant::now();
        
        // replacing vertices with the transformations of the original vertices
        //self.vertices.reserve(self.vertices_original.len().saturating_sub(self.vertices.len()));
        self.vertices = vec![Vertex::default(); self.vertices_original.len()];
        self.vertices[0..self.vertices_original.len()].fill(Vertex::default());
        for (i, vertex) in self.vertices_original.iter().enumerate() {
            let mut vert = rotate(Float4::new(
                vertex.position.x - camera_position.x,
                vertex.position.y - camera_position.y,
                vertex.position.z - camera_position.z,
                0.0,
            ), &camera_rotation);
            vert.x += window_size.0 as f32 / 2.0;
            vert.y += window_size.1 as f32 / 2.0;
            self.vertices[i] = Vertex::new(vert, Float2::new(vertex.uv.x, vertex.uv.y));
        }
        
        for i in 0..self.binned_indices.len() / 64 {
            self.binned_indices[i * 64] = 0;
        }
        
        let window_width = (window_size.0 as f32 / CELL_SIZE as f32).ceil() as usize * 64;
        for tri_index in 0..self.indices.len() {
            // finding all bounding box cells it falls within
            // getting the bounding box
            let v1 = &self.vertices[self.indices[tri_index].x as usize];
            let v2 = &self.vertices[self.indices[tri_index].y as usize];
            let v3 = &self.vertices[self.indices[tri_index].z as usize];
            let min_x = v1.position.x.min(v2.position.x.min(v3.position.x));
            let max_x = v1.position.x.max(v2.position.x.max(v3.position.x));
            let min_y = v1.position.y.min(v2.position.y.min(v3.position.y));
            let max_y = v1.position.y.max(v2.position.y.max(v3.position.y));
            let min_x_bin = (min_x / CELL_SIZE as f32).floor() as u32;
            let max_x_bin = (max_x / CELL_SIZE as f32).ceil()  as u32;
            let min_y_bin = (min_y / CELL_SIZE as f32).floor() as u32;
            let max_y_bin = (max_y / CELL_SIZE as f32).ceil()  as u32;
            
            for x in min_x_bin..max_x_bin {
                let x_coord = x as usize * 64;
                for y in min_y_bin..max_y_bin {
                    // placing it into the bin
                    let bin_index_base = y as usize * window_width + x_coord;
                    self.binned_indices[bin_index_base] += 1;
                    let current_count = self.binned_indices[bin_index_base];
                    if current_count >= 63 { continue; }
                    self.binned_indices[bin_index_base + current_count as usize] = tri_index as u32;
                }
            }
        }
        
        //shader_handler.get_shader().update_buffer(3,  camera_position                 ).unwrap();
        let new_camera_rotation = Float4::new(
            camera_rotation.x - PI * 0.0,
            camera_rotation.y - PI * 0.0,
            camera_rotation.z - PI * 0.0,
            0.0
        ).normalized();
        shader_handler.get_shader().update_buffer(4,  new_camera_rotation             ).unwrap();
        shader_handler.get_shader().update_buffer_slice(5, self.vertices.as_slice()   ).unwrap();
        shader_handler.get_shader().update_buffer_slice(6, self.normals.as_slice()    ).unwrap();
        shader_handler.get_shader().update_buffer_slice(7, self.indices.as_slice()    ).unwrap();
        shader_handler.get_shader().update_buffer_slice(8, &self.binned_indices[0..((window_size.0 as f32 / CELL_SIZE as f32).ceil() * (window_size.1 as f32 / CELL_SIZE as f32).ceil()) as usize * 64]).unwrap();
        
        let duration = start.elapsed();
        println!("Remeshed in: {:?}", duration);
        //std::thread::sleep(std::time::Duration::from_millis(2500));
    }
}

