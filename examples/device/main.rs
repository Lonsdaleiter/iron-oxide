use iron_oxide::*;

unsafe fn execute() {
    let devices = MTLCopyAllDevices();
    let device = devices.into_iter().find_map(|d| Some(d)).unwrap();
    println!("Device name: {}", device.get_name());
}

fn main() {
    unsafe { execute() };
}
