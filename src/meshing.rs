use crate::{CELL_SIZE, MAXIMUM_WINDOW_HEIGHT, MAXIMUM_WINDOW_WIDTH, WINDOW_START_HEIGHT, WINDOW_START_WIDTH};
use crate::shader_handling::{Float2, Float4, Pipeline, ShaderHandler, Uint4, Vertex};
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Mat4x4 {
    pub m: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn mul_vec4(&self, v: Float4) -> Float4 {
        let m = &self.m;
        Float4 {
            x: m[0][0]*v.x + m[0][1]*v.y + m[0][2]*v.z + m[0][3]*v.w,
            y: m[1][0]*v.x + m[1][1]*v.y + m[1][2]*v.z + m[1][3]*v.w,
            z: m[2][0]*v.x + m[2][1]*v.y + m[2][2]*v.z + m[2][3]*v.w,
            w: m[3][0]*v.x + m[3][1]*v.y + m[3][2]*v.z + m[3][3]*v.w,
        }
    }
}

pub fn perspective(fov_radians: f32, aspect: f32, near: f32, far: f32) -> Mat4x4 {
    let f = 1.0 / (fov_radians * 0.5).tan();
    
    Mat4x4 {
        m: [
            [f / aspect, 0.0, 0.0,                          0.0],
            [0.0,        f,   0.0,                          0.0],
            [0.0,        0.0, (far+near)/(near-far), (2.0*far*near)/(near-far)],
            [0.0,        0.0, -1.0,                         0.0],
        ]
    }
}

pub fn transform_vertex(
    pos: Float4,
    proj: Mat4x4,
) -> Float4 {
    // clip space
    let clip = proj.mul_vec4(pos);
    
    // perspective divide → NDC
    let inv_w = 1.0 / clip.w;
    
    Float4 {
        x: clip.x * inv_w,
        y: clip.y * inv_w,
        z: clip.z * inv_w,
        w: inv_w,
    }
}

pub fn ndc_to_screen(ndc: Float4, width: f32, height: f32) -> Float4 {
    let x = (ndc.x * 0.5 + 0.5) * width;
    let y = (1.0 - (ndc.y * 0.5 + 0.5)) * height; // flip Y for screen coords
    let z = ndc.z * 0.5 + 0.5;                   // depth 0..1
    
    Float4::new(x, y, z, ndc.w)
}

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








fn inside(v: &Vertex, near: f32) -> bool {
    v.position.z <= -near
}

fn intersect(a: &Vertex, b: &Vertex, near: f32) -> Vertex {
    let plane_z = -near;
    
    let t = (plane_z - a.position.z) / (b.position.z - a.position.z);
    
    Vertex {
        position: Float4 {
            x: a.position.x + (b.position.x - a.position.x) * t,
            y: a.position.y + (b.position.y - a.position.y) * t,
            z: plane_z,
            w: 0.0,
        },
        uv: Float4 {
            x: a.uv.x + (b.uv.x - a.uv.x) * t,
            y: a.uv.y + (b.uv.y - a.uv.y) * t,
            ..Default::default()
        },
    }
}

pub fn clip_triangle_near_plane_inplace(
    tri_indices: [u32; 3],
    near: f32,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<Uint4>,
    dead: &mut Vec<bool>,
    w_arg: u32,
) -> bool {
    let v0 = vertices[tri_indices[0] as usize];
    let v1 = vertices[tri_indices[1] as usize];
    let v2 = vertices[tri_indices[2] as usize];
    
    let inside0 = v0.position.z >= near;
    let inside1 = v1.position.z >= near;
    let inside2 = v2.position.z >= near;
    
    let inside_count = inside0 as u32 + inside1 as u32 + inside2 as u32;
    inside_count != 3
    
    /*match inside_count {
        0 => {
            // Entire triangle behind near plane → discard
            // Could remove indices if desired, or skip
            true
        }
        
        3 => {
            // Fully inside → nothing to do, indices remain
            false
        }
        
        1 => {
            // One vertex inside → one triangle, mutate original triangle in-place
            let (i_in, i_out1, i_out2, v_in, v_out1, v_out2) =
                if inside0 {
                    (tri_indices[0], tri_indices[1], tri_indices[2], v0, v1, v2)
                } else if inside1 {
                    (tri_indices[1], tri_indices[2], tri_indices[0], v1, v2, v0)
                } else {
                    (tri_indices[2], tri_indices[0], tri_indices[1], v2, v0, v1)
                };
            
            // Compute intersections
            let i1p = intersect(&v_in, &v_out1, near);
            let i2p = intersect(&v_in, &v_out2, near);
            
            // Mutate the *existing* triangle to keep indices stable
            vertices[i_in as usize] = v_in;
            vertices[i_out1 as usize] = i1p;
            vertices[i_out2 as usize] = i2p;
            
            // Indices array remains unchanged
            true
        }
        
        2 => {
            return true;
            // Two vertices inside → one original triangle mutated + one new triangle appended
            // Determine which vertex is outside
            let (i_out, i_in1, i_in2, v_out, v_in1, v_in2) =
                if !inside0 {
                    (tri_indices[0], tri_indices[1], tri_indices[2], v0, v1, v2)
                } else if !inside1 {
                    (tri_indices[1], tri_indices[2], tri_indices[0], v1, v2, v0)
                } else {
                    (tri_indices[2], tri_indices[0], tri_indices[1], v2, v0, v1)
                };
            
            // Intersect edges from inside vertices to outside vertex
            let p1 = intersect(&v_in1, &v_out, near);
            let p2 = intersect(&v_in2, &v_out, near);
            
            // Mutate the original triangle to be the "first" triangle
            vertices[i_in1 as usize] = v_in1;
            vertices[i_in2 as usize] = v_in2;
            vertices[i_out as usize] = p2; // overwrite the outside vertex to form first triangle
            
            // Create second triangle with new vertices
            let base = vertices.len() as u32;
            vertices.push(p1);
            vertices.push(p2);
            
            // Push new triangle indices
            indices.push(Uint4::new(i_in1, base, base + 1, w_arg));
            dead.push(false);
            false
        }
        
        _ => unreachable!(),
    }*/
}












