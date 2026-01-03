use ctor::ctor;
use std::env;

#[ctor]
pub fn init_environment() {
        // 1. Detect if we are on Wayland
    let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();
    println!("XDG_SESSION_TYPE: {}", session_type);

    if session_type == "wayland" {
        // Force the windowing system to X11 (XWayland)
    unsafe {std::env::set_var("GDK_BACKEND", "x11");

    // Fix the GBM buffer error by forcing WebKit to use a different rendering path
    // This disables the DMABUF renderer which often fails on XWayland/NVIDIA
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    
    // Optional: If you still see glitches, disable hardware acceleration entirely
    // std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }
    println!("Wayland detected. Forcing X11 backend and disabling DMABUF...");
    }
  println!("XDG_SESSION_TYPE: {}", session_type);
}