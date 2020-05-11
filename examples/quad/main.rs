use iron_oxide::*;
use std::os::raw::c_void;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{WindowBuilder, Window};
use winit::dpi::PhysicalSize;
use std::time::{Duration, Instant};
use winit::event::{Event, StartCause, WindowEvent};

struct MetalBoilerplate {
    //
}

impl MetalBoilerplate {
    unsafe fn new(window: &Window) -> MetalBoilerplate {
        let devices = MTLCopyAllDevices();
        let device = devices.into_iter().find_map(|d| Some(d)).unwrap();

        let layer = CAMetalLayer::new();
        layer.set_device(&device);
        layer.set_vsync(true);
        window.set_layer(&layer);

        let queue = device.new_command_queue();

        let library = device
            .new_library_with_source(include_str!("quad.metal"), &MTLCompileOptions::new())
            .unwrap();
        let quad_vertex = library.new_function_with_name("quad_v").unwrap();
        let quad_fragment = library.new_function_with_name("quad_f").unwrap();

        let quad_pipeline = device
            .new_render_pipeline_state_with_descriptor(&{
                let desc = MTLRenderPipelineDescriptor::new();
                desc.set_vertex_descriptor(&MTLVertexDescriptor::new());
                desc.set_vertex_function(&quad_vertex);
                desc.set_fragment_function(&quad_fragment);
                desc.get_color_attachments()
                    .set_object_at_indexed_subscript(0, &{
                        let desc = MTLRenderPipelineColorAttachmentDescriptor::new();
                        desc.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
                        desc
                    });
                desc
            })
            .unwrap();

        const QUAD_BYTES: [f32; 21] = [ // TODO add the other triangle
            -1.0f32, -1.0, 0.0, // v1
            0.0, 0.0, 0.0, 1.0, // black
            1.0, -1.0, 0.0, // v2
            1.0, 0.0, 0.0, 1.0, // red
            1.0, 1.0, 0.0, // v3
            0.0, 1.0, 0.0, 1.0, // green
        ];
        const SIZE: NSUInteger = QUAD_BYTES.len() as NSUInteger * 4;
        let quad_buffer = device.new_buffer_with_bytes(
            QUAD_BYTES.as_ptr() as *const c_void,
            SIZE,
            MTLResourceOptions::new()
                .set_cpu_cache_mode(MTLCPUCacheMode::WriteCombined)
                .set_storage_mode(MTLStorageMode::Managed),
        );
        quad_buffer.did_modify_range(0..SIZE);

        MetalBoilerplate {}
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1280, 720))
        .with_title("Quad")
        .build(&event_loop)
        .unwrap();

    let boilerplate = unsafe { MetalBoilerplate::new(&window) };

    let duration = Duration::from_millis(17);
    let mut now = Instant::now();
    event_loop.run(move |event, _, control_flow|{
        *control_flow = ControlFlow::WaitUntil(now + duration);

        match event {
            Event::NewEvents(cause) => {
                match cause {
                    StartCause::ResumeTimeReached { start: _, requested_resume: _ } => {
                        now = Instant::now();
                    },
                    _ => {},
                }
            },
            Event::RedrawRequested(_) => {},
            Event::WindowEvent { window_id: _, event } => {
                match event {
                    WindowEvent::Resized(new_size) => {},
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    })
}
