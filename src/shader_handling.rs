use metal::{Buffer, CommandQueue, CompileOptions, ComputePipelineDescriptor, ComputePipelineState, Device, MTLClearColor, MTLLoadAction, MTLPixelFormat, MTLPrimitiveType, MTLResourceOptions, MTLSize, MTLStoreAction, MTLTextureUsage, NSUInteger, RenderPassDescriptor, RenderPipelineDescriptor, RenderPipelineState, Texture, TextureDescriptor};
use crate::meshing::Mesh;

// this could be aligned with the alignment derive, but it's not needed
// as it's already aligned by the hard coded types
// 128 bits or 16 bytes
// the metal float4 type has no padding between elements either
// padding at the end doesn't really matter here (only when defining buffer sizes)
#[repr(C)]
#[derive(Debug, Clone, Default, Copy)]
pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Default, Copy)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    pub(crate) fn new(p0: f32, p1: f32) -> Float2 {
        Self {
            x: p0,
            y: p1,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Uint4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

impl Float4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Float4 { x, y, z, w }
    }
    
    pub fn negate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
    
    pub fn normalized(self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length == 0.0 {
            Self { x: 0.0, y: 0.0, z: 0.0, w: self.w }
        } else {
            Self {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
                w: self.w,
            }
        }
    }
}

impl Uint4 {
    pub fn tri_index(&self, index: usize) -> u32 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Index out of bounds for tri_index: {}", index),
        }
    }
    
    pub fn new(x: u32, y: u32, z: u32, w: u32) -> Self {
        Uint4 { x, y, z, w }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default, Copy)]
pub struct Uchar4 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub w: u8,
}

impl Uchar4 {
    pub fn new(x: u8, y: u8, z: u8, w: u8) -> Self {
        Uchar4 { x, y, z, w }
    }
}

pub struct Tuple<A, B> {
    pub first: A,
    pub second: B,
}

/// Handles a set of mutable shaders for different contexts
pub struct ShaderHandler {
    pub device: Device,
    shader: Shader,
}

impl From<ShaderError> for String {
    fn from(details: ShaderError) -> String {
        format!("[Shader Error] {:?}", details.details)
    }
}

impl ShaderHandler {
    pub fn new(device: Device, shader: Shader) -> Self {
        ShaderHandler {
            device,
            shader,
        }
    }
    
    pub fn get_shader(&mut self) -> &mut Shader {
        &mut self.shader
    }
}

/// Handles a single shader, its pipeline state, and its buffers
pub struct Shader {
    pipeline_state: ComputePipelineState,
    command_queue: CommandQueue,
    buffers: Vec<Buffer>,
}

impl Shader {
    /// Creates a new shader from the given device, source file, buffer sizes, and entry function name
    pub fn new(device: &Device, source: &str, buffer_sizes: &[u64], entry_function_name: &str) -> Result<Self, ShaderError> {
        let src = std::fs::read_to_string(source)
            .map_err(|e| ShaderError { details: format!("Failed to read shader file: {}", e) })?;
        // compiling the shaders and getting them ready
        let opts = CompileOptions::new();
        let lib = device.new_library_with_source(&src, &opts)
            .map_err(|e| ShaderError { details: format!("Failed to compile Metal shader: {}", e) })?;
        
        let func = lib.get_function(entry_function_name, None)
            .map_err(|e| ShaderError {
                details: format!("Failed to locate and get function '{}' from Metal library (Please verify the name is correct): {}", entry_function_name, e)
            })?;
        let desc = ComputePipelineDescriptor::new();
        desc.set_compute_function(Some(&func));
        
        let pipeline_state = device
            .new_compute_pipeline_state(&desc)
            .map_err(|e| ShaderError { details: format!("Failed to create compute pipeline state: {}", e) })?;
        
        let command_queue = device.new_command_queue();
        
        let mut buffers = Vec::new();
        for buffer_size in buffer_sizes {
            buffers.push(
                // there can't really be any data in this yet as this function doesn't request the user to provide that
                device.new_buffer(
                    *buffer_size,
                    MTLResourceOptions::StorageModeShared
                )
            );
        }
        
        Ok(Shader {
            pipeline_state,
            command_queue,
            buffers,
        })
    }
    
