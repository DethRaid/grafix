use wgpu::{Buffer, Device, BufferUsage};
use cgmath::{Vector3, Matrix4};
use std::rc::Rc;
use std::sync::Arc;

type MeshId = usize;

pub struct Model {
    pub model_matrix: Matrix4<f32>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

pub struct ModelStorage {
    device: Arc<Device>,

    models: Vec<Model>,

}

impl ModelStorage {
    pub fn with_device(device: Arc<Device>) -> Self {
        ModelStorage { device, models: vec![] }
    }

    /// Adds a mesh with the provided index and vertex data
    pub fn add_mesh(&mut self, model_matrix: Matrix4<f32>, vertices: &[Vector3<f32>], indices: &[u32]) -> MeshId {
        let vertex_buffer = device.create_buffer_with_data(vertices.as_bytes(), BufferUsage::VERTEX);
        let index_buffer = device.create_buffer_with_data(indices.as_bytes(), BufferUsage::INDEX);

        let id: MeshId = self.models.len();

        let mesh = Model { model_matrix, vertex_buffer, index_buffer };

        self.models.push(mesh);

        id
    }

    pub fn all_models(&self) -> &[Model] {
        self.models.as_slice()
    }
}
