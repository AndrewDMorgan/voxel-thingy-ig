use crate::{CELL_SIZE, MAXIMUM_WINDOW_HEIGHT, MAXIMUM_WINDOW_WIDTH, WINDOW_START_HEIGHT, WINDOW_START_WIDTH};
use crate::shader_handling::{Float2, Float4, Pipeline, ShaderHandler, Uint4, Vertex};

pub struct MeshDoubleBuffer {
    pub front: std::sync::Arc<parking_lot::RwLock<Mesh>>,
    pub back: std::sync::Arc<parking_lot::RwLock<Mesh>>,
    pub current_front: std::sync::Arc<parking_lot::RwLock<bool>>,
    pub swapping: std::sync::Arc<parking_lot::RwLock<bool>>,
}

impl MeshDoubleBuffer {
    pub fn current(&self) -> std::sync::Arc<parking_lot::RwLock<Mesh>> {
        if *self.current_front.read() {
            self.front.clone()
        } else {
            self.back.clone()
        }
    }
    
    pub fn back(&self) -> std::sync::Arc<parking_lot::RwLock<Mesh>> {
        if *self.current_front.read() {
            self.back.clone()
        } else {
            self.front.clone()
        }
    }
    
    pub fn swap(&self) {
        *self.swapping.write() = true;
    }
    
    pub fn update(&self) -> bool {
        if !*self.swapping.read() {
            return false;
        }
        *self.swapping.write() = false;
        let mut front = self.current_front.write();
        *front = !*front;
        true
    }
}

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
        light: Float4::new(1.0, 1.0, 1.0, 0.0)
    }
}

pub fn clip_triangle_near_plane_inplace(
    tri_indices: [u32; 3],
    near: f32,
    vertices: &[Vertex],
) -> bool {
    let v0 = vertices[tri_indices[0] as usize];
    let v1 = vertices[tri_indices[1] as usize];
    let v2 = vertices[tri_indices[2] as usize];
    
    let inside0 = v0.position.z >= near;
    let inside1 = v1.position.z >= near;
    let inside2 = v2.position.z >= near;
    
    let inside_count = inside0 as u32 + inside1 as u32 + inside2 as u32;
    inside_count != 3
}

fn is_triangle_culled_inline(tri_center: Float4, radius: f32, tan_half_fov_y: f32, tan_half_fov_x: f32) -> bool {
    let x = tri_center.x;
    let y = tri_center.y;
    let z = tri_center.z;
    let z_term = z * tan_half_fov_x;
    if x < -z_term - radius {
        return true;
    }
    if x > z_term + radius {
        return true;
    }
    let z_term = z * tan_half_fov_y;
    if y < -z_term - radius {
        return true;
    }
    
    if y > z_term + radius {
        return true;
    }
    false
}

struct UnsafePtrWrapper<T> { ptr: T }
impl<T> UnsafePtrWrapper<T> {
    fn new(ptr: T) -> Self {
        UnsafePtrWrapper { ptr }
    }
    fn unwrap(&self) -> T where T: Copy {
        self.ptr
    }
}
unsafe impl<T> Send for UnsafePtrWrapper<T> {}
unsafe impl<T> Sync for UnsafePtrWrapper<T> {}

#[derive(Clone)]
pub struct Mesh {
    mutated: bool,
    vertices_original: Vec<Vertex>,
    vertex_ownership: Vec<usize>,
    vertices: Vec<Vertex>,
    indices: Vec<Uint4>,
    index_chunks: Vec<usize>,
    chunks: Vec<(Float4, Float4)>,  // position, size
    normals: Vec<Float4>,
    binned_indices: Vec<u32>,
    dead: Vec<bool>,
    is_chunk_culled: Vec<bool>,
    vert_chunk_index: Vec<usize>,
}
//let mut dead = vec![false; self.indices.len()];
impl Mesh {
    pub fn mutated(&mut self, value: bool) {
        self.mutated = value;
    }
    
    pub fn indices_ref(&self) -> &Vec<Uint4> {
        &self.indices
    }

