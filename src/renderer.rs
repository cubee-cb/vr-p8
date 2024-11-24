use super::VR_MAX_VERTS;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub coords: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct VertexBuffer {
    pub verts: [Vertex; VR_MAX_VERTS as usize]
}
