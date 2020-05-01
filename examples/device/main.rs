use iron_oxide::*;

unsafe fn execute() {
    let devices = MTLCopyAllDevices();
    let device = devices.into_iter().find_map(|d| Some(d)).unwrap();
    println!("Name: {}", device.get_name());
    println!(
        "{} headless",
        match device.is_headless() {
            true => "Is",
            false => "Is not",
        }
    );
    println!(
        "{} low powered",
        match device.is_low_power() {
            true => "Is",
            false => "Is not",
        }
    );
    println!(
        "{} removable",
        match device.is_removable() {
            true => "Is",
            false => "Is not",
        }
    );
    println!("Allocated size: {}", device.get_current_allocated_size());
    println!(
        "Threadgroup memory length: {}",
        device.get_max_threadgroup_memory_length()
    );
    println!(
        "Max threads per threadgroup: {}",
        device.get_max_threads_per_threadgroup()
    );
    println!(
        "Max working set size: {}",
        device.get_recommended_max_working_set_size()
    );
    println!(
        "Programmable positions {} supported",
        match device.are_programmable_sample_positions_supported() {
            true => "are",
            false => "are not",
        }
    );
    println!(
        "Default sample positions: {}",
        device.get_default_sample_positions(1)
    );
    println!(
        "Raster order groups {} supported",
        match device.are_raster_order_groups_supported() {
            true => "are",
            false => "are not",
        }
    );
    println!(
        "D24S8 {} supported",
        match device.is_d24_s8_pixel_format_supported() {
            true => "is",
            false => "is not",
        }
    );

    let _queue = device.new_command_queue();
    let _l = device.new_library_with_source(include_str!("quad.metal"), MTLCompileOptions::new());
    let library = device
        .new_library_with_data(include_bytes!("quad.metallib"))
        .unwrap();
    println!("{:?}", library.get_function_names());
    let vertex = library.new_function_with_name("vertex_shader").unwrap();
    println!(
        "Vertex function is of type: {}",
        match vertex.get_function_type() {
            MTLFunctionType::Vertex => "vertex... duh",
            MTLFunctionType::Fragment => "fragment... ???",
            MTLFunctionType::Kernel => "kernel... ???",
        }
    );
    println!("Vertex function is called: {}", vertex.get_name());
    let _fragment = library.new_function_with_name("fragment_shader").unwrap();
}

fn main() {
    unsafe { execute() };
}
