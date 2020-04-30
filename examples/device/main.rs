use iron_oxide::*;

unsafe fn execute() {
    let devices = MTLCopyAllDevices();
    let device = devices.into_iter().find_map(|d| Some(d)).unwrap();
    println!("Name: {}", device.get_name());
    println!("{} headless", match device.is_headless() {
        true => "Is",
        false => "Is not",
    });
    println!("{} low powered", match device.is_low_power() {
        true => "Is",
        false => "Is not",
    });
    println!("{} removable", match device.is_removable() {
        true => "Is",
        false => "Is not",
    });
    println!("Allocated size: {}", device.get_current_allocated_size());
    println!("Threadgroup memory length: {}", device.get_max_threadgroup_memory_length());
    println!("Max threads per threadgroup: {}", device.get_max_threads_per_threadgroup());
    println!("Max working set size: {}", device.get_recommended_max_working_set_size());
}

fn main() {
    unsafe { execute() };
}
