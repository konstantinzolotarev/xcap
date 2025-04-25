use image::RgbaImage;

use crate::error::XCapResult;

use super::{
    impl_monitor::ImplMonitor,
    impl_window::ImplWindow,
    utils::{get_current_screen_buf, get_monitor_info_buf, wayland_detect},
    wayland_capture::wayland_capture,
    xorg_capture::xorg_capture,
};

pub fn capture_full_monitor(impl_monitor: &ImplMonitor) -> XCapResult<RgbaImage> {
    let monitor_info_buf = get_monitor_info_buf(impl_monitor.output)?;

    capture_monitor(
        monitor_info_buf.x(),
        monitor_info_buf.y(),
        monitor_info_buf.width() as i32,
        monitor_info_buf.height() as i32,
    )
}

pub fn capture_monitor(x: i32, y: i32, width: i32, height: i32) -> XCapResult<RgbaImage> {
    if wayland_detect() {
        wayland_capture(x, y, width, height)
    } else {
        let screen_buf = get_current_screen_buf()?;

        xorg_capture(screen_buf.root(), x, y, width as u32, height as u32)
    }
}

pub fn capture_window(impl_window: &ImplWindow) -> XCapResult<RgbaImage> {
    let width = impl_window.width()?;
    let height = impl_window.height()?;

    xorg_capture(impl_window.window, 0, 0, width, height)
}
