mod shader_handling;
mod meshing;
mod chunk;

use metal::Device;
use sdl2::render::{TextureAccess, TextureCreator};
use sdl2::pixels::PixelFormatEnum;
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use shader_handling::{ShaderHandler, Shader};
use crate::meshing::{rotate, Mesh};
use crate::chunk::{generate_cube, Chunk};
use crate::shader_handling::{Float4, Float4x4, Pipeline, Uchar4, Uint4, Vertex};

/// The starting width of the application window
static WINDOW_START_WIDTH: u32 = 1200;
/// The starting height of the application window
static WINDOW_START_HEIGHT: u32 = 750;

/// The minimum size of the window (mostly so ui doesn't get completely messed up)
static MINIMUM_WINDOW_WIDTH: u32 = 1200;
/// The minimum size of the window (mostly so ui doesn't get completely messed up)
static MINIMUM_WINDOW_HEIGHT: u32 = 750;

static MAXIMUM_WINDOW_WIDTH: u64 = 4096u64;
static MAXIMUM_WINDOW_HEIGHT: u64 = 4096u64;

static _GAME_VERSION: &'static str = "0.0.1-alpha";

static MAX_VERTICES: u64 = 250_000_u64;   // HOW?????? how did it even handle this while still be quick???????? that shouldn't work
static MAX_TRIANGLES: u64 = 125_000_u64;
static MAX_TEXTURES: u64 = 1024_u64;

static TILE_TEXTURE_WIDTH: u64 = 16u64;
static TILE_TEXTURE_HEIGHT: u64 = 16u64;

static CELL_SIZE: u32 = 4;  // seems like a good size for performance; 16 was much slower