    pub fn vertices_original_ref(&self) -> &Vec<Vertex> {
        &self.vertices_original
    }

    pub fn add_chunk(&mut self, position: Float4, size: Float4) {
        self.chunks.push((position, size));
        self.is_chunk_culled.push(false);
    }
    
    pub fn push_index(&mut self, index: Uint4, chunk_index: usize) {
        self.indices.push(index);
        self.dead.push(false);
        self.index_chunks.push(chunk_index);
    }
    
    pub fn append_indices(&mut self, indices: &mut Vec<Uint4>, chunk_index: usize) {
        for _ in 0..indices.len() {
            self.dead.push(false);
            self.index_chunks.push(chunk_index);
        }
        self.indices.extend_from_slice(&indices);
    }
    
    pub fn push_vertex(&mut self, vertex: Vertex, owner: usize, chunk_index: usize) {
        self.vertices_original.push(vertex);
        //self.vertices.push(vertex);
        self.vertex_ownership.push(owner);
        self.vert_chunk_index.push(chunk_index)
    }
    
    pub fn append_vertices(&mut self, vertices: &mut Vec<Vertex>, owner: usize, chunk_index: usize) {
        for _ in 0..vertices.len() {
            self.vertex_ownership.push(owner);
            self.vert_chunk_index.push(chunk_index)
        }
        self.vertices_original.extend_from_slice(&vertices);
        //self.vertices.append(vertices);
    }
    
    pub fn chunk_ref(&self) -> &Vec<(Float4, Float4)> {
        &self.chunks
    }

    pub fn new(mutated: bool, vertices_original: Vec<Vertex>, vertex_ownership: Vec<usize>, index_chunks: Vec<usize>, chunks: Vec<(Float4, Float4)>, vertices: Vec<Vertex>, indices: Vec<Uint4>, normals: Vec<Float4>, binned_indices: Vec<u32>, dead: Vec<bool>, is_chunk_culled: Vec<bool>, vert_chunk_index: Vec<usize>) -> Self {
        Mesh {
            mutated,
            vertices_original,
            vertex_ownership,
            index_chunks,
            chunks,
            vertices,
            indices,
            normals,
            binned_indices,
            dead,
            is_chunk_culled,
            vert_chunk_index,
        }
    }
    
    pub fn was_mutated(&self) -> bool {
        self.mutated
    }
    
