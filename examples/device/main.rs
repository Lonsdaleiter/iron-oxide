use iron_oxide::*;

unsafe fn execute() {
    let devices = MTLCopyAllDevices();
    let _device = devices.into_iter().find_map(|d| Some(d));
}

fn main() {
    unsafe { execute() };
}
