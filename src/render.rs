use crate::ecs::system::System;

use winit::window::Window;

use wgpu::{BackendBit, Adapter, PowerPreference, DeviceDescriptor, Surface, Device, Queue, SwapChainDescriptor, TextureUsage, TextureFormat, PresentMode, SwapChain};

pub struct Renderer {
    surface: Surface,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    swapchain: SwapChain,
}

impl Renderer {
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

        let window_size = window.inner_size();

        let mut swapchain_desc = SwapChainDescriptor {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: window_size.width,
            height: window_size.height,
            present_mode: PresentMode::NoVsync,
        };

        let swapchain = device.create_swap_chain(&surface, &swapchain_desc);

        Renderer { surface, adapter, device, queue, swapchain }
    }
}

impl System for Renderer {
    fn tick(delta_time: f32) {
        // Things to do:
        // - Launch compute shader for light/object culling
        // - Launch compute shader for object drawing
        // - Draw the actual objects

        unimplemented!()
    }
}