    pub fn check_remesh(&mut self, /*shader_handler: &mut ShaderHandler,*/ window_size: (u32, u32), camera_position: Float4, camera_rotation: Float4, meshing_priority_min: usize, meshing_priority_max: usize, print_debug: bool) {
        if !self.mutated { return; }
        self.mutated = false;
        
        let start = std::time::Instant::now();
        
        // going through all chunks and finding which ones should be culled
        let fov_y = 60_f32.to_radians();
        let aspect = window_size.0 as f32 / window_size.1 as f32;
        let tan_half_fov_y = (fov_y * 0.5).tan();
        let tan_half_fov_x = tan_half_fov_y * aspect;
        
        let mut culled_chunks = 0;
        
        for chunk_index in 0..self.chunks.len() {
            self.is_chunk_culled[chunk_index] = {
                // checking if the chunk is outside the view frustum
                let position = &self.chunks[chunk_index];
                let center = rotate(Float4::new(
                    position.0.x + position.1.x * 0.5 - camera_position.x,
                    position.0.y + position.1.y * 0.5 - camera_position.y,
                    position.0.z + position.1.z * 0.5 - camera_position.z,
                    0.0,
                ), &camera_rotation);
                let rad = (
                    position.1.x * position.1.x +
                    position.1.y * position.1.y +
                    position.1.z * position.1.z
                ).sqrt();// * 1.5;
                is_triangle_culled_inline(center, rad, tan_half_fov_y, tan_half_fov_x)
            };
            if self.is_chunk_culled[chunk_index] {
                culled_chunks += 1;
            }
        }
        
        // replacing vertices with the transformations of the original vertices
        let projection_matrix = perspective(60_f32.to_radians(), window_size.0 as f32 / window_size.1 as f32, 0.1, 9999.0);
        
        // creating slices which DO not overlap to ensure thread safety
        const THREAD_COUNT: usize = 8;  // seems to be a good number for the best speed, but idk
        let length = self.vertices_original.len();
        let mut slices = vec![];
        for i in 0..THREAD_COUNT {
            let start = i * (length / THREAD_COUNT);
            let end = if i == THREAD_COUNT-1 {
                length
            } else {
                (i + 1) * (length / THREAD_COUNT)
            };
            slices.push((
                start, end,
            ));
        }
        
        let mut thread_handles = vec![];
        // these pointers are safe as the data lives for the length of this class, but the pointers are used purely for part of the function call
        // the pointers are only used in either immutable state where they all get cleaned up after use, or mutable state where the slices ensure no overlap
        let vert_orig_ptr = std::sync::Arc::new(UnsafePtrWrapper::new(self.vertices_original.as_ptr()));
        let vert_mut_ptr  = std::sync::Arc::new(UnsafePtrWrapper::new(self.vertices.as_mut_ptr()));
        let ownership_ptr  = std::sync::Arc::new(UnsafePtrWrapper::new(self.vertex_ownership.as_ptr()));
        let vert_chunk_index_ptr  = std::sync::Arc::new(UnsafePtrWrapper::new(self.vert_chunk_index.as_ptr()));
        let culled_chunks_len = self.is_chunk_culled.len();
        let is_chunk_culled_ptr = std::sync::Arc::new(UnsafePtrWrapper::new(self.is_chunk_culled.as_ptr()));
        for (start, end) in &slices {
            let (start, end) = (*start, *end);
            let vert_orig_ptr = vert_orig_ptr.clone();
            let vert_mut_ptr  = vert_mut_ptr.clone();
            let ownership_ptr = ownership_ptr.clone();
            let vert_chunk_index_ptr = vert_chunk_index_ptr.clone();
            let is_chunk_culled_ptr = is_chunk_culled_ptr.clone();
            
            let camera_position = camera_position.clone();
            let camera_rotation = camera_rotation.clone();
            
            let thread = std::thread::spawn(move || unsafe {
                let len = end - start;
                let vertices_original = std::slice::from_raw_parts(vert_orig_ptr.unwrap().add(start), len);
                let vertices = std::slice::from_raw_parts_mut(vert_mut_ptr.unwrap().add(start), len);
                let ownership = std::slice::from_raw_parts(ownership_ptr.unwrap().add(start), len);
                let vert_chunk_index = std::slice::from_raw_parts(vert_chunk_index_ptr.unwrap().add(start), len);
                let is_chunk_culled = std::slice::from_raw_parts(is_chunk_culled_ptr.unwrap(), culled_chunks_len);
                for (i, vertex) in vertices_original.iter().enumerate() {
                    if is_chunk_culled[vert_chunk_index[i]] || ownership[i] < meshing_priority_min || ownership[i] > meshing_priority_max {
                        continue;  // using the old results (hopefully they're ok, sometimes a complete remesh will be necessary though)
                    }
                    let vert = rotate(Float4::new(
                        vertex.position.x - camera_position.x,
                        vertex.position.y - camera_position.y,
                        vertex.position.z - camera_position.z,
                        0.0,
                    ), &camera_rotation);
                    vertices[i] = Vertex::new(vert, Float2::new(vertex.uv.x, vertex.uv.y), vertex.light);
                }
            });
            thread_handles.push(thread);
        }
        for handle in thread_handles {
            handle.join().unwrap();
        }
        
        let culled = std::sync::Arc::new(parking_lot::RwLock::new(0));
        let length = self.indices.len();
        let mut slices_tri = vec![];
        for i in 0..THREAD_COUNT {
            let start = i * (length / THREAD_COUNT);
            let end = if i == THREAD_COUNT-1 {
                length
            } else {
                (i + 1) * (length / THREAD_COUNT)
            };
            slices_tri.push((
                start, end,
            ));
        }
        
        let dead_ptr = std::sync::Arc::new(UnsafePtrWrapper::new(self.dead.as_mut_ptr()));
        let indices_ptr = std::sync::Arc::new(UnsafePtrWrapper::new(self.indices.as_ptr()));
        let vertices_ptr = std::sync::Arc::new(UnsafePtrWrapper::new(self.vertices.as_ptr()));
        let normals_ptr = std::sync::Arc::new(UnsafePtrWrapper::new(self.normals.as_ptr()));
        let chunk_owner_ptr = std::sync::Arc::new(UnsafePtrWrapper::new(self.index_chunks.as_ptr()));
        let ownership_ptr  = std::sync::Arc::new(UnsafePtrWrapper::new(self.vertex_ownership.as_ptr()));
 
        let num_norms = self.normals.len();
        let num_verts = self.vertices_original.len();
        let mut handles = vec![];
        for (start, end) in &slices_tri {
            let culled = culled.clone();
            let (start, end) = (*start, *end);
            let dead_ptr = dead_ptr.clone();
            let indices_ptr = indices_ptr.clone();
            let vertices_ptr = vertices_ptr.clone();
            let camera_rotation = camera_rotation.clone();
            let normals_ptr = normals_ptr.clone();
            let ownership_ptr = ownership_ptr.clone();
            let chunk_owner_ptr = chunk_owner_ptr.clone();
            let is_chunk_culled_ptr = is_chunk_culled_ptr.clone();
            let thread = std::thread::spawn(move || unsafe {
                let dead = std::slice::from_raw_parts_mut(dead_ptr.unwrap(), length);
                let indices = std::slice::from_raw_parts(indices_ptr.unwrap(), length);
                let vertices = std::slice::from_raw_parts(vertices_ptr.unwrap(), num_verts);
                let normals = std::slice::from_raw_parts(normals_ptr.unwrap(), num_norms);
                let ownership = std::slice::from_raw_parts(ownership_ptr.unwrap(), num_verts);
                let chunk_owner = std::slice::from_raw_parts(chunk_owner_ptr.unwrap(), length);
                let is_chunk_culled = std::slice::from_raw_parts(is_chunk_culled_ptr.unwrap(), culled_chunks_len);
                for tri_index in start..end {
                    let index = &indices[tri_index];
                    if is_chunk_culled[chunk_owner[tri_index]] {
                        dead[tri_index] = true;
                        continue;
                    }
                    // all vertices should have the same priority
                    if ownership[index.x as usize] > meshing_priority_max || ownership[index.x as usize] < meshing_priority_min {
                        continue;  // not mutated dead either, as the previous result will continue to be used
                    }
                    let p0 = &vertices[index.x as usize].position;
                    let p1 = &vertices[index.y as usize].position;
                    let p2 = &vertices[index.z as usize].position;
                    let tri_center = Float4::new(
                        (p0.x + p1.x + p2.x) * const { 1.0 / 3.0 },
                        (p0.y + p1.y + p2.y) * const { 1.0 / 3.0 },
                        (p0.z + p1.z + p2.z) * const { 1.0 / 3.0 },
                        0.0
                    );
                    let view_vector = tri_center.negate().normalized();
                    let normal_view = rotate(normals[(indices[tri_index].w & 0xFFFF) as usize], &camera_rotation).normalized();
                    if view_vector.dot(&normal_view) < 0.0 {
                        dead[tri_index] = true;
                        continue;
                    }
                    let rad = (
                        (tri_center.x - p0.x) * (tri_center.x - p0.x) +
                        (tri_center.y - p0.y) * (tri_center.y - p0.y) +
                        (tri_center.z - p0.z) * (tri_center.z - p0.z)
                    ).sqrt();
                    if is_triangle_culled_inline(tri_center, rad, tan_half_fov_y, tan_half_fov_x) {
                        dead[tri_index] = true;
                        *culled.write() += 1;
                        continue;
                    }
                    dead[tri_index] = false;
                    
                    let tri = indices[tri_index].clone();
                    let d = clip_triangle_near_plane_inplace(
                        [tri.x, tri.y, tri.z],
                        0.01,
                        vertices,
                    );
                    dead[tri_index] = d;
                }
            });
            handles.push(thread);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        
        let mut thread_handles = vec![];
        let vert_mut_ptr  = std::sync::Arc::new(UnsafePtrWrapper::new(self.vertices.as_mut_ptr()));
        for (start, end) in &slices {
            let ownership_ptr = ownership_ptr.clone();
            let projection_matrix = projection_matrix.clone();
            let (start, end) = (*start, *end);
            let vert_mut_ptr  = vert_mut_ptr.clone();
            let is_chunk_culled_ptr = is_chunk_culled_ptr.clone();
            let vert_chunk_index_ptr = vert_chunk_index_ptr.clone();
            let thread = std::thread::spawn(move || unsafe {
                let len = end - start;
                let vertices = std::slice::from_raw_parts_mut(vert_mut_ptr.unwrap().add(start), len);
                let ownership = std::slice::from_raw_parts(ownership_ptr.unwrap().add(start), len);
                let vert_chunk_index = std::slice::from_raw_parts(vert_chunk_index_ptr.unwrap().add(start), len);
                let is_chunk_culled = std::slice::from_raw_parts(is_chunk_culled_ptr.unwrap(), culled_chunks_len);
                for (i, vertex) in vertices.iter_mut().enumerate() {
                    if is_chunk_culled[vert_chunk_index[i]] || ownership[i] < meshing_priority_min || ownership[i] > meshing_priority_max {
                        continue;  // using the old results (hopefully they're ok, sometimes a complete remesh will be necessary though)
                    }
                    let ndc = transform_vertex(
                        vertex.position,
                        projection_matrix
                    );
                    let mut vert = ndc_to_screen(ndc, window_size.0 as f32, window_size.1 as f32);
                    vert.z = vertex.position.z;
                    vertex.position = vert;
                }
            });
            thread_handles.push(thread);
        }
        for handle in thread_handles {
            handle.join().unwrap();
        }
        
        let middle_split = start.elapsed();
        
        for i in 0..self.binned_indices.len() / 64 {
            self.binned_indices[i * 64] = 0;
        }
        
        let window_width = (window_size.0 as f32 / CELL_SIZE as f32).ceil() as usize * 64;
        for tri_index in 0..self.indices.len() {
            if self.dead[tri_index] { continue; }
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
        
        let duration = start.elapsed();
        println!("\n\nRemeshed in: {:?}     Binned in {:?}      Transformed in {:?}      Culled: {}      Chunks Culled: {}\n\n", duration, duration - middle_split, middle_split, *culled.read(), culled_chunks);
    }
    
    pub fn update_shader_buffers(&self, shader_handler: &mut ShaderHandler, window_size: (u32, u32), camera_rotation: Float4) {
        let camera_vector = rotate(Float4::new(0.0, 0.0, 1.0, 0.0), &camera_rotation.negate()).normalized();
        //shader_handler.get_shader().update_buffer(3,  camera_position                 ).unwrap();
        shader_handler.get_shader().update_buffer(4,  camera_vector                   ).unwrap();
        shader_handler.get_shader().update_buffer_slice(5, &self.vertices[0..self.vertices_original.len()]   ).unwrap();
        shader_handler.get_shader().update_buffer_slice(6, self.normals.as_slice()    ).unwrap();
        shader_handler.get_shader().update_buffer_slice(7, self.indices.as_slice()    ).unwrap();
        shader_handler.get_shader().update_buffer_slice(8, &self.binned_indices[0..((window_size.0 as f32 / CELL_SIZE as f32).ceil() * (window_size.1 as f32 / CELL_SIZE as f32).ceil()) as usize * 64]).unwrap();
    }
}

