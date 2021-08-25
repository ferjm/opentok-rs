use anyhow::Error;
use byte_slice_cast::*;
use gst::prelude::*;
use opentok::video_frame::FrameFormat;

#[path = "./common.rs"]
mod common;

use common::{gst_from_otc_format, MissingElement};

pub struct CapturerBuffer(gst::buffer::MappedBuffer<gst::buffer::Readable>);

impl AsRef<[u8]> for CapturerBuffer {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref().as_slice_of::<u8>().unwrap()
    }
}

pub struct Capturer {
    pipeline: gst::Pipeline,
    sink: gst::Element,
}

impl Capturer {
    pub fn new(format: FrameFormat) -> Result<Self, Error> {
        gst::init()?;

        let format = gst_from_otc_format(format);
        let caps = gst::Caps::new_simple("video/x-raw", &[("format", &format.to_string())]);

        let pipeline = gst::Pipeline::new(None);
        let src = gst::ElementFactory::make("videotestsrc", None)
            .map_err(|_| MissingElement("videotestsrc"))?;
        let capsfilter = gst::ElementFactory::make("capsfilter", None)
            .map_err(|_| MissingElement("capsfilter"))?;
        capsfilter.set_property("caps", &caps).unwrap();
        let sink =
            gst::ElementFactory::make("appsink", None).map_err(|_| MissingElement("appsink"))?;

        pipeline.add_many(&[&src, &capsfilter, &sink])?;
        gst::Element::link_many(&[&src, &capsfilter, &sink])?;

        pipeline.set_state(gst::State::Playing)?;

        let bin_ref = pipeline.upcast_ref::<gst::Bin>();
        gst::debug_bin_to_dot_file_with_ts(
            bin_ref,
            gst::DebugGraphDetails::all(),
            "CapturerPipeline",
        );

        Ok(Self { pipeline, sink })
    }

    pub fn pull_buffer(&self) -> Result<Box<dyn AsRef<[u8]>>, Error> {
        let appsink = self.sink.downcast_ref::<gst_app::AppSink>().unwrap();
        let sample = appsink.pull_sample().unwrap();
        let buffer = sample.buffer_owned().unwrap();
        let map = buffer.into_mapped_buffer_readable().unwrap();
        Ok(Box::new(CapturerBuffer(map)))
    }
}

impl Drop for Capturer {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }
}
