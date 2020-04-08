use sarekt::renderer::VulkanRenderer;
use winit::event_loop::EventLoop;
use winit::window::Window;
use std::sync::Arc;

fn main() {
    let event_loop = EventLoop::new();
    let window = Arc::new(Window::new(&event_loop).unwrap());
    let renderer = VulkanRenderer::new(window, 800, 600).unwrap();
}
