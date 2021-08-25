use anyhow::Error;
use gst::prelude::*;
use opentok::video_frame::FrameFormat;

#[path = "./common.rs"]
mod common;

use common::{gst_from_otc_format, MissingElement};

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
        let gst_format = gst_from_otc_format(format);
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
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }
}
