use iron_oxide::{
    Array, MTLCPUCacheMode, MTLCompileOptions, MTLCopyAllDevices, MTLPixelFormat,
    MTLRenderPipelineColorAttachmentDescriptor, MTLRenderPipelineDescriptor, MTLResourceOptions,
    MTLStorageMode, MTLVertexDescriptor,
};
use std::os::raw::c_void;

fn main() {
    unsafe {
        let devices = MTLCopyAllDevices();
        let device = devices.into_iter().find_map(|d| Some(d)).unwrap();

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

        let quad_bytes = [ // TODO add the other triangle
            -1.0f32, -1.0, 0.0, // v1
            0.0, 0.0, 0.0, 1.0, // black
            1.0, -1.0, 0.0, // v2
            1.0, 0.0, 0.0, 1.0, // red
            1.0, 1.0, 0.0, // v3
        ];
        let quad = device.new_buffer_with_bytes(
            quad_bytes.as_ptr() as *const c_void,
            quad_bytes.len() as u64 * 4,
            MTLResourceOptions::new()
                .set_cpu_cache_mode(MTLCPUCacheMode::Default)
                .set_storage_mode(MTLStorageMode::Private),
        );
    }
}
