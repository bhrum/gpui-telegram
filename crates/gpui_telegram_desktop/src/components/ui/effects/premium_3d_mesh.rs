//! Upstream: `Telegram/SourceFiles/ui/effects/premium_3d_mesh.cpp:L1-L139`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_3d_mesh.h:L15-L53`
//!
//! Low-overhead 3D geometry mesh representation for star, coin, and diamond models.
//! Encapsulates vertex positions, normals, and face index buffers.

use gpui::*;

#[derive(Debug, Clone)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[derive(Debug, Clone)]
pub struct Premium3dMeshEffects {
    vertices: Vec<Vertex3D>,
    indices: Vec<u32>,
    name: String,
}

impl Premium3dMeshEffects {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            name: name.into(),
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for Premium3dMeshEffects {
    fn default() -> Self {
        Self::new("StarMesh")
    }
}

impl Render for Premium3dMeshEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_0()
            .invisible()
    }
}
