// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  configure_linux_webkit_environment();
  app_lib::run();
}

#[cfg(target_os = "linux")]
fn configure_linux_webkit_environment() {
  if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
    // Set before WebKitGTK starts; changing process env later is not thread-safe.
    unsafe {
      std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
  }
}

#[cfg(not(target_os = "linux"))]
fn configure_linux_webkit_environment() {}
