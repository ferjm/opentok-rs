use crate::utils::common::{gst_from_otc_format, MissingElement};

use crate::audio_device::{AudioDeviceSettings, AudioSampleData};
use crate::video_capturer::VideoCapturerSettings;
use anyhow::Error;
use byte_slice_cast::*;
use gst::prelude::*;

pub struct CapturerBuffer(gst::buffer::MappedBuffer<gst::buffer::Readable>);

impl AsRef<[u8]> for CapturerBuffer {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref().as_slice_of::<u8>().unwrap()
    }
}

impl AsRef<[i16]> for CapturerBuffer {
    fn as_ref(&self) -> &[i16] {
        self.0.as_ref().as_slice_of::<i16>().unwrap()
    }
}

pub struct Capturer {
    pipeline: gst::Pipeline,
    sink: gst::Element,
}

impl Capturer {
    pub fn new(settings: &VideoCapturerSettings) -> Result<Self, Error> {
        gst::init()?;

        let format = gst_from_otc_format(settings.format);
        let caps = gst::Caps::builder_full()
            .structure(
                gst::Structure::builder("video/x-raw")
                    .field("format", &format.to_string())
                    .field("width", &settings.width)
                    .field("height", &settings.height)
                    .field("framerate", &gst::Fraction::new(settings.fps, 1))
                    .build(),
            )
            .build();

        let pipeline = gst::Pipeline::new(None);
        let src = gst::ElementFactory::make("videotestsrc")
            .build()
            .map_err(|_| MissingElement("videotestsrc"))?;
        let capsfilter = gst::ElementFactory::make("capsfilter")
            .build()
            .map_err(|_| MissingElement("capsfilter"))?;
        capsfilter.set_property("caps", &caps);
        let sink =
            gst::ElementFactory::make("appsink").build().map_err(|_| MissingElement("appsink"))?;

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

pub struct AudioCapturer {
    pipeline: gst::Pipeline,
    sink: gst::Element,
    samples_per_buffer: usize,
}

impl AudioCapturer {
    pub fn new(settings: &AudioDeviceSettings) -> Result<Self, Error> {
        gst::init()?;

        let caps = gst::Caps::builder_full()
            .structure(
                gst::Structure::builder("audio/x-raw")
                    .field("format", &"S16LE")
                    .field("layout", &"interleaved")
                    .field("rate", &settings.sampling_rate)
                    .field("channels", &settings.number_of_channels)
                    .build(),
            )
            .build();

        let pipeline = gst::Pipeline::new(None);
        let src = gst::ElementFactory::make("audiotestsrc").build()
            .map_err(|_| MissingElement("audiotestsrc"))?;
        let capsfilter = gst::ElementFactory::make("capsfilter").build()
            .map_err(|_| MissingElement("capsfilter"))?;
        capsfilter.set_property("caps", &caps);
        let sink =
            gst::ElementFactory::make("appsink").build().map_err(|_| MissingElement("appsink"))?;

        let samples_per_buffer = (settings.sampling_rate / 100) * settings.number_of_channels;
        src.set_property("samplesperbuffer", samples_per_buffer);
        pipeline.add_many(&[&src, &capsfilter, &sink])?;
        gst::Element::link_many(&[&src, &capsfilter, &sink])?;

        pipeline.set_state(gst::State::Playing)?;

        let bin_ref = pipeline.upcast_ref::<gst::Bin>();
        gst::debug_bin_to_dot_file_with_ts(
            bin_ref,
            gst::DebugGraphDetails::all(),
            "AudioCapturerPipeline",
        );

        Ok(Self {
            pipeline,
            sink,
            samples_per_buffer: samples_per_buffer as usize,
        })
    }

    pub fn pull_buffer(&self) -> Option<AudioSampleData> {
        // NOTE: Ideally we should use size here, with an adapter. For testing
        // purpose it was simpler to set the samplesperbuffer property to match
        // our settings.
        let size = 441;
        let appsink = self.sink.downcast_ref::<gst_app::AppSink>().unwrap();
        if let Ok(sample) = appsink.pull_sample() {
            let buffer = sample.buffer_owned().unwrap();
            let map = buffer.into_mapped_buffer_readable().unwrap();
            let m = CapturerBuffer(map);
            let mut d: Vec<i16> = vec![0; self.samples_per_buffer];
            d[..size].clone_from_slice(m.as_ref());
            Some(AudioSampleData(d))
        } else {
            None
        }
    }
}

impl Drop for AudioCapturer {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }
}
