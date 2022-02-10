use crate::video_frame::FrameFormat;
use derive_more::{Display, Error};
use gst_video::VideoFormat;

#[derive(Debug, Display, Error)]
pub struct MissingElement(#[error(not(source))] pub &'static str);

#[derive(Clone, Debug, Default)]
pub struct Credentials {
    pub api_key: String,
    pub session_id: String,
    pub token: String,
}

#[allow(dead_code)]
pub fn gst_from_otc_format(format: FrameFormat) -> VideoFormat {
    match format {
        FrameFormat::Abgr32 => VideoFormat::Abgr,
        FrameFormat::Argb32 => VideoFormat::Argb,
        FrameFormat::Bgra32 => VideoFormat::Bgra,
        FrameFormat::Nv12 => VideoFormat::Nv12,
        FrameFormat::Nv21 => VideoFormat::Nv21,
        FrameFormat::Rgba32 => VideoFormat::Rgba,
        FrameFormat::Uyvy => VideoFormat::Uyvy,
        FrameFormat::Yuv420P => VideoFormat::I420,
        FrameFormat::Yuy2 => VideoFormat::Yuy2,
        _ => unimplemented!(),
    }
}
