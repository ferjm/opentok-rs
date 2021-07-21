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
        let rawvideoparse = gst::ElementFactory::make("rawvideoparse", None)
            .map_err(|_| MissingElement("rawvideoparse"))?;
        let queue =
            gst::ElementFactory::make("queue", None).map_err(|_| MissingElement("queue"))?;
        let videoconvert = gst::ElementFactory::make("videoconvert", None)
            .map_err(|_| MissingElement("videoconvert"))?;
        let videoscale = gst::ElementFactory::make("videoscale", None)
            .map_err(|_| MissingElement("videoscale"))?;
        let sink = gst::ElementFactory::make("autovideosink", None)
            .map_err(|_| MissingElement("autovideosink"))?;

        pipeline.add_many(&[
            &src,
            &rawvideoparse,
            &queue,
            &videoconvert,
            &videoscale,
            &sink,
        ])?;
        gst::Element::link_many(&[
            &src,
            &rawvideoparse,
            &queue,
            &videoconvert,
            &videoscale,
            &sink,
        ])?;

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
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.copy_from_slice(0, data).expect("copying failed");
            gst_video::VideoMeta::add_full(
                buffer,
                gst_video::VideoFrameFlags::empty(),
                Renderer::gst_from_otc_format(format),
                width,
                height,
                offset,
                stride,
            )
            .unwrap();
        }
        let appsrc = self
            .src
            .clone()
            .dynamic_cast::<gst_app::AppSrc>()
            .expect("Source element is expected to be an appsrc!");
        let _ = appsrc.push_buffer(buffer);
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
