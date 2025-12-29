use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { position: [x, y, z] }
    }
}

pub const fn get_vertices() -> [Vertex; 24] {
    #[rustfmt::skip]
    let vertices: [Vertex; 24] = [
        // Front
        Vertex::new(-1.0, -1.0,  1.0), Vertex::new( 1.0, -1.0,  1.0), Vertex::new(1.0,  1.0,  1.0), Vertex::new(-1.0,  1.0,  1.0),
        // Back
        Vertex::new(-1.0, -1.0, -1.0), Vertex::new(-1.0,  1.0, -1.0), Vertex::new(1.0,  1.0, -1.0), Vertex::new(1.0, -1.0, -1.0),
        // Top
        Vertex::new(-1.0,  1.0, -1.0), Vertex::new(-1.0,  1.0,  1.0), Vertex::new(1.0,  1.0,  1.0), Vertex::new(1.0,  1.0, -1.0),
        // Bottom
        Vertex::new(-1.0, -1.0, -1.0), Vertex::new( 1.0, -1.0, -1.0), Vertex::new(1.0, -1.0,  1.0), Vertex::new(-1.0, -1.0,  1.0),
        // Right
        Vertex::new( 1.0, -1.0, -1.0), Vertex::new( 1.0,  1.0, -1.0), Vertex::new(1.0,  1.0,  1.0), Vertex::new(1.0, -1.0,  1.0),
        // Left
        Vertex::new(-1.0, -1.0, -1.0), Vertex::new(-1.0, -1.0,  1.0), Vertex::new(-1.0,  1.0,  1.0), Vertex::new(-1.0,  1.0, -1.0),
    ];
    vertices
}

pub const fn get_indices() -> &'static [u16] {
    #[rustfmt::skip]
    let indices: &[u16] = &[
         0,  1,  2,  2,  3,  0, // Front
         4,  5,  6,  6,  7,  4, // Back
         8,  9, 10, 10, 11,  8, // Top
        12, 13, 14, 14, 15, 12, // Bottom
        16, 17, 18, 18, 19, 16, // Right
        20, 21, 22, 22, 23, 20, // Left
    ];
    indices
}

pub fn get_vertex_buffer(label: Option<&str>, device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label,
        contents: bytemuck::cast_slice(&get_vertices()),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

pub fn get_index_buffer(label: Option<&str>, device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label,
        contents: bytemuck::cast_slice(get_indices()),
        usage: wgpu::BufferUsages::INDEX,
    })
}
