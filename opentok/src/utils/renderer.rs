use crate::utils::common::{gst_from_otc_format, MissingElement};

use crate::audio_device::AudioSample;
use crate::video_frame::FrameFormat;
use anyhow::Error;
use byte_slice_cast::*;
use gst::prelude::*;

pub struct Renderer {
    pipeline: gst::Pipeline,
    video_src: gst::Element,
    audio_src: gst::Element,
}

impl Renderer {
    pub fn new() -> Result<Self, Error> {
        gst::init()?;

        let pipeline = gst::Pipeline::new(None);
        let video_src =
            gst::ElementFactory::make("appsrc", None).map_err(|_| MissingElement("appsrc"))?;
        let videoconvert = gst::ElementFactory::make("videoconvert", None)
            .map_err(|_| MissingElement("videoconvert"))?;
        let sink = gst::ElementFactory::make("autovideosink", None)
            .map_err(|_| MissingElement("autovideosink"))?;

        pipeline.add_many(&[&video_src, &videoconvert, &sink])?;
        gst::Element::link_many(&[&video_src, &videoconvert, &sink])?;

        let audio_src =
            gst::ElementFactory::make("appsrc", None).map_err(|_| MissingElement("appsrc"))?;
        let audioresample = gst::ElementFactory::make("audioresample", None)
            .map_err(|_| MissingElement("audioresample"))?;
        let audioconvert = gst::ElementFactory::make("audioconvert", None)
            .map_err(|_| MissingElement("audioconvert"))?;
        let sink = gst::ElementFactory::make("autoaudiosink", None)
            .map_err(|_| MissingElement("autoaudiosink"))?;

        pipeline.add_many(&[&audio_src, &audioresample, &audioconvert, &sink])?;
        gst::Element::link_many(&[&audio_src, &audioresample, &audioconvert, &sink])?;

        pipeline.set_state(gst::State::Playing)?;

        if cfg!(debug_assertions) {
            let bin = pipeline.clone().upcast::<gst::Bin>();
            gst::debug_bin_to_dot_file_with_ts(
                &bin,
                gst::DebugGraphDetails::all(),
                "RendererPipeline",
            );
        }

        Ok(Self {
            pipeline,
            video_src,
            audio_src,
        })
    }

    pub fn push_video_buffer(
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
            .video_src
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

    pub fn render_audio_sample(&self, sample: AudioSample) {
        struct CastVec(Vec<i16>);
        impl AsRef<[u8]> for CastVec {
            fn as_ref(&self) -> &[u8] {
                self.0.as_byte_slice()
            }
        }
        let buffer = gst::Buffer::from_slice(CastVec(sample.data.0));

        // TODO: Set PTS on buffer
        let gst_format = "S16LE";
        let appsrc = self
            .audio_src
            .clone()
            .dynamic_cast::<gst_app::AppSrc>()
            .expect("Source element is expected to be an appsrc!");
        let caps = gst::Caps::builder("audio/x-raw")
            .field("format", gst_format.to_string())
            .field("layout", "interleaved")
            .field("rate", sample.sampling_rate)
            .field("channels", sample.number_of_channels)
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