    /// Updates the data in the specified buffer
    pub fn update_buffer<T>(&mut self, index: usize, data: T) -> Result<(), ShaderError> {
        let ptr = self.buffers[index].contents() as *mut T;
        if ptr.is_null() {
            return Err(ShaderError { details: "Failed to get buffer contents; the pointer to its contents was null.".to_string() });
        }
        unsafe { *ptr = data; }
        Ok(())
    }
    
    /// Updates the data in the specified buffer from a slice
    pub fn update_buffer_slice<T>(&mut self, index: usize, data: &[T]) -> Result<(), ShaderError> {
        let ptr = self.buffers[index].contents() as *mut T;
        if ptr.is_null() {
            return Err(ShaderError { details: "Failed to get buffer contents; the pointer to its contents was null.".to_string() });
        }
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, data.len());
        }
        Ok(())
    }
    
    /// Executes the shader with the given grid and threadgroup sizes
    pub fn execute<'a, T>(&self, grid_size: MTLSize, threadgroup_size: MTLSize, callback: Option<impl FnOnce() -> Result<(), T> + 'a>) -> Result<(), T> {
        let command_buffer = self.command_queue.new_command_buffer();
        let encoder = command_buffer.new_compute_command_encoder();
        encoder.set_compute_pipeline_state(&self.pipeline_state);
        
        // attaching the buffers
        for (i, buffer) in self.buffers.iter().enumerate() {
            encoder.set_buffer(i as u64, Some(buffer), 0);
        }
        
        encoder.dispatch_threads(grid_size, threadgroup_size);
        encoder.end_encoding();
        command_buffer.commit();
        
        let result = {
            if let Some(callback) = callback { callback() }
            else { Ok(()) }
        };
        
        command_buffer.wait_until_completed();
        result
    }
    
    /// Gets a mutable pointer to the contents of the specified buffer
    pub fn get_buffer_contents<T>(&self, index: usize) -> *mut T {
        self.buffers[index].contents() as *mut T
    }
}

/// Represents an error that occurred while handling shaders
#[derive(Debug)]
pub struct ShaderError {
    pub(crate) details: String,
}

