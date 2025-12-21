use crate::{CELL_SIZE, MAXIMUM_WINDOW_HEIGHT, MAXIMUM_WINDOW_WIDTH, WINDOW_START_HEIGHT, WINDOW_START_WIDTH};
use crate::shader_handling::{Float4, ShaderHandler, Uint4, Vertex};

pub struct Mesh {
    pub vertices: Vec<Float4>,
    pub(crate) vertices_original: Vec<[f32; 3]>,
    pub indices: Vec<Uint4>,
    pub normals: Vec<Float4>,
    pub binned_indices: Vec<u32>,
    pub mutated: bool,
    pub vertex_buffer: Vec<Vertex>,
}

pub fn rotate(vertex: Float4, rotation: &Float4) -> Float4 {
    vertex
}

impl Mesh {
    pub fn get_vertex_buffer(&self) -> &Vec<Vertex> {
        &self.vertex_buffer
    }
    
    pub fn check_remesh(&mut self, shader_handler: &mut ShaderHandler, window_size: (u32, u32), camera_position: Float4, camera_rotation: Float4) {
        if !self.mutated { return; }
        
        self.mutated = false;
        
        self.vertex_buffer.clear();
        for vertex in &self.vertices_original {
            let v = Vertex {
                position: vertex.clone(),
                normal: [0.0, 0.0, 0.0],
                uv: [0.0, 0.0],
                texture_id: 0,
            };
            self.vertex_buffer.push(v);
        }
        
        // replacing vertices with the transformations of the original vertices
        /*for (i, vertex) in self.vertices_original.iter().enumerate() {
            self.vertices[i] = rotate(Float4::new(
                (vertex[0] - camera_position.x),  // todo! add rotation
                (vertex[1] - camera_position.y),  // todo! add rotation
                (vertex[2] - camera_position.z),  // todo! add rotation
                1.0,
            ), &camera_rotation);
        }
        
        for i in 0..self.binned_indices.len() / 64 {
            self.binned_indices[i * 64] = 0;
        }
        
        let window_width = (WINDOW_START_WIDTH as f32 / CELL_SIZE as f32).ceil() as usize * 64;
        for tri_index in 0..self.indices.len() {
            // finding all bounding box cells it falls within
            // getting the bounding box
            let v1 = &self.vertices[self.indices[tri_index].x as usize];
            let v2 = &self.vertices[self.indices[tri_index].y as usize];
            let v3 = &self.vertices[self.indices[tri_index].z as usize];
            let min_x = v1.x.min(v2.x.min(v3.x));
            let max_x = v1.x.max(v2.x.max(v3.x));
            let min_y = v1.y.min(v2.y.min(v3.y));
            let max_y = v1.y.max(v2.y.max(v3.y));
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
        }*/
        
        //shader_handler.get_shader().update_buffer(3,  camera_position                 ).unwrap();
        //shader_handler.get_shader().update_buffer(4,  camera_rotation                 ).unwrap();
        shader_handler.get_shader().update_buffer_slice(5, self.vertices.as_slice()   ).unwrap();
        shader_handler.get_shader().update_buffer_slice(6, self.normals.as_slice()    ).unwrap();
        shader_handler.get_shader().update_buffer_slice(7, self.indices.as_slice()    ).unwrap();
        shader_handler.get_shader().update_buffer_slice(8, &self.binned_indices[0..((window_size.0 as f32 / CELL_SIZE as f32).ceil() * (window_size.1 as f32 / CELL_SIZE as f32).ceil()) as usize * 64]).unwrap();
        
        //let duration = start.elapsed();
        //println!("Remeshed in: {:?}", duration);
        //std::thread::sleep(std::time::Duration::from_millis(2500));
    }
}

