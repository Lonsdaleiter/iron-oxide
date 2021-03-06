use iron_oxide::*;
use std::os::raw::c_void;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

struct MetalBoilerplate {
    _device: MTLDevice,
    layer: CAMetalLayer,
    queue: MTLCommandQueue,
    quad_pipeline: MTLRenderPipelineState,
    quad_buffer: MTLBuffer,
    quad_indices: MTLBuffer,
}

const QUAD_LEN: NSUInteger = 6;
const QUAD_VERTS: NSUInteger = 24;
const QUAD_BYTES: [f32; QUAD_VERTS as usize] = [
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

        let quad_buffer = device.new_buffer_with_bytes(
            QUAD_BYTES.as_ptr() as *const c_void,
            QUAD_VERTS * 4,
            MTLResourceOptions::new()
                .set_cpu_cache_mode(MTLCPUCacheMode::WriteCombined)
                .set_storage_mode(MTLStorageMode::Managed),
        );
        quad_buffer.did_modify_range(0..QUAD_VERTS * 4);

        let quad_indices = device.new_buffer_with_bytes(
            QUAD_INDICES.as_ptr() as *const c_void,
            QUAD_LEN * 4,
            MTLResourceOptions::new()
                .set_cpu_cache_mode(MTLCPUCacheMode::WriteCombined)
                .set_storage_mode(MTLStorageMode::Managed),
        );
        quad_indices.did_modify_range(0..QUAD_LEN * 4);

        MetalBoilerplate {
            _device: device,
            layer,
            queue,
            quad_pipeline,
            quad_buffer,
            quad_indices,
        }
    }
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
                if let Some(drawable) = boilerplate.layer.next_drawable() {
                    let command_buffer = boilerplate.queue.new_command_buffer(true);

                    let encoder = command_buffer.new_render_command_encoder_with_descriptor(&{
                        let desc = MTLRenderPassDescriptor::new();
                        desc.get_color_attachments()
                            .set_object_at_indexed_subscript(0, &{
                                let desc = MTLRenderPassColorAttachmentDescriptor::new();
                                desc.set_texture(&drawable.get_texture());
                                desc.set_load_action(MTLLoadAction::Clear);
                                desc.set_store_action(MTLStoreAction::Store);
                                desc
                            });
                        desc
                    });
                    encoder.set_vertex_buffer(&boilerplate.quad_buffer, 0, 0);
                    encoder.set_render_pipeline_state(&boilerplate.quad_pipeline);
                    encoder.draw_indexed_primitives(
                        MTLPrimitiveType::Triangle,
                        QUAD_LEN,
                        MTLIndexType::UInt16,
                        &boilerplate.quad_indices,
                        0,
                        1,
                        0,
                        0,
                    );
                    encoder.end_encoding();

                    command_buffer.present_drawable(&drawable);
                    command_buffer.commit();
                    command_buffer.wait_until_completed();
                }
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