impl ShaderError {
    pub fn new(details: String) -> Self {
        ShaderError { details }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vertex {
    pub(crate) position: Float4,
    pub(crate) uv: Float4,
}

impl Vertex {
    pub fn new(position: Float4, uv: Float2) -> Self {
        Vertex { position, uv: Float4::new(uv.x, uv.y, 0.0, 0.0) }
    }
}

#[repr(C)]
pub struct Float4x4 {
    pub(crate) mvp: [[f32; 4]; 4],
}

pub struct Pipeline {
    pub pipeline_state: RenderPipelineState,
    pub command_queue: CommandQueue,
    pub vertex_buffer: Buffer,
    pub perspective_matrix: Buffer,
}

impl Pipeline {
    pub fn new(device: &Device,
               source: &str,
               vertex_function_name: &str,
               fragment_function_name: &str,
               vertex_buffer: &Vec<Vertex>,
               perspective_matrix: Float4x4,
    ) -> Result<Self, ShaderError> {
        let src = std::fs::read_to_string(source)
            .map_err(|e| ShaderError { details: format!("Failed to read shader file: {}", e) })?;
        // compiling the shaders and getting them ready
        let opts = CompileOptions::new();
        let lib = device.new_library_with_source(&src, &opts)
            .map_err(|e| ShaderError { details: format!("Failed to compile Metal shader: {}", e) })?;
        
        let desc = RenderPipelineDescriptor::new();
        let func = lib.get_function(vertex_function_name, None)
            .map_err(|e| ShaderError {
                details: format!("Failed to locate and get function '{}' from Metal library (Please verify the name is correct): {}", vertex_function_name, e)
            })?;
        desc.set_vertex_function(Some(&func));
        let func = lib.get_function(fragment_function_name, None)
            .map_err(|e| ShaderError {
                details: format!("Failed to locate and get function '{}' from Metal library (Please verify the name is correct): {}", fragment_function_name, e)
            })?;
        desc.set_fragment_function(Some(&func));
        desc.color_attachments()
            .object_at(0)
            .unwrap()
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        
        let pipeline_state = device
            .new_render_pipeline_state(&desc)
            .map_err(|e| ShaderError { details: format!("Failed to create compute pipeline state: {}", e) })?;
        
        let command_queue = device.new_command_queue();
        
        // every change in vertex length will need a realloc of this ig
        let vertex_buffer = device.new_buffer_with_data(
            vertex_buffer.as_ptr() as *const _,
            (size_of::<Vertex>() * vertex_buffer.len()) as u64,
            MTLResourceOptions::StorageModeShared,
        );
        
        let uniform_buffer = device.new_buffer_with_data(
            &perspective_matrix as *const _ as *const _,
            size_of::<Float4x4>() as u64,
            MTLResourceOptions::StorageModeShared,
        );
        
        Ok(Pipeline {
            pipeline_state,
            command_queue,
            vertex_buffer,
            perspective_matrix: uniform_buffer,
        })
    }
    
    pub fn update_vertex_buffer(&mut self, vertex_buffer: &Vec<Vertex>) {
        let new_buffer = self.pipeline_state.device().new_buffer_with_data(
            vertex_buffer.as_ptr() as *const _,
            (size_of::<Vertex>() * vertex_buffer.len()) as u64,
            MTLResourceOptions::StorageModeShared,
        );
        self.vertex_buffer = new_buffer;
    }
    
    pub fn update_perspective_matrix(&mut self, perspective_matrix: Float4x4) {
        let ptr = self.perspective_matrix.contents() as *mut Float4x4;
        unsafe { *ptr = perspective_matrix; }
    }
    
    pub fn execute(&self, callback: impl FnOnce(),
                   vertex_buffer: &Vec<Vertex>,
                   width: u32,
                   height: u32,
                   device: &Device
    ) {
        let texture_desc = TextureDescriptor::new();
        texture_desc.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
        texture_desc.set_width(width as u64);
        texture_desc.set_height(height as u64);
        texture_desc.set_usage(MTLTextureUsage::ShaderRead | MTLTextureUsage::RenderTarget);
        let texture = device.new_texture(&texture_desc);
        
        let render_pass = RenderPassDescriptor::new();
        render_pass.color_attachments()
            .object_at(0)
            .unwrap()
            .set_texture(Some(texture.as_ref()));
        render_pass.color_attachments()
            .object_at(0)
            .unwrap()
            .set_load_action(MTLLoadAction::Clear);
        render_pass.color_attachments()
            .object_at(0)
            .unwrap()
            .set_store_action(MTLStoreAction::Store);
        render_pass.color_attachments()
            .object_at(0)
            .unwrap()
            .set_clear_color(MTLClearColor::new(0.0, 0.0, 0.0, 1.0));
        let cmd = self.command_queue.new_command_buffer();
        let encoder = cmd.new_render_command_encoder(&render_pass);
        encoder.set_render_pipeline_state(&self.pipeline_state);
        encoder.set_vertex_buffer(0, Some(&self.vertex_buffer), 0);
        encoder.set_vertex_buffer(1, Some(&self.perspective_matrix), 0);
        encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, vertex_buffer.len() as NSUInteger);
        encoder.end_encoding();
        cmd.commit();
        
        callback();
        
        cmd.wait_until_completed();
    }
}