pub fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    
    // Create window
    let mut window = video
        .window("Name of Game (todo!)", WINDOW_START_WIDTH, WINDOW_START_HEIGHT)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    window.set_minimum_size(MINIMUM_WINDOW_WIDTH, MINIMUM_WINDOW_HEIGHT)
        .map_err(|e| e.to_string())?;
    
    // --- Create an SDL2 surface and texture ---
    let (_device_width, _device_height) = (video.desktop_display_mode(0)?.w, video.desktop_display_mode(0)?.h);
    let mut window_surface = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    
    // creating the texture that all runtime drawing will be done to
    // this texture will then be uploaded onto the window_surface
    let texture_creator: TextureCreator<WindowContext> = window_surface.texture_creator();
    let mut surface_texture = texture_creator
        .create_texture(PixelFormatEnum::RGB24, TextureAccess::Streaming,   WINDOW_START_WIDTH, WINDOW_START_HEIGHT)
        .map_err(|e| e.to_string())?;
    let mut surface_texture_size = (WINDOW_START_WIDTH, WINDOW_START_HEIGHT);
    
    let mut event_pump = sdl.event_pump()?;
    
    let device = Device::system_default().unwrap();
    let shader = Shader::new(&device, "shaders/triangles.metal", &[
        size_of::<u32   >() as u64,
        size_of::<u32   >() as u64,
        size_of::<u32   >() as u64,
        size_of::<Float4>() as u64,
        size_of::<Float4>() as u64,
        size_of::<Float4>() as u64 * MAX_VERTICES,
        size_of::<Float4>() as u64 * MAX_TRIANGLES,
        size_of::<Uint4 >() as u64 * MAX_TRIANGLES,
        size_of::<u32   >() as u64 * ((MAXIMUM_WINDOW_WIDTH / CELL_SIZE as u64) * (MAXIMUM_WINDOW_HEIGHT / CELL_SIZE as u64) * 64),
        size_of::<Uchar4 >() as u64 * MAX_TEXTURES * (TILE_TEXTURE_WIDTH * TILE_TEXTURE_HEIGHT),
        size_of::<f32   >() as u64 * (MAXIMUM_WINDOW_HEIGHT * MAXIMUM_WINDOW_WIDTH),
        size_of::<u8    >() as u64 * (MAXIMUM_WINDOW_HEIGHT * MAXIMUM_WINDOW_WIDTH),
    ], "ComputeShader")?;
    let mut shader_handler = ShaderHandler::new(device, shader);
    
    let mut texture = vec![];
    for i in 0..=255 {
        if i / 16 > 4 {
            texture.push(Uchar4::new(150, 75, 10, 0));
        } else {
            texture.push(Uchar4::new(75, 225, 75, 0));
        }
    }
    for _ in 0..=255 {
        texture.push(Uchar4::new(75, 225, 75, 0));
    }
    for _ in 0..=255 {
        texture.push(Uchar4::new(150, 75, 10, 0));
    }
    shader_handler.get_shader().update_buffer_slice(9, &texture)?;
    
    let mut camera_position = Float4::new(0.0, 2.0, -2.0, 0.0);
    let mut camera_rotation = Float4::new(0.0, 0.0, 0.0, 0.0);
    let mut vertex_buffer: Vec<Vertex> = vec![];
    let mut triangles_buffer: Vec<Uint4> = vec![];
    let mut normals: Vec<Float4> = vec![
        Float4::new( 0.0,  1.0,  0.0, 0.0),
        Float4::new( 0.0, -1.0,  0.0, 0.0),
        Float4::new( 1.0,  0.0,  0.0, 0.0),
        Float4::new(-1.0,  0.0,  0.0, 0.0),
        Float4::new( 0.0,  0.0,  1.0, 0.0),
        Float4::new( 0.0,  0.0, -1.0, 0.0),
    ];
    
    let depth_buffer = vec![f32::MAX; const { (MAXIMUM_WINDOW_WIDTH * MAXIMUM_WINDOW_HEIGHT) as usize }];
    
    let mut mesh = Mesh {
        vertices_original: vertex_buffer.clone(),
        vertices: vec![Vertex::default(); MAX_VERTICES as usize],
        indices: triangles_buffer,
        normals,
        binned_indices: vec![0u32; 64 * (MAXIMUM_WINDOW_WIDTH / CELL_SIZE as u64) as usize * (MAXIMUM_WINDOW_HEIGHT / CELL_SIZE as u64) as usize],
        mutated: true,
    };
    for chunk_x in 0..7 {
        for chunk_z in 0..7 {
            let mut chunk = Chunk::new(Float4::new(chunk_x as f32 * 16.0, 0.0, chunk_z as f32 * 16.0, 0.0));
            for x in 0..16 {
                for z in 0..16 {
                    for y in 0..rand::random_range(2..6) {
                        chunk.tile_data[x][y][z] = 1;  // setting some tiles to be solid
                    }
                }
            }
            chunk.remesh_chunk(&mut mesh, 1.0);
        }
    }
    println!("Mesh has {} vertices and {} triangles.", mesh.vertices_original.len(), mesh.indices.len());
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    // --- Main loop ---
    'running: loop {
        let frame_start = std::time::Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        let mut movement = Float4::new(0.0, 0.0, 0.0, 0.0);
                        match key {
                            sdl2::keyboard::Keycode::A => {
                                movement.x += 4.0;
                                mesh.mutated = true;
                            },
                            sdl2::keyboard::Keycode::D => {
                                movement.x -= 4.0;
                                mesh.mutated = true;
                            },
                            sdl2::keyboard::Keycode::W => {
                                movement.z += 4.0;
                                mesh.mutated = true;
                            },
                            sdl2::keyboard::Keycode::S => {
                                movement.z -= 4.0;
                                mesh.mutated = true;
                            },
                            sdl2::keyboard::Keycode::Space => {
                                movement.y += 4.0;
                                mesh.mutated = true;
                            },
                            sdl2::keyboard::Keycode::LSHIFT => {
                                movement.y -= 4.0;
                                mesh.mutated = true;
                            },
                            sdl2::keyboard::Keycode::Left => {
                                camera_rotation.y -= 0.1;
                                mesh.mutated = true;
                            },
                            sdl2::keyboard::Keycode::Right => {
                                camera_rotation.y += 0.1;
                                mesh.mutated = true;
                            },
                            sdl2::keyboard::Keycode::Up => {
                                camera_rotation.x += 0.025;
                                mesh.mutated = true;
                            },
                            sdl2::keyboard::Keycode::Down => {
                                camera_rotation.x -= 0.025;
                                mesh.mutated = true;
                            },
                            _ => {}
                        }
                        let offset = rotate(movement, &camera_rotation.negate());
                        camera_position.x += offset.x;
                        camera_position.y += offset.y;
                        camera_position.z += offset.z;
                    }
                }
                _ => {}
            }
        }
        
        // checking the surface texture's size
        let window_size = window_surface.output_size()?;
        if surface_texture_size != window_size {
            mesh.mutated = true;
            surface_texture = texture_creator
                .create_texture(PixelFormatEnum::RGB24, TextureAccess::Streaming, window_size.0, window_size.1)
                .map_err(|e| e.to_string())?;
            surface_texture_size = window_size;
        }
        
        if window_size.0 as u64 > MAXIMUM_WINDOW_WIDTH || window_size.1 as u64 > MAXIMUM_WINDOW_HEIGHT {
            return Err(format!("Window size exceeded maximum dimensions of {}x{}.", MAXIMUM_WINDOW_WIDTH, MAXIMUM_WINDOW_HEIGHT));
        }
        
        // !====! Do Rendering Here! !====!
        
        // rendering
        // creating a pixel buffer to pass around to reduce draw calls as the cpu is faster than repeatedly waiting for the gpu to return data
        // the gpu is fast, but data moves between the gpu and cpu slowly
        let _buffer_result = surface_texture.with_lock(None, |pixels, pitch| {
            let start = std::time::Instant::now();
            pixels.fill(0);  // clearing the pixel buffer
            shader_handler.get_shader().update_buffer(0, pitch as u32  ).unwrap();
            shader_handler.get_shader().update_buffer(1, window_size.0 ).unwrap();
            shader_handler.get_shader().update_buffer(2, window_size.1 ).unwrap();
            
            mesh.check_remesh(&mut shader_handler, window_size, camera_position.clone(), camera_rotation.clone());
            
            shader_handler.get_shader().update_buffer_slice(10, &depth_buffer[0..(window_size.0 * window_size.1) as usize]).unwrap();
            shader_handler.get_shader().update_buffer_slice(11, pixels).unwrap();
            
            let grid_size = metal::MTLSize::new(
                //metal::NSUInteger::from(triangles_buffer.len() as u64),
                metal::NSUInteger::from((window_size.0 as f32 / CELL_SIZE as f32).ceil() as u64),
                metal::NSUInteger::from((window_size.1 as f32 / CELL_SIZE as f32).ceil() as u64),
                metal::NSUInteger::from(1u64),
            );
            
            let thread_group_size = metal::MTLSize::new(
                metal::NSUInteger::from(8_u64),  // 8x8 seems to be the best currently? 9-12 ms per iteration
                metal::NSUInteger::from(8_u64),
                metal::NSUInteger::from(1_u64),
            );
            
            let execution_start = start.elapsed();
            
            shader_handler.get_shader().execute::<()>(grid_size, thread_group_size, Some(|| {
                // runs while rendering is happening
                Ok(())
            })).unwrap();
            
            let execution_end = start.elapsed() - execution_start;
            
            let contents: *mut &[u8] = shader_handler.get_shader().get_buffer_contents(11);
            if contents.is_null() { panic!("Null pointer when unwrapping shader pixel result for triangle rendering."); }
            pixels.copy_from_slice(
                unsafe {
                    std::slice::from_raw_parts(contents as *const u8, pixels.len())
                }
            );
            
            let total_end = start.elapsed() - execution_end - execution_start;
            println!("Buffer Upload Time: {:?}, Execution Time: {:?}, Read In Time: {:?}", execution_start, execution_end, total_end);
        })?;
        // !====! No Rendering Beyond Here !====!
        
        // clearing and drawing the texture
        window_surface.clear();
        window_surface.copy(&surface_texture, None, Rect::new(0, 0, window_size.0, window_size.1))?;

        // flushing the screen and stuff
        window_surface.present();
        
        let frame_time = frame_start.elapsed();
        println!("Frame Time: {:?}\nFPS: {}", frame_time, 1.0 / frame_time.as_secs_f32());
    } Ok(())
}