impl Mesh {
    pub fn check_remesh(&mut self, shader_handler: &mut ShaderHandler, window_size: (u32, u32), camera_position: Float4, camera_rotation: Float4) {
        if !self.mutated { return; }
        self.mutated = false;
        
        let start = std::time::Instant::now();
        
        // replacing vertices with the transformations of the original vertices
        let projection_matrix = perspective(60_f32.to_radians(), window_size.0 as f32 / window_size.1 as f32, 0.1, 9999.0);
        
        //self.vertices = vec![Vertex::default(); self.vertices_original.len()];
        //self.vertices.resize(self.vertices_original.len(), Vertex::default());
        for (i, vertex) in self.vertices_original.iter().enumerate() {
            let vert = rotate(Float4::new(
                vertex.position.x - camera_position.x,
                vertex.position.y - camera_position.y,
                vertex.position.z - camera_position.z,
                0.0,
            ), &camera_rotation);
            self.vertices[i] = Vertex::new(vert, Float2::new(vertex.uv.x, vertex.uv.y));
        }
        
        let start_len = self.indices.len();
        let mut dead = vec![false; self.indices.len()];
        for tri_index in 0..start_len {
            let tri = self.indices[tri_index].clone();
            let d = clip_triangle_near_plane_inplace(
                [tri.x, tri.y, tri.z],
                0.01,
                &mut self.vertices,
                &mut self.indices,
                &mut dead,
                tri.w,
            );
            if d {
                dead[tri_index] = true;
            }
        }
        
        for vertex in self.vertices.iter_mut() {
            let ndc = transform_vertex(
                vertex.position,
                projection_matrix
            );
            let mut vert = ndc_to_screen(ndc, window_size.0 as f32, window_size.1 as f32);
            vert.z = vertex.position.z;
            vertex.position = vert;
        }
        
        for i in 0..self.binned_indices.len() / 64 {
            self.binned_indices[i * 64] = 0;
        }
        
        let window_width = (window_size.0 as f32 / CELL_SIZE as f32).ceil() as usize * 64;
        for tri_index in 0..self.indices.len() {
            if dead[tri_index] { continue; }
            // finding all bounding box cells it falls within
            // getting the bounding box
            let v1 = &self.vertices[self.indices[tri_index].x as usize];
            let v2 = &self.vertices[self.indices[tri_index].y as usize];
            let v3 = &self.vertices[self.indices[tri_index].z as usize];
            let min_x = v1.position.x.min(v2.position.x.min(v3.position.x));
            let max_x = v1.position.x.max(v2.position.x.max(v3.position.x));
            let min_y = v1.position.y.min(v2.position.y.min(v3.position.y));
            let max_y = v1.position.y.max(v2.position.y.max(v3.position.y));
            let min_x_bin = ((min_x / CELL_SIZE as f32).floor().max(0.0) as u32).min((window_size.0 as f32 / CELL_SIZE as f32).ceil() as u32);
            let max_x_bin = ((max_x / CELL_SIZE as f32).ceil().max(0.0) as u32).min((window_size.0 as f32 / CELL_SIZE as f32).ceil() as u32);
            let min_y_bin = ((min_y / CELL_SIZE as f32).floor().max(0.0) as u32).min((window_size.1 as f32 / CELL_SIZE as f32).ceil() as u32);
            let max_y_bin = ((max_y / CELL_SIZE as f32).ceil().max(0.0) as u32).min((window_size.1 as f32 / CELL_SIZE as f32).ceil() as u32);
            
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
        shader_handler.get_shader().update_buffer_slice(5, &self.vertices[0..self.vertices_original.len()]   ).unwrap();
        shader_handler.get_shader().update_buffer_slice(6, self.normals.as_slice()    ).unwrap();
        shader_handler.get_shader().update_buffer_slice(7, self.indices.as_slice()    ).unwrap();
        shader_handler.get_shader().update_buffer_slice(8, &self.binned_indices[0..((window_size.0 as f32 / CELL_SIZE as f32).ceil() * (window_size.1 as f32 / CELL_SIZE as f32).ceil()) as usize * 64]).unwrap();
        
        let duration = start.elapsed();
        println!("Remeshed in: {:?}", duration);
        //std::thread::sleep(std::time::Duration::from_millis(2500));
    }
}

