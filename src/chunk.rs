use crate::shader_handling::{Float2, Float4, Uint4, Vertex};
use crate::meshing::Mesh;

pub fn generate_cube(mesh: &mut Mesh, size: f32, position: Float4, chunk_priority: usize, chunk_index: usize) {
    mesh.mutated(true);
    let half_size = size / 2.0;
    let mut vertices = vec![
        // Front face
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, Float4::new(1.0, 1.0, 1.0, 0.0)),
    ];
    let start_index = mesh.vertices_original_ref().len() as u32;
    mesh.append_vertices(&mut vertices, chunk_priority, chunk_index);
    
    // adding the triangle linkages
    let mut triangles = vec![
        Uint4::new(start_index, start_index + 1, start_index + 3, 4),
        Uint4::new(start_index, start_index + 2, start_index + 3, 4),
        
        Uint4::new(start_index + 4, start_index + 1 + 4, start_index + 3 + 4, 5 | (0 << 16)),
        Uint4::new(start_index + 4, start_index + 2 + 4, start_index + 3 + 4, 5 | (0 << 16)),
        
        Uint4::new(start_index + 8, start_index + 1 + 8, start_index + 3 + 8, 2 | (0 << 16)),
        Uint4::new(start_index + 8, start_index + 2 + 8, start_index + 3 + 8, 2 | (0 << 16)),
        
        Uint4::new(start_index + 12, start_index + 1 + 12, start_index + 3 + 12, 3 | (0 << 16)),
        Uint4::new(start_index + 12, start_index + 2 + 12, start_index + 3 + 12, 3 | (0 << 16)),
        
        Uint4::new(start_index + 16, start_index + 1 + 16, start_index + 3 + 16, 0 | (1 << 16)),
        Uint4::new(start_index + 16, start_index + 2 + 16, start_index + 3 + 16, 0 | (1 << 16)),
        
        Uint4::new(start_index + 20, start_index + 1 + 20, start_index + 3 + 20, 1 | (2 << 16)),
        Uint4::new(start_index + 20, start_index + 2 + 20, start_index + 3 + 20, 1 | (2 << 16)),
    ];
    mesh.append_indices(&mut triangles, chunk_index);
}

pub struct Chunk {
    pub position: Float4,
    pub tile_data: [[[u32; 16]; 16]; 16],
    pub mesh_vert: Vec<Vertex>,
    pub mesh_tris: Vec<Uint4>,
    pub chunk_index: usize,  // used for culling in the main mesh
    pub mutated: bool,
}

impl Chunk {
    pub fn new(position: Float4, chunk_index: usize) -> Self {
        Self {
            position,
            tile_data: [[[0u32; 16]; 16]; 16],
            mesh_tris: vec![],
            mesh_vert: vec![],
            chunk_index,
            mutated: false,
        }
    }
    
    pub fn remesh_tile(&mut self, x: usize, y: usize, z: usize, tile_size: f32) {
        // ignoring chunk boundaries for now     todo! actually handle that at some point
        let half_size = tile_size / 2.0;
        let mut vertices = vec![];
        let mut triangles = vec![];
        
        let lighting = Float4::new(1.0, 1.0, 1.0, 0.0);  // todo! implement proper lighting
        let mut start_index = self.mesh_vert.len() as u32;
        if x > 0 && self.tile_data[x-1][y][z] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 3 | (0 << 16)));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 3 | (0 << 16)));
            start_index += 4;
        }
        if y > 0 && self.tile_data[x][y-1][z] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 1 | (2 << 16)));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 1 | (2 << 16)));
            start_index += 4;
        }
        if z > 0 && self.tile_data[x][y][z-1] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 5 | (0 << 16)));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 5 | (0 << 16)));
            start_index += 4;
        }
        if x < 15 && self.tile_data[x+1][y][z] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 2 | (0 << 16)));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 2 | (0 << 16)));
            start_index += 4;
        }
        if y < 15 && self.tile_data[x][y+1][z] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 0 | (1 << 16)));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 0 | (1 << 16)));
            start_index += 4;
        }
        if z < 15 && self.tile_data[x][y][z+1] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }, lighting));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 4));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 4));
            start_index += 4;
        }
        self.mesh_vert.append(&mut vertices);
        self.mesh_tris.append(&mut triangles);
    }
    
    pub fn remesh_chunk(&mut self, mesh: &mut Mesh, tile_size: f32, chunk_priority: usize) {
        if self.mutated {
            self.mutated = false;
            self.mesh_tris.clear();
            self.mesh_vert.clear();
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        if self.tile_data[x][y][z] != 0u32 {
                            self.remesh_tile(x, y, z, tile_size);
                        }
                    }
                }
            }
        }
        let start_index = mesh.vertices_original_ref().len() as u32;
        mesh.append_vertices(&mut self.mesh_vert, chunk_priority, self.chunk_index);
        mesh.append_indices(&mut self.mesh_tris.iter().map(|tri| Uint4::new(tri.x + start_index, tri.y + start_index, tri.z + start_index, tri.w)).collect(), self.chunk_index);
    }
}

