use anyhow::{Result as AnyResult, anyhow};
use std::process::Command;

/// Converts a `.webm` file into `.mp4` using FFmpeg.
pub fn convert_webm_to_mp4(file_path: &str) -> AnyResult<String> {
    let output_path = format!("{}.mp4", file_path);
    let output = Command::new("ffmpeg")
        .args(["-i", file_path, &output_path])
        .output()?;
    if !output.status.success() {
        return Err(anyhow!("FFmpeg conversion failed: {:?}", output));
    }
    Ok(output_path)
}
