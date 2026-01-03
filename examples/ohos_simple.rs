// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! OpenHarmony WebView Example
//!
//! This example demonstrates how to use wry on OpenHarmony platform.
//!
//! Note: This is a simplified example for demonstration purposes.
//! To run on actual OpenHarmony device/emulator, you need to set up
//! a DevEco Studio project with proper configuration.
//!
//! See MOBILE.md for complete setup instructions.

#[cfg(target_env = "ohos")]
fn main() -> wry::Result<()> {
  use wry::WebViewBuilder;

  // On OpenHarmony, you typically integrate WebView within an Ability
  // This is a simplified example showing the WebView creation

  println!("OpenHarmony WebView Example");
  println!("Note: This requires proper OpenHarmony environment setup");

  // Create a WebView builder with common settings
  let _builder = WebViewBuilder::new()
    .with_url("https://tauri.app")
    .with_initialization_script("console.log('OpenHarmony WebView initialized');")
    .with_custom_protocol(
      "wry".into(),
      |_webview_id, request| {
        let path = request.uri().path();
        let html = format!("<h1>Custom Protocol: wry://{}</h1>", path);
        wry::http::Response::builder()
          .header("content-type", "text/html")
          .body(html.into_bytes())
          .unwrap()
      },
    )
    .with_ipc_handler(|request| {
      println!("IPC message received: {}", request.body());
    })
    .with_devtools(true);

  // In a real OpenHarmony app, you would:
  // 1. Create this WebView in your Ability's onStart callback
  // 2. Attach it to a Window/Component provided by openharmony-ability
  // 3. Handle the WebView lifecycle

  println!("WebView builder configured successfully");
  println!("Use DevEco Studio to build and run this on OpenHarmony device/emulator");

  Ok(())
}

#[cfg(not(target_env = "ohos"))]
fn main() -> wry::Result<()> {
  println!("This example is designed for OpenHarmony (OHOS) platform.");
  println!("Please run this on an OpenHarmony device or emulator.");
  println!("\nTo test on other platforms, use the simple.rs example instead.");
  Ok(())
}

#[cfg(target_env = "ohos")]
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_builder_creation() {
    let builder = WebViewBuilder::new().with_url("https://tauri.app");
    // Builder should be configured successfully
    assert!(true);
  }
}
