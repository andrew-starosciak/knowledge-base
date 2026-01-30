use anyhow::Result;
use serde::Deserialize;
use crate::storage::models::{Video, Transcript, TranscriptSegment};
use chrono::{NaiveDate, Utc};

#[derive(Deserialize)]
struct YtDlpMetadata {
    id: String,
    title: String,
    channel: Option<String>,
    uploader: Option<String>,
    upload_date: Option<String>,
    description: Option<String>,
    webpage_url: Option<String>,
    original_url: Option<String>,
}

#[derive(Deserialize)]
struct Json3Transcript {
    events: Option<Vec<Json3Event>>,
}

#[derive(Deserialize)]
struct Json3Event {
    #[serde(rename = "tStartMs")]
    t_start_ms: Option<i64>,
    #[serde(rename = "dDurationMs")]
    d_duration_ms: Option<i64>,
    segs: Option<Vec<Json3Seg>>,
}

#[derive(Deserialize)]
struct Json3Seg {
    utf8: Option<String>,
}

pub fn parse_video_metadata(json: &str) -> Result<Video> {
    let meta: YtDlpMetadata = serde_json::from_str(json)?;

    let upload_date = meta.upload_date.and_then(|d| {
        NaiveDate::parse_from_str(&d, "%Y%m%d").ok()
    });

    let url = meta.webpage_url
        .or(meta.original_url)
        .unwrap_or_else(|| format!("https://www.youtube.com/watch?v={}", meta.id));

    Ok(Video {
        id: meta.id,
        url,
        title: meta.title,
        channel: meta.channel.or(meta.uploader),
        upload_date,
        description: meta.description,
        added_at: Utc::now(),
    })
}

pub fn parse_transcript(json: &str, video_id: &str) -> Result<Transcript> {
    let data: Json3Transcript = serde_json::from_str(json)?;

    let mut segments = Vec::new();
    let mut full_text_parts = Vec::new();

    if let Some(events) = data.events {
        for event in events {
            let start_ms = event.t_start_ms.unwrap_or(0);
            let duration_ms = event.d_duration_ms.unwrap_or(0);

            if let Some(segs) = event.segs {
                let text: String = segs
                    .into_iter()
                    .filter_map(|s| s.utf8)
                    .collect::<Vec<_>>()
                    .join("");

                let text = text.trim().to_string();
                if !text.is_empty() && text != "\n" {
                    full_text_parts.push(text.clone());
                    segments.push(TranscriptSegment {
                        start_time: start_ms as f64 / 1000.0,
                        duration: duration_ms as f64 / 1000.0,
                        text,
                    });
                }
            }
        }
    }

    Ok(Transcript {
        video_id: video_id.to_string(),
        language: "en".to_string(),
        segments,
        full_text: full_text_parts.join(" "),
    })
}
