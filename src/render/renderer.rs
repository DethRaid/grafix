use crate::ecs::system::System;

use winit::window::Window;

use wgpu::{BackendBit, Adapter, PowerPreference, DeviceDescriptor, Surface, Device, Queue, SwapChainDescriptor, TextureUsage, TextureFormat, PresentMode, SwapChain, CommandEncoderDescriptor, Buffer, BufferUsage, CreateBufferMapped, BufferDescriptor};
use std::sync::Arc;
use crate::render::mesh_storage::ModelStorage;
use std::mem::size_of;

const NUM_DRAWS: u64 = 1000;
const UNIFORM_SIZE: u64 = 16 * size_of::<f32>();
const MODEL_MATRIX_BUFFER_SIZE: u64 = NUM_DRAWS * UNIFORM_SIZE as u64;

pub struct Renderer {
    surface: Surface,
    adapter: Adapter,
    device: Arc<Device>,
    queue: Queue,
    swapchain: SwapChain,
    mesh_storage: Arc<ModelStorage>,

    /// Buffer for all the current frame's model matrices
    model_matrix_buffer: Buffer,
}

impl Renderer {
    /// Creates a new Renderer that can output to the specified window
    pub fn from_window(window: &Window) -> Self {
        let surface = Surface::create(window);

        let adapter = Adapter::request(&wgpu::RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            backends: BackendBit::DX12,
        }).unwrap();

        let (device, queue) = adapter.request_device(&DeviceDescriptor {
            extensions: Default::default(),
            limits: Default::default(),
        });

        let device = Arc::new(device);

        let window_size = window.inner_size();

        let mut swapchain_desc = SwapChainDescriptor {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: window_size.width,
            height: window_size.height,
            present_mode: PresentMode::NoVsync,
        };

        let swapchain = device.create_swap_chain(&surface, &swapchain_desc);

        let mesh_storage = Arc::new(ModelStorage::with_device(Arc::clone(&device)));

        let model_matrix_buffer = device.create_buffer(&BufferDescriptor {
            size: MODEL_MATRIX_BUFFER_SIZE,
            usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
        });

        Renderer { surface, adapter, device, queue, swapchain, mesh_storage, model_matrix_buffer }
    }

    pub fn get_mesh_storage(&self) -> Arc<ModelStorage> {
        Arc::clone(&self.mesh_storage)
    }
}

impl System for Renderer {
    fn tick(&mut self, delta_time: f32) {
        // Things to do:
        // - Launch compute shader for light/object culling
        // - Launch compute shader for object drawing
        // - Draw the actual objects

        // v1: Render everything in the most boring way possible
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { todo: 0 });

        let mut staging_buffer = self.device.create_buffer_mapped(MODEL_MATRIX_BUFFER_SIZE as usize, BufferUsage::COPY_SRC);

        encoder.copy_buffer_to_buffer(&staging_buffer, MODEL_MATRIX_BUFFER_SIZE, &self.model_matrix_buffer, 0, MODEL_MATRIX_BUFFER_SIZE);

        let mut renderpass = encoder.begin_render_pass();

        for (model, model_matrix_buffer_slot) in self.mesh_storage.all_models().iter().zip(staging_buffer.data.chunks_exact_mut(UNIFORM_SIZE)) {
            model_matrix_buffer_slot.copy_from_slice(model.model_matrix.into());
        }
    }
}
