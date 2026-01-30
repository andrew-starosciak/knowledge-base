use anyhow::Result;
use super::parser;
use crate::storage::models::{Video, Transcript};

pub struct Fetcher {
    yt_dlp_path: String,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            yt_dlp_path: "yt-dlp".to_string(),
        }
    }

    pub fn with_path(path: &str) -> Self {
        Self {
            yt_dlp_path: path.to_string(),
        }
    }

    pub fn fetch(&self, url: &str) -> Result<(Video, Option<Transcript>)> {
        let video = self.fetch_metadata(url)?;
        let transcript = self.fetch_transcript(url, &video.id)?;
        Ok((video, transcript))
    }

    fn fetch_metadata(&self, url: &str) -> Result<Video> {
        let output = std::process::Command::new(&self.yt_dlp_path)
            .args(["--dump-json", "--no-download", url])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("yt-dlp failed: {}", stderr);
        }

        let json = String::from_utf8(output.stdout)?;
        parser::parse_video_metadata(&json)
    }

    fn fetch_transcript(&self, url: &str, video_id: &str) -> Result<Option<Transcript>> {
        let temp_dir = std::env::temp_dir();
        let output_template = temp_dir.join(format!("{}.%(ext)s", video_id));

        let output = std::process::Command::new(&self.yt_dlp_path)
            .args([
                "--write-subs",
                "--write-auto-subs",
                "--sub-langs", "en",
                "--sub-format", "json3",
                "--skip-download",
                "-o", output_template.to_str().unwrap(),
                url,
            ])
            .output()?;

        if !output.status.success() {
            return Ok(None);
        }

        // Look for the transcript file
        let patterns = [
            temp_dir.join(format!("{}.en.json3", video_id)),
            temp_dir.join(format!("{}.en-orig.json3", video_id)),
        ];

        for pattern in patterns {
            if pattern.exists() {
                let content = std::fs::read_to_string(&pattern)?;
                let _ = std::fs::remove_file(&pattern);
                return Ok(Some(parser::parse_transcript(&content, video_id)?));
            }
        }

        Ok(None)
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self::new()
    }
}
