use ada_tool_api::{router::ToolHandler, ToolRequest, ToolError, ToolErrorCode};
use async_trait::async_trait;
use scrap::{Display, Capturer};
use std::io::ErrorKind::WouldBlock;
use std::{thread, time::Duration};
use image::{ImageBuffer, Rgba, DynamicImage};
use base64::{Engine as _, engine::general_purpose};

pub struct ScreenCaptureHandler;

#[async_trait]
impl ToolHandler for ScreenCaptureHandler {
    async fn handle(&self, _req: &ToolRequest) -> Result<serde_json::Value, ToolError> {
        let display = Display::primary().map_err(|e| ToolError {
            code: ToolErrorCode::ExecFailed,
            message: format!("Failed to find primary display: {}", e),
            detail: None,
        })?;
        let mut capturer = Capturer::new(display).map_err(|e| ToolError {
            code: ToolErrorCode::ExecFailed,
            message: format!("Failed to create capturer: {}", e),
            detail: None,
        })?;
        let (w, h) = (capturer.width(), capturer.height());

        loop {
            match capturer.frame() {
                Ok(frame) => {
                    let mut buffer = ImageBuffer::<Rgba<u8>, _>::new(w as u32, h as u32);
                    for (i, pixel) in frame.chunks(4).enumerate() {
                        let x = (i as u32) % (w as u32);
                        let y = (i as u32) / (w as u32);
                        if y < buffer.height() {
                            buffer.put_pixel(x, y, Rgba([pixel[2], pixel[1], pixel[0], pixel[3]]));
                        }
                    }

                    let dynamic_image = DynamicImage::ImageRgba8(buffer);
                    let mut bytes: Vec<u8> = Vec::new();
                    dynamic_image.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).map_err(|e| ToolError {
                        code: ToolErrorCode::ExecFailed,
                        message: format!("Encoding failed: {}", e),
                        detail: None,
                    })?;
                    
                    let base64_img = general_purpose::STANDARD.encode(bytes);
                    return Ok(serde_json::json!({
                        "width": w,
                        "height": h,
                        "base64_png": format!("data:image/png;base64,{}", base64_img)
                    }));
                }
                Err(e) if e.kind() == WouldBlock => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
                Err(e) => return Err(ToolError {
                    code: ToolErrorCode::ExecFailed,
                    message: format!("Capture failed: {}", e),
                    detail: None,
                }),
            }
        }
    }
}
