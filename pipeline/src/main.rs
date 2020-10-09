use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}
};
use std::iter;

struct State {
    surface: wgpu::Surface,             // 表面
    device: wgpu::Device,               // デバイス
    queue: wgpu::Queue,                 // キュー
    sc_desc: wgpu::SwapChainDescriptor,  
    swap_chain: wgpu::SwapChain,         
    size: winit::dpi::PhysicalSize<u32>,// サイズ 
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // adapter, surfaceの作成 backend指定 
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        // make surface
        let surface = unsafe { instance.create_surface(window) };
        // enumerate adapter
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default, // power preference
            compatible_surface: Some(&surface), // 互換性
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            // device
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                shader_validation: true,
            },
            None,
        ).await.unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let clear_color = wgpu::Color::BLACK;

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            clear_color,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved{ position, .. } => {
                self.clear_color = wgpu::Color {
                    r: position.x as f64 / self.size.width as f64,
                    g: position.y as f64 / self.size.height as f64,
                    b: 1.0,
                    a: 1.0,
                };
                true
            }
            _ => false
        }
    }

    fn update(&mut self) {

    }

    fn render(&mut self) {
        // return texture(frame)
        let frame = self.swap_chain.get_current_frame()
                        .expect("Timeout getting texture")
                        .output;
        // GPU operations encode
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            // render pass の記録開始
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.clear_color),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    use futures::executor::block_on;

    let mut state = block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { ref event, window_id, } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        },
        Event::RedrawRequested(_) => {
            state.update();
            state.render();
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}

