#![allow(dead_code, unused_variables, clippy::too_many_arguments, clippy::unnecessary_wraps)]

//Modules
mod app;
mod data;
mod instance;
mod devices;
mod swapchain;
mod pipeline;
mod buffers;
mod semaphores;
mod queue_family;
mod vertex;
mod textures;

//External Modules
use anyhow::Result;
use lazy_static::lazy_static;
use nalgebra_glm as glm;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::Version;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

//Internal Modules
use crate::app::App;
use crate::vertex::Vertex;

/// Whether the validation layers should be enabled.
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
/// The name of the validation layers.
const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

/// The Vulkan SDK version that started requiring the portability subset extension for macOS.
const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

/// The maximum number of frames that can be processed concurrently.
const MAX_FRAMES_IN_FLIGHT: usize = 2;

lazy_static! {
    static ref VERTICES: Vec<Vertex> = vec![
        Vertex::new(glm::vec2(-0.5, -0.5), glm::vec3(1.0, 1.0, 0.0)),
        Vertex::new(glm::vec2(0.5, -0.5), glm::vec3(0.0, 1.0, 1.0)),
        Vertex::new(glm::vec2(0.5, 0.5), glm::vec3(1.0, 0.0, 1.0)),
        Vertex::new(glm::vec2(-0.5, 0.5), glm::vec3(0.0, 0.0, 0.0)),
    ];
}

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

#[rustfmt::skip]
fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("RustyRenderer")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App
    let mut app = unsafe { App::create(&window)? };
    let mut destroying = false;
    let mut minimized = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            Event::MainEventsCleared if !destroying && !minimized => unsafe { app.render(&window) }.unwrap(),
            // Mark the window as having been resized.
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                if size.width == 0 || size.height == 0 {
                    minimized = true;
                } else {
                    minimized = false;
                    app.resized = true;
                }
            }
            // Destroy our Vulkan app.
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe { app.destroy(); }
            }
            _ => {}
        }
    });
}
