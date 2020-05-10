use iron_oxide::*;

unsafe fn execute() {
    colog::init();

    let devices = MTLCopyAllDevices();
    let device = devices.into_iter().find_map(|d| Some(d)).unwrap();
    log::log!(log::Level::Info, "Name: {}", device.get_name());
    log::log!(
        log::Level::Info,
        "{} headless",
        match device.is_headless() {
            true => "Is",
            false => "Is not",
        }
    );
    log::log!(
        log::Level::Info,
        "{} low powered",
        match device.is_low_power() {
            true => "Is",
            false => "Is not",
        }
    );
    log::log!(
        log::Level::Info,
        "{} removable",
        match device.is_removable() {
            true => "Is",
            false => "Is not",
        }
    );
    log::log!(
        log::Level::Info,
        "Allocated size: {}",
        device.get_current_allocated_size()
    );
    log::log!(
        log::Level::Info,
        "Threadgroup memory length: {}",
        device.get_max_threadgroup_memory_length()
    );
    log::log!(
        log::Level::Info,
        "Max threads per threadgroup: {}",
        device.get_max_threads_per_threadgroup()
    );
    log::log!(
        log::Level::Info,
        "Max working set size: {}",
        device.get_recommended_max_working_set_size()
    );
    log::log!(
        log::Level::Info,
        "Programmable positions {} supported",
        match device.are_programmable_sample_positions_supported() {
            true => "are",
            false => "are not",
        }
    );
    log::log!(
        log::Level::Info,
        "Default sample positions: {}",
        device.get_default_sample_positions(1)
    );
    log::log!(
        log::Level::Info,
        "Raster order groups {} supported",
        match device.are_raster_order_groups_supported() {
            true => "are",
            false => "are not",
        }
    );
    log::log!(
        log::Level::Info,
        "D24S8 {} supported",
        match device.is_d24_s8_pixel_format_supported() {
            true => "is",
            false => "is not",
        }
    );
    log::log!(
        log::Level::Info,
        "Max buffer length: {}",
        device.get_max_buffer_length()
    );
    log::log!(
        log::Level::Info,
        "Sample count of 2 {} supported",
        match device.supports_texture_sample_count(2) {
            true => "is",
            false => "is not",
        }
    );

    let queue = device.new_command_queue();
    debug(&queue);

    let library = device
        .new_library_with_data(include_bytes!("quad.metallib"))
        .unwrap();
    log::log!(log::Level::Info, "{:?}", library.get_function_names());
    let vertex = library.new_function_with_name("vertex_shader").unwrap();
    log::log!(
        log::Level::Info,
        "Vertex function is of type: {}",
        match vertex.get_function_type() {
            MTLFunctionType::Vertex => "vertex... duh",
            MTLFunctionType::Fragment => "fragment... ???",
            MTLFunctionType::Kernel => "kernel... ???",
        }
    );
    log::log!(
        log::Level::Info,
        "Vertex function is called: {}",
        vertex.get_name()
    );
    let fragment = library.new_function_with_name("fragment_shader").unwrap();

    let desc = MTLRenderPipelineDescriptor::new();
    desc.set_vertex_function(&vertex);
    desc.set_fragment_function(&fragment);
    let color_attachments = desc.get_color_attachments();
    color_attachments.set_object_at_indexed_subscript(0, &{
        let primary_color_attachment = MTLRenderPipelineColorAttachmentDescriptor::new();
        primary_color_attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        primary_color_attachment.set_write_mask(
            MTLColorWriteMask::Red
                | MTLColorWriteMask::Green
                | MTLColorWriteMask::Blue
                | MTLColorWriteMask::Alpha,
        );
        primary_color_attachment
    });
    let render_pipeline = device
        .new_render_pipeline_state_with_descriptor(&desc)
        .unwrap();
    debug(&render_pipeline);

    let buffer = device.new_buffer_with_length(
        8,
        MTLResourceOptions::new()
            .set_storage_mode(MTLStorageMode::Shared)
            .set_cpu_cache_mode(MTLCPUCacheMode::Default),
    );
    debug(&buffer);
    log::log!(
        log::Level::Info,
        "Buffer's CPU cache mode: {}",
        buffer.get_cpu_cache_mode() as u64
    );
    println!(
        "Buffer contents: {:?}",
        &*(buffer.get_contents() as *mut [u8; 8])
    );

    let texture = device.new_texture_with_descriptor(&{
        let desc = MTLTextureDescriptor::new();
        desc.set_width(10);
        desc.set_height(10);
        desc.set_usage(MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite);
        desc.set_texture_type(MTLTextureType::D2);
        desc.set_resource_options(
            MTLResourceOptions::new()
                .set_storage_mode(MTLStorageMode::Private)
                .set_cpu_cache_mode(MTLCPUCacheMode::WriteCombined),
        );
        desc
    });
    debug(&texture);

    let sampler = device.new_sampler_state_with_descriptor(&{
        let desc = MTLSamplerDescriptor::new();
        desc
    });
    debug(&sampler);

    let depth_stencil = device.new_depth_stencil_state_with_descriptor(&{
        let desc = MTLDepthStencilDescriptor::new();
        desc.set_depth_write_enabled(true);
        desc.set_depth_compare_function(MTLCompareFunction::Less);
        desc
    });
    debug(&depth_stencil);

    let command_buffer = queue.new_command_buffer(true);
    command_buffer.commit();

    let layer = CAMetalLayer::new();
    layer.set_device(&device);
    layer.set_vsync(true);
    layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    layer.set_framebuffer_only(true);
    layer.set_drawable_size(CGSize {
        width: 500.0,
        height: 500.0,
    });

    let rp_desc = MTLRenderPassDescriptor::new();
    rp_desc
        .get_color_attachments()
        .set_object_at_indexed_subscript(0, &{
            let color = MTLRenderPassColorAttachmentDescriptor::new();
            color.set_load_action(MTLLoadAction::Clear);
            color.set_store_action(MTLStoreAction::Store);
            color.set_texture(&texture);
            color
        });
    rp_desc.get_depth_attachment();
}

fn main() {
    unsafe { execute() };
}
