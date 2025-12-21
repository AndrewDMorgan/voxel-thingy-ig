use crate::shader_handling::{Float2, Float4, Uint4, Vertex};
use crate::meshing::Mesh;

pub fn generate_cube(mesh: &mut Mesh, size: f32, position: Float4) {
    mesh.mutated = true;
    let half_size = size / 2.0;
    let mut vertices = vec![
        // Front face
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x - half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y + half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x - half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
        Vertex::new(Float4::new(position.x + half_size, position.y - half_size, position.z + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }),
    ];
    let start_index = mesh.vertices_original.len() as u32;
    mesh.vertices_original.append(&mut vertices);
    
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
    mesh.indices.append(&mut triangles);
}

pub struct Chunk {
    pub position: Float4,
    pub tile_data: [[[u32; 16]; 16]; 16],
}

impl Chunk {
    pub fn new(position: Float4) -> Self {
        Self {
            position,
            tile_data: [[[0u32; 16]; 16]; 16],
        }
    }
    
    pub fn remesh_tile(&self, mesh: &mut Mesh, x: usize, y: usize, z: usize, tile_size: f32) {
        // ignoring chunk boundaries for now     todo! actually handle that at some point
        let half_size = tile_size / 2.0;
        let mut vertices = vec![];
        let mut triangles = vec![];
        
        let mut start_index = mesh.vertices_original.len() as u32;
        if x > 0 && self.tile_data[x-1][y][z] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 3 | (0 << 16)));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 3 | (0 << 16)));
            start_index += 4;
        }
        if y > 0 && self.tile_data[x][y-1][z] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 1 | (2 << 16)));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 1 | (2 << 16)));
            start_index += 4;
        }
        if z > 0 && self.tile_data[x][y][z-1] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 5 | (0 << 16)));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 5 | (0 << 16)));
            start_index += 4;
        }
        if x < 15 && self.tile_data[x+1][y][z] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 2 | (0 << 16)));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 2 | (0 << 16)));
            start_index += 4;
        }
        if y < 15 && self.tile_data[x][y+1][z] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size - half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 0 | (1 << 16)));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 0 | (1 << 16)));
            start_index += 4;
        }
        if z < 15 && self.tile_data[x][y][z+1] == 0u32 {  // todo! use at some point another metric to determine if a tile is solid
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size - half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 0.0, y: 1.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size - half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            vertices.push(Vertex::new(Float4::new((self.position.x + x as f32) * tile_size + half_size, (self.position.y + y as f32) * tile_size + half_size, (self.position.z + z as f32) * tile_size + half_size, 0.0), Float2 { x: 1.0, y: 0.0 }));
            triangles.push(Uint4::new(start_index, start_index + 1, start_index + 3, 4));
            triangles.push(Uint4::new(start_index, start_index + 2, start_index + 3, 4));
            start_index += 4;
        }
        mesh.vertices_original.append(&mut vertices);
        mesh.indices.append(&mut triangles);
    }
    
    pub fn remesh_chunk(&self, mesh: &mut Mesh, tile_size: f32) {
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if self.tile_data[x][y][z] != 0u32 {
                        self.remesh_tile(mesh, x, y, z, tile_size);
                    }
                }
            }
        }
    }
}

