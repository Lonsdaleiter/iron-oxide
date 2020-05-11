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
}

fn main() {
    unsafe { execute() };
}
