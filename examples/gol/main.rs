use iron_oxide::*;
use winit::window::{WindowBuilder, Window};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use std::os::raw::c_void;

struct MetalBoilerplate {
    device: MTLDevice,
    layer: CAMetalLayer,
    queue: MTLCommandQueue,
    library: MTLLibrary,
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
            .new_library_with_source(include_str!("gol.metal"), &MTLCompileOptions::new())
            .unwrap();

        MetalBoilerplate {
            device,
            layer,
            queue,
            library,
        }
    }
}

const QUAD_LEN: NSUInteger = 6;
const QUAD_VERTS: NSUInteger = 24;
const QUAD_BYTES: [f32; QUAD_VERTS as usize] = [
    // TODO add the other triangle
    -1.0f32, -1.0, // v1
    0.0, 0.0, 0.0, 1.0, // black
    1.0, -1.0, // v2
    1.0, 0.0, 0.0, 1.0, // red
    1.0, 1.0, // v3
    0.0, 1.0, 0.0, 1.0, // green
    -1.0, 1.0, // v4
    0.0, 0.0, 1.0, 1.0, // blue
];
const QUAD_INDICES: [u16; QUAD_LEN as usize] = [0, 1, 2, 2, 3, 0];

struct RenderState {
    quad_pipeline: MTLRenderPipelineState,
    quad_buffer: MTLBuffer,
    quad_indices: MTLBuffer,
}

impl RenderState {
    unsafe fn new(boilerplate: &MetalBoilerplate) -> RenderState {
        let quad_vertex = library.new_function_with_name("quad_v").unwrap();
        let quad_fragment = library.new_function_with_name("quad_f").unwrap();

        let quad_pipeline = boilerplate.device
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

        let quad_buffer = boilerplate.device.new_buffer_with_bytes(
            QUAD_BYTES.as_ptr() as *const c_void,
            QUAD_VERTS * 4,
            MTLResourceOptions::new()
                .set_cpu_cache_mode(MTLCPUCacheMode::WriteCombined)
                .set_storage_mode(MTLStorageMode::Managed),
        );
        quad_buffer.did_modify_range(0..QUAD_VERTS * 4);

        let quad_indices = boilerplate.device.new_buffer_with_bytes(
            QUAD_INDICES.as_ptr() as *const c_void,
            QUAD_LEN * 4,
            MTLResourceOptions::new()
                .set_cpu_cache_mode(MTLCPUCacheMode::WriteCombined)
                .set_storage_mode(MTLStorageMode::Managed),
        );
        quad_indices.did_modify_range(0..QUAD_LEN * 4);

        RenderState {
            quad_pipeline,
            quad_buffer,
            quad_indices,
        }
    }
}

struct ComputeState {
    //
}

fn main() {
    colog::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1280, 720))
        .with_title("Quad")
        .build(&event_loop)
        .unwrap();

    let boilerplate = unsafe { MetalBoilerplate::new(&window) };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => unsafe {
                //
            },
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::Resized(_new_size) => {}
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    })
}
