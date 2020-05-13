use iron_oxide::*;
use std::os::raw::c_void;

const IN_DATA: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

fn main() {
    unsafe {
        let devices = MTLCopyAllDevices();
        let device = devices.into_iter().find_map(|d| Some(d)).unwrap();

        let queue = device.new_command_queue();

        let library = device
            .new_library_with_source(include_str!("sum.metal"), &MTLCompileOptions::new())
            .unwrap();
        let sum_kernel = library.new_function_with_name("sum").unwrap();
        let sum_pipeline = device
            .new_compute_pipeline_state_with_function(&sum_kernel)
            .unwrap();

        let out_buffer = device.new_buffer_with_length(
            1,
            MTLResourceOptions::new()
                .set_storage_mode(MTLStorageMode::Shared)
                .set_cpu_cache_mode(MTLCPUCacheMode::Default),
        );

        let command_buffer = queue.new_command_buffer(true);

        let encoder = command_buffer.new_compute_encoder();
        encoder.set_compute_pipeline_state(&sum_pipeline);
        encoder.set_bytes(
            IN_DATA.as_ptr() as *const c_void,
            IN_DATA.len() as NSUInteger * 4,
            0,
        );
        encoder.set_buffer(&out_buffer, 0, 1);
        encoder.dispatch_threads(MTLSize {
            width: IN_DATA.len() as NSUInteger,
            height: 1,
            depth: 1,
        }, MTLSize {
            width: IN_DATA.len() as NSUInteger,
            height: 1,
            depth: 1,
        });
        encoder.end_encoding();

        command_buffer.commit();
        command_buffer.wait_until_completed();

        let sum: u32 = *(out_buffer.get_contents() as *const u32);
        println!("{}", sum);
    };
}
