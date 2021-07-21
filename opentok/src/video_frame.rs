use crate::{OtcError, OtcResult};

use std::convert::TryInto;
use std::ops::Deref;
use std::slice;

/// Video frame format enumeration.
#[derive(Clone, Copy, Debug)]
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
pub enum FramePlane {
    Packed,
    U,
    UvInterleaved,
    V,
    VuInterleaved,
    Y,
    __Unknown,
}

impl From<ffi::otc_video_frame_plane> for FramePlane {
    fn from(plane: ffi::otc_video_frame_plane) -> FramePlane {
        match plane {
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

impl From<FramePlane> for ffi::otc_video_frame_plane {
    fn from(plane: FramePlane) -> ffi::otc_video_frame_plane {
        match plane {
            FramePlane::Packed => ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_PACKED,
            FramePlane::U => ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_U,
            FramePlane::UvInterleaved => {
                ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_UV_INTERLEAVED
            }
            FramePlane::V => ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_V,
            FramePlane::VuInterleaved => {
                ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_VU_INTERLEAVED
            }
            FramePlane::Y => ffi::otc_video_frame_plane_OTC_VIDEO_FRAME_PLANE_Y,
            FramePlane::__Unknown => u32::MAX,
        }
    }
}

pub struct VideoFrame {
    ptr: *const ffi::otc_video_frame,
}

impl VideoFrame {
    pub fn new(format: FrameFormat, width: i32, height: i32, buffer: Vec<u8>) -> Self {
        Self {
            ptr: unsafe { ffi::otc_video_frame_new(format.into(), width, height, buffer.as_ptr()) },
        }
    }

    pub fn new_mjpeg(width: i32, height: i32, buffer: Vec<u8>, size: usize) -> Self {
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

    pub fn new_compressed(width: i32, height: i32, buffer: Vec<u8>, size: usize) -> Self {
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

    pub fn get_buffer(&self) -> Result<&[u8], OtcError> {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        let data = unsafe { ffi::otc_video_frame_get_buffer(self.ptr) };
        let size = unsafe { ffi::otc_video_frame_get_buffer_size(self.ptr) };
        Ok(unsafe { slice::from_raw_parts(data, size.try_into().expect("u64 to usize cast")) })
    }

    pub fn get_timestamp(&self) -> Result<i64, OtcError> {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        Ok(unsafe { ffi::otc_video_frame_get_timestamp(self.ptr) })
    }

    pub fn set_timestamp(&mut self, timestamp: i64) -> OtcResult {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        unsafe {
            ffi::otc_video_frame_set_timestamp(self.ptr as *mut ffi::otc_video_frame, timestamp);
        }
        Ok(())
    }

    pub fn get_width(&self) -> Result<i32, OtcError> {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        Ok(unsafe { ffi::otc_video_frame_get_width(self.ptr) })
    }

    pub fn get_height(&self) -> Result<i32, OtcError> {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        Ok(unsafe { ffi::otc_video_frame_get_height(self.ptr) })
    }

    pub fn get_number_of_planes(&self) -> Result<usize, OtcError> {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        Ok(unsafe {
            ffi::otc_video_frame_get_number_of_planes(self.ptr)
                .try_into()
                .expect("u64 to usize cast")
        })
    }

    pub fn get_format(&self) -> Result<FrameFormat, OtcError> {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        Ok(unsafe { ffi::otc_video_frame_get_format(self.ptr) }.into())
    }

    pub fn set_format(&mut self, format: FrameFormat) -> OtcResult {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        unsafe {
            ffi::otc_video_frame_set_format(self.ptr as *mut ffi::otc_video_frame, format.into());
        }
        Ok(())
    }

    pub fn convert(&mut self, format: FrameFormat) -> Result<VideoFrame, OtcError> {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        Ok(((unsafe {
            ffi::otc_video_frame_convert(format.into(), self.ptr as *mut ffi::otc_video_frame)
        }) as *const ffi::otc_video_frame)
            .into())
    }

    pub fn get_plane_size(&self, plane: FramePlane) -> Result<usize, OtcError> {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        Ok(unsafe {
            ffi::otc_video_frame_get_plane_size(self.ptr as *mut ffi::otc_video_frame, plane.into())
                as usize
        })
    }

    pub fn get_plane_stride(&self, plane: FramePlane) -> Result<i32, OtcError> {
        if self.ptr.is_null() {
            return Err(OtcError::NullError);
        }
        Ok(unsafe {
            ffi::otc_video_frame_get_plane_stride(
                self.ptr as *mut ffi::otc_video_frame,
                plane.into(),
            )
        })
    }
}

impl Drop for VideoFrame {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            panic!("Attempting to drop an invalid VideoFrame pointer");
        }

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

impl From<*const ffi::otc_video_frame> for VideoFrame {
    fn from(ptr: *const ffi::otc_video_frame) -> Self {
        Self { ptr }
    }
}
