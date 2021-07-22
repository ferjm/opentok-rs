use anyhow::Error;
use derive_more::{Display, Error};
use gst::prelude::*;
use gst_video::VideoFormat;
use opentok::video_frame::FrameFormat;

#[derive(Debug, Display, Error)]
struct MissingElement(#[error(not(source))] &'static str);

pub struct Renderer {
    pipeline: gst::Pipeline,
    src: gst::Element,
}

impl Renderer {
    pub fn new() -> Result<Self, Error> {
        gst::init()?;

        let pipeline = gst::Pipeline::new(None);
        let src =
            gst::ElementFactory::make("appsrc", None).map_err(|_| MissingElement("appsrc"))?;
        let videoconvert = gst::ElementFactory::make("videoconvert", None)
            .map_err(|_| MissingElement("videoconvert"))?;
        let sink = gst::ElementFactory::make("autovideosink", None)
            .map_err(|_| MissingElement("autovideosink"))?;

        pipeline.add_many(&[&src, &videoconvert, &sink])?;
        gst::Element::link_many(&[&src, &videoconvert, &sink])?;

        pipeline.set_state(gst::State::Playing)?;

        Ok(Self { pipeline, src })
    }

    pub fn push_buffer(
        &self,
        data: &[u8],
        format: FrameFormat,
        width: u32,
        height: u32,
        offset: &[usize],
        stride: &[i32],
    ) {
        let mut buffer = gst::Buffer::with_size(data.len()).unwrap();
        let gst_format = Renderer::gst_from_otc_format(format);
        // TODO: Set PTS on buffer
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.copy_from_slice(0, data).expect("copying failed");
            let flags = gst_video::VideoFrameFlags::empty();
            gst_video::VideoMeta::add_full(
                buffer, flags, gst_format, width, height, offset, stride,
            )
            .unwrap();
        }
        let appsrc = self
            .src
            .clone()
            .dynamic_cast::<gst_app::AppSrc>()
            .expect("Source element is expected to be an appsrc!");
        let caps = gst::Caps::builder("video/x-raw")
            .field("width", width as i32)
            .field("height", height as i32)
            .field("framerate", gst::Fraction::new(1, 1))
            .field("format", format!("{}", gst_format))
            .build();

        let sample = gst::Sample::builder().caps(&caps).buffer(&buffer).build();
        let _ = appsrc.push_sample(&sample);
    }

    fn gst_from_otc_format(format: FrameFormat) -> VideoFormat {
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
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }
}
