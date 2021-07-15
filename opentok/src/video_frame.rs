use std::convert::TryInto;
use std::ops::Deref;
use std::slice;

/// Video frame format enumeration.
#[derive(Clone, Copy)]
pub enum FrameFormat {
    Abgr32,
    Argb32,
    Bgra32,
    Compressed,
    Max,
    Mjpeg,
    Nv12,
    Nv21,
    Rgba32,
    Rgb24,
    Uyvy,
    Yuv420P,
    Yuy2,
    __Unknown,
}

impl From<ffi::otc_video_frame_format> for FrameFormat {
    fn from(type_: ffi::otc_video_frame_format) -> FrameFormat {
        match type_ {
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_ABGR32 => FrameFormat::Abgr32,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_ARGB32 => FrameFormat::Argb32,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_BGRA32 => FrameFormat::Bgra32,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_COMPRESSED => {
                FrameFormat::Compressed
            }
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_MAX => FrameFormat::Max,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_MJPEG => FrameFormat::Mjpeg,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_NV12 => FrameFormat::Nv12,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_NV21 => FrameFormat::Nv21,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_RGBA32 => FrameFormat::Rgba32,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_RGB24 => FrameFormat::Rgb24,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_UYVY => FrameFormat::Uyvy,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_YUV420P => FrameFormat::Yuv420P,
            ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_YUY2 => FrameFormat::Yuy2,
            _ => FrameFormat::__Unknown,
        }
    }
}

impl From<FrameFormat> for ffi::otc_video_frame_format {
    fn from(type_: FrameFormat) -> ffi::otc_video_frame_format {
        match type_ {
            FrameFormat::Abgr32 => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_ABGR32,
            FrameFormat::Argb32 => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_ARGB32,
            FrameFormat::Bgra32 => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_BGRA32,
            FrameFormat::Compressed => {
                ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_COMPRESSED
            }
            FrameFormat::Max => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_MAX,
            FrameFormat::Mjpeg => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_MJPEG,
            FrameFormat::Nv12 => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_NV12,
            FrameFormat::Nv21 => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_NV21,
            FrameFormat::Rgba32 => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_RGBA32,
            FrameFormat::Rgb24 => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_RGB24,
            FrameFormat::Uyvy => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_UYVY,
            FrameFormat::Yuv420P => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_YUV420P,
            FrameFormat::Yuy2 => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_YUY2,
            FrameFormat::__Unknown => ffi::otc_video_frame_format_OTC_VIDEO_FRAME_FORMAT_UNKNOWN,
        }
    }
}

/// Video frame video plane enumeration.
enum FramePlane {
    Packed,
    U,
    UvInterleaved,
    V,
    VuInterleaved,
    Y,
    __Unknown,
}

impl From<ffi::otc_video_frame_plane> for FramePlane {
    fn from(type_: ffi::otc_video_frame_plane) -> FramePlane {
        match type_ {
            ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_PACKED => FramePlane::Packed,
            ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_U => FramePlane::U,
            ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_UV_INTERLEAVED => {
                FramePlane::UvInterleaved
            }
            ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_V => FramePlane::V,
            ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_VU_INTERLEAVED => {
                FramePlane::VuInterleaved
            }
            ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_Y => FramePlane::Y,
            _ => FramePlane::__Unknown,
        }
    }
}

pub struct VideoFrame {
    ptr: *const ffi::otc_video_frame,
}

impl VideoFrame {
    fn new(format: FrameFormat, width: i32, height: i32, buffer: Vec<u8>) -> Self {
        Self {
            ptr: unsafe { ffi::otc_video_frame_new(format.into(), width, height, buffer.as_ptr()) },
        }
    }

    fn new_mjpeg(width: i32, height: i32, buffer: Vec<u8>, size: usize) -> Self {
        Self {
            ptr: unsafe {
                ffi::otc_video_frame_new_MJPEG(
                    width,
                    height,
                    buffer.as_ptr(),
                    size.try_into().expect("usize to size_t cast"),
                )
            },
        }
    }

    fn new_compressed(width: i32, height: i32, buffer: Vec<u8>, size: usize) -> Self {
        Self {
            ptr: unsafe {
                ffi::otc_video_frame_new_compressed(
                    width,
                    height,
                    buffer.as_ptr(),
                    size.try_into().expect("usize to size_t cast"),
                )
            },
        }
    }

    // FIXME: implement more constructors as needed.

    fn get_buffer(&self) -> &[u8] {
        let data = unsafe { ffi::otc_video_frame_get_buffer(self.ptr) };
        let size = unsafe { ffi::otc_video_frame_get_buffer_size(self.ptr) };
        unsafe { slice::from_raw_parts(data, size.try_into().expect("u64 to usize cast")) }
    }

    fn get_timestamp(&self) -> i64 {
        unsafe { ffi::otc_video_frame_get_timestamp(self.ptr) }
    }

    fn set_timestamp(&mut self, timestamp: i64) {
        unsafe {
            ffi::otc_video_frame_set_timestamp(self.ptr as *mut ffi::otc_video_frame, timestamp)
        }
    }

    fn get_width(&self) -> i32 {
        unsafe { ffi::otc_video_frame_get_width(self.ptr) }
    }

    fn get_height(&self) -> i32 {
        unsafe { ffi::otc_video_frame_get_height(self.ptr) }
    }

    fn get_number_of_planes(&self) -> usize {
        unsafe {
            ffi::otc_video_frame_get_number_of_planes(self.ptr)
                .try_into()
                .expect("u64 to usize cast")
        }
    }

    fn get_format(&self) -> FrameFormat {
        unsafe { ffi::otc_video_frame_get_format(self.ptr) }.into()
    }

    fn set_format(&mut self, format: FrameFormat) {
        unsafe {
            ffi::otc_video_frame_set_format(self.ptr as *mut ffi::otc_video_frame, format.into())
        }
    }
}

impl Drop for VideoFrame {
    fn drop(&mut self) {
        unsafe {
            ffi::otc_video_frame_delete(self.ptr as *mut ffi::otc_video_frame);
        }
    }
}

impl Deref for VideoFrame {
    type Target = *const ffi::otc_video_frame;

    fn deref(&self) -> &*const ffi::otc_video_frame {
        &self.ptr
    }
}
