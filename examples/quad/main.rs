use iron_oxide::{MTLCompileOptions, MTLCopyAllDevices};

fn main() {
    unsafe {
        let devices = MTLCopyAllDevices();
        let device = devices.into_iter().find_map(|d| Some(d)).unwrap();

        let queue = device.new_command_queue();

        let library = device
            .new_library_with_source(include_str!("quad.metal"), &MTLCompileOptions::new())
            .unwrap();
        let vertex = library.new_function_with_name("quad_v");
        let fragment = library.new_function_with_name("quad_f");
    }
}
