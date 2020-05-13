use iron_oxide::*;
use std::fs::File;
use std::os::raw::c_void;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent, StartCause};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use std::time::{Duration, Instant};

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
const QUAD_VERTS: NSUInteger = 16;
const QUAD_BYTES: [f32; QUAD_VERTS as usize] = [
    // TODO add the other triangle
    -1.0f32, -1.0, // v1
    0.0, 1.0, // tc1
    1.0, -1.0, // v2
    1.0, 1.0, // tc2
    1.0, 1.0, // v3
    1.0, 0.0, // tc3
    -1.0, 1.0, // v4
    0.0, 0.0, // tc4
];
const QUAD_INDICES: [u16; QUAD_LEN as usize] = [0, 1, 2, 2, 3, 0];

struct RenderState {
    quad_pipeline: MTLRenderPipelineState,
    quad_buffer: MTLBuffer,
    quad_indices: MTLBuffer,
}

impl RenderState {
    unsafe fn new(boilerplate: &MetalBoilerplate) -> RenderState {
        let quad_vertex = boilerplate
            .library
            .new_function_with_name("quad_v")
            .unwrap();
        let quad_fragment = boilerplate
            .library
            .new_function_with_name("quad_f")
            .unwrap();

        let quad_pipeline = boilerplate
            .device
            .new_render_pipeline_state_with_descriptor(&{
                let desc = MTLRenderPipelineDescriptor::new();
                desc.set_vertex_descriptor(&MTLVertexDescriptor::new());
                desc.set_vertex_function(&quad_vertex);
                desc.set_fragment_function(&quad_fragment);
                desc.get_color_attachments()
                    .set_object_at_indexed_subscript(0, &{
                        let desc = MTLRenderPipelineColorAttachmentDescriptor::new();
                        desc.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
                        desc.set_blending_enabled(true);
                        desc.set_source_rgb_blend_factor(MTLBlendFactor::SourceAlpha);
                        desc.set_destination_rgb_blend_factor(MTLBlendFactor::OneMinusSourceAlpha);
                        desc.set_source_alpha_blend_factor(MTLBlendFactor::One);
                        desc.set_destination_alpha_blend_factor(
                            MTLBlendFactor::OneMinusSourceAlpha,
                        );
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

struct GameState {
    compute_pipeline: MTLComputePipelineState,
    cell_state: MTLTexture,
    sampler: MTLSamplerState,
    size: (NSUInteger, NSUInteger),
}

impl GameState {
    unsafe fn new(boilerplate: &MetalBoilerplate) -> GameState {
        let decoder = png::Decoder::new(File::open("examples/gol/gol_init_state.png").unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut img = vec![0; info.buffer_size()];
        reader.next_frame(&mut img).unwrap();

        let cell_state = boilerplate.device.new_texture_with_descriptor(&{
            let desc = MTLTextureDescriptor::new();
            desc.set_width(info.width as NSUInteger);
            desc.set_height(info.height as NSUInteger);
            desc.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
            desc.set_texture_type(MTLTextureType::D2);
            desc.set_usage(MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite);
            desc
        });
        cell_state.replace_region(
            MTLRegion {
                origin: MTLSize {
                    width: 0,
                    height: 0,
                    depth: 0,
                },
                size: MTLSize {
                    width: info.width as NSUInteger,
                    height: info.height as NSUInteger,
                    depth: 1,
                },
            },
            0,
            0,
            img.as_ptr() as *const c_void,
            info.width as NSUInteger * 4,
            0,
        );

        let compute_fn = boilerplate
            .library
            .new_function_with_name("update_game")
            .unwrap();
        let compute_pipeline = boilerplate
            .device
            .new_compute_pipeline_state_with_function(&compute_fn)
            .unwrap();

        let sampler = boilerplate.device.new_sampler_state_with_descriptor(&{
            let desc = MTLSamplerDescriptor::new();
            desc.set_min_filter(MTLSamplerMinMagFilter::Nearest);
            desc.set_mag_filter(MTLSamplerMinMagFilter::Nearest);
            desc
        });

        GameState {
            compute_pipeline,
            cell_state,
            sampler,
            size: (info.width as NSUInteger, info.height as NSUInteger)
        }
    }
}

fn main() {
    colog::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(500, 500))
        .with_title("Conway's Game of Life")
        .build(&event_loop)
        .unwrap();

    let boilerplate = unsafe { MetalBoilerplate::new(&window) };
    let render_state = unsafe { RenderState::new(&boilerplate) };
    let compute_state = unsafe { GameState::new(&boilerplate) };

    let duration = Duration::from_millis(100);
    let mut now = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(now + duration);

        match event {
            Event::NewEvents(cause) => {
                match cause {
                    StartCause::ResumeTimeReached { start: _, requested_resume: _ } => {
                        now = Instant::now();
                        window.request_redraw();
                    },
                    _ => {},
                }
            },
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
                                desc.set_clear_color(MTLClearColor {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.0,
                                });
                                desc
                            });
                        desc
                    });
                    encoder.set_vertex_buffer(&render_state.quad_buffer, 0, 0);
                    encoder.set_render_pipeline_state(&render_state.quad_pipeline);
                    encoder.set_fragment_texture(&compute_state.cell_state, 0);
                    encoder.set_fragment_sampler_state(&compute_state.sampler, 0);
                    encoder.draw_indexed_primitives(
                        MTLPrimitiveType::Triangle,
                        QUAD_LEN,
                        MTLIndexType::UInt16,
                        &render_state.quad_indices,
                        0,
                        1,
                        0,
                        0,
                    );
                    encoder.end_encoding();

                    let encoder = command_buffer.new_compute_encoder();
                    encoder.set_compute_pipeline_state(&compute_state.compute_pipeline);
                    encoder.set_texture(&compute_state.cell_state, 0);
                    encoder.set_texture(&compute_state.cell_state, 1);
                    encoder.dispatch_threadgroups(MTLSize {
                        width: compute_state.size.0 / 10,
                        height: compute_state.size.1 / 10,
                        depth: 1,
                    }, MTLSize {
                        width: 10,
                        height: 10,
                        depth: 1
                    });
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
