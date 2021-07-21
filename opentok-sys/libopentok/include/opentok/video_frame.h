/** @file video_frame.h
    @brief Video frame

    This file includes the type definition for a video frame along with
    several function declarations useful when handling them.
*/
#ifndef VIDEO_FRAME_H
#define VIDEO_FRAME_H

#ifndef _WIN32
#include <unistd.h>
#endif

#include "base.h"
#include "config.h"

OTC_BEGIN_DECL

/** Max size for an array containing metadata items in a video frame.
 */
#define OTC_VIDEO_FRAME_METADATA_MAX_SIZE 32

/** Video frame type definition.
 */
typedef struct otc_video_frame otc_video_frame;

/** Video frame video format enumeration.

    This enumeration represents several video frame formats.
 */
enum otc_video_frame_format {
  OTC_VIDEO_FRAME_FORMAT_UNKNOWN =
      0, /**< Uknown video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_YUV420P =
      1, /**< YUV420P video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_NV12 = 2, /**< NV12 video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_NV21 = 3, /**< NV21 video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_YUY2 = 4, /**< YUY2 video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_UYVY = 5, /**< UYVY video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_ARGB32 =
      6, /**< ARGB32 video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_BGRA32 =
      7, /**< BGRA32 video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_RGB24 = 8, /**< RGB24 video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_ABGR32 =
      9, /**< ABGR32 video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_MJPEG =
      10, /**< MJPEG video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_RGBA32 =
      11,                     /**< RGBA video frame format enum value. */
  OTC_VIDEO_FRAME_FORMAT_MAX, /**< Maxinum value. */
  // Add any new enum value before this one.
  OTC_VIDEO_FRAME_FORMAT_COMPRESSED =
      255 /**< Compressed video frame format enum value. */
};

/** Video frame video plane enumeration.
 */
enum otc_video_frame_plane {
  OTC_VIDEO_FRAME_PLANE_Y = 0, /**< Video frame plane Y enum value. */
  OTC_VIDEO_FRAME_PLANE_U = 1, /**< Video frame plane U enum value. */
  OTC_VIDEO_FRAME_PLANE_V = 2, /**< Video frame plane V enum value. */

  OTC_VIDEO_FRAME_PLANE_PACKED = 3, /**< Video frame plane packed  enum value. */
  OTC_VIDEO_FRAME_PLANE_UV_INTERLEAVED =
      4, /**< Video frame plane UV interleaved enum value. */
  OTC_VIDEO_FRAME_PLANE_VU_INTERLEAVED =
      5, /**< Video frame plane VU interleaved enum value. */
};

/** Defines a struct containing callback functions for a video frame plane
    memory wrapper struct.
    @see otc_video_frame_new_planar_memory_wrapper
 */
struct otc_video_frame_planar_memory_callbacks {
  /**
      Called when a video frame plane is requested. It is mandatory to implement
      this callback function.

      @param user_data A pointer to the user data bound to this struct.
      @param plane Given plane.
      @return Pointer to video frame plane.
   */
  const uint8_t* (*get_plane)(void* user_data,
                              enum otc_video_frame_plane plane);

  /**
      Called when a video frame plane stride is requested.

      @param user_data A pointer to the user data bound to this struct.
      @param plane Given plane.
      @return Video frame plane stride.
   */
  int (*get_plane_stride)(void* user_data, enum otc_video_frame_plane plane);

  /**
      Called when you can release the video frame data from memory.
      It is not mandatory to implement this callback function.

      @param user_data A pointer to the user data bound to this struct.
   */
  void (*release)(void* user_data);

  /**
      A pointer to any useful user data related to this struct.
  */
  void* user_data;

  /**
      A void pointer to a memory area holding reserved resources used for the
      internal implementation.
   */
  void* reserved;
};

/** Type definition for the video frame memory release callback function.

    This is a function pointer to the callback function that is called
    when the app should release the memory holding a video frame.

    @param buffer A function pointer to the buffer.
    @param arg A pointer to additional arguments.
 */
typedef void (*otc_video_frame_memory_release_callback)(const uint8_t* buffer,
                                                        void* arg);

OTC_DECL(void*)
otc_video_frame_get_native_handle(const otc_video_frame* frame);

/**
    Creates a new video frame with a given format.

    @param format The desired format.
    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param buffer A pointer to a buffer containing video frame data.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new(enum otc_video_frame_format format,
                    int width,
                    int height,
                    const uint8_t* buffer);

/**
    Free resources associated with the frame.

    @param frame A pointer to a video frame instance.
    @return Return value indicating either error or success.
    @see otc_error_code
 */
OTC_DECL(otc_status) otc_video_frame_delete(otc_video_frame* frame);

/**
    Creates a new video frame with I420 format.

    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param y_plane A pointer to a buffer containing Y plane data.
    @param y_stride Y stride.
    @param u_plane A pointer to a buffer containing U plane data.
    @param u_stride U stride.
    @param v_plane A pointer to a buffer containing V plane data.
    @param v_stride V stride.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new_I420(int width,
                         int height,
                         const uint8_t* y_plane,
                         int y_stride,
                         const uint8_t* u_plane,
                         int u_stride,
                         const uint8_t* v_plane,
                         int v_stride);

/**
    Creates a new video frame with MJPEG format.

    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param buffer A pointer to a buffer containing video frame data.
    @param size The video frame size in memory.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new_MJPEG(int width,
                          int height,
                          const uint8_t* buffer,
                          size_t size);

/**
    Creates a new compressed video frame.

    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param buffer A pointer to a buffer containing video frame data.
    @param size The video frame size in memory.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new_compressed(int width,
                               int height,
                               const uint8_t* buffer,
                               size_t size);

/**
    Creates a new video frame with a given format from its planes.

    @param format The desired format.
    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param planes A pointer to a pointer to the buffer containing the video
   frame planes.
    @param strides Strides.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new_from_planes(enum otc_video_frame_format format,
                                int width,
                                int height,
                                const uint8_t** planes,
                                int* strides);

/**
    Creates a new video frame with I420 format from a list of planes.

    @param input_format The format for the video frame passed as input.
    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param planes A pointer to a pointer to the buffer containing the video
   frame planes.
    @param strides Strides.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new_I420_from_planes_with_format(
    enum otc_video_frame_format input_format,
    int width,
    int height,
    const uint8_t** planes,
    int* strides);

/**
    Creates a new video frame from data in a contiguous memmory buffer.

    @param format The format for the video frame passed in.
    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param is_shallow_copyable Whether you can make a shallow copy of the frame.
    @param buffer A pointer to the buffer containing the frame data.
    @param size The size of the frame data.
    @param callback The video frame memory release callback function.
    @param arg A pointer to additional arguments.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new_contiguous_memory_wrapper(
    enum otc_video_frame_format format,
    int width,
    int height,
    otc_bool is_shallow_copyable,
    const uint8_t* buffer,
    size_t size,
    otc_video_frame_memory_release_callback callback,
    void* arg);

/**
    Creates a new video frame from a planar memory wrapper.
    The {@link otc_video_frame_planar_memory_callbacks} struct, passed in,
    defines callback functions that are invoked when video frame and stride
    data is requested.

    @param format The format for the video frame passed in.
    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param is_shallow_copyable Whether you can make a shallow copy of the frame.
    @param callbacks A pointer to the struct containing the video frame callback
   functions.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new_planar_memory_wrapper(
    enum otc_video_frame_format format,
    int width,
    int height,
    otc_bool is_shallow_copyable,
    struct otc_video_frame_planar_memory_callbacks* callbacks);

/**
    Creates a new I420 format video frame from YUV data.

    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param y_plane A pointer the Y plane.
    @param y_stride The Y stride.
    @param u_plane A pointer the U plane.
    @param u_stride The U stride.
    @param v_plane A pointer the V plane.
    @param v_stride The V stride.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new_I420_wrapper(int width,
                                 int height,
                                 const uint8_t* y_plane,
                                 int y_stride,
                                 const uint8_t* u_plane,
                                 int u_stride,
                                 const uint8_t* v_plane,
                                 int v_stride);

/**
    Creates a new video frame with an NV21 wrapper.

    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param y_plane A pointer the Y plane.
    @param y_stride The Y stride.
    @param uv_plane A pointer the UV plane.
    @param uv_stride The UV stride.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new_NV21_wrapper(int width,
                                 int height,
                                 const uint8_t* y_plane,
                                 int y_stride,
                                 const uint8_t* uv_plane,
                                 int uv_stride);

/**
    Creates a new video frame with an NV12 wrapper.

    @param width The width of the video, in pixels.
    @param height The height of the video, in pixels.
    @param y_plane A pointer the Y plane.
    @param y_stride The Y stride.
    @param uv_plane A pointer the UV plane.
    @param uv_stride The UV stride.
    @return A pointer to a video frame instance. This can be null if there is an
   error.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_new_NV12_wrapper(int width,
                                 int height,
                                 const uint8_t* y_plane,
                                 int y_stride,
                                 const uint8_t* uv_plane,
                                 int uv_stride);

/**
    Gets a pointer to the buffer containing video frame data.

    @param frame A pointer to the video frame instance to get the buffer from.
    @return A pointer to the buffer containing video frame data.
 */
OTC_DECL(const uint8_t*)
otc_video_frame_get_buffer(const otc_video_frame* frame);

/**
    Gets the size of the buffer containing video frame data.

    @param frame A pointer to the video frame instance to get the buffer size
   from.
    @return The size of the buffer containing video frame data.
 */
OTC_DECL(size_t) otc_video_frame_get_buffer_size(const otc_video_frame* frame);

/**
    Gets a video frame timestamp.

    @param frame A pointer to the video frame instance.
    @return The video frame timestamp.
 */
OTC_DECL(int64_t) otc_video_frame_get_timestamp(const otc_video_frame* frame);

/**
    Sets the timestamp for a video frame.

    @param frame A pointer to the video frame instance.
    @param timestamp The timestamp value.
 */
OTC_DECL(void)
otc_video_frame_set_timestamp(otc_video_frame* frame, int64_t timestamp);

/**
    Gets the width of a video frame, in pixels.

    @param frame A pointer to the video frame instance.
    @return The width of the given, frame in pixels.
 */
OTC_DECL(int) otc_video_frame_get_width(const otc_video_frame* frame);

/**
    Gets the height of a video frame, in pixels.

    @param frame A pointer to the video frame instance.
    @return The height of the given frame, in pixels.
 */
OTC_DECL(int) otc_video_frame_get_height(const otc_video_frame* frame);

/**
    Gets the number of planes of a video frame. Typically, this returns 3
    for a YUV fame and 1 for a RGBA/BGRA fame.

    @param frame A pointer to the video frame instance.
    @return The number of planes.
 */
OTC_DECL(size_t)
otc_video_frame_get_number_of_planes(const otc_video_frame* frame);

/**
    Gets the video format of a video frame.

    @param frame A pointer to the video frame instance.
    @return The video frame format.
    @see otc_video_frame_format
 */
OTC_DECL(enum otc_video_frame_format)
otc_video_frame_get_format(const otc_video_frame* frame);

/**
    Sets the video format of a video frame.

    @param frame A pointer to the video frame instance.
    @param format  The video frame format.
    @see otc_video_frame_format
 */
OTC_DECL(void)
otc_video_frame_set_format(otc_video_frame* frame,
                           enum otc_video_frame_format format);

/**
    Gets the binary data from one of the planes of a video frame.

    @param frame A pointer to the video frame instance.
    @param plane The video plane to get data from.
    @return A pointer to a buffer containing the plane data. This can be null if
   there is an error.
 */
OTC_DECL(const uint8_t*)
otc_video_frame_get_plane_binary_data(const otc_video_frame* frame,
                                      enum otc_video_frame_plane plane);

/**
    Gets the size of a plane in a video frame.

    @param frame A pointer to the video frame instance.
    @param plane The video plane to get the size from.
    @return Size of the plane.
 */
OTC_DECL(size_t)
otc_video_frame_get_plane_size(const otc_video_frame* frame,
                               enum otc_video_frame_plane plane);

/**
    Gets plane stride from a give plane in a video frame.

    @param frame A pointer to the video frame instance.
    @param plane The video plane to get the stride from.
    @return Stride.
 */
OTC_DECL(int)
otc_video_frame_get_plane_stride(const otc_video_frame* frame,
                                 enum otc_video_frame_plane plane);

/**
    Gets the width of a plane in a video frame.

    @param frame A pointer to the video frame instance.
    @param plane The video plane to get the width from.
    @return With.
 */
OTC_DECL(int)
otc_video_frame_get_plane_width(const otc_video_frame* frame,
                                enum otc_video_frame_plane plane);

/**
    Gets the height of a plane in a video frame.

    @param frame A pointer to the video frame instance.
    @param plane The video plane to get the height from.
    @return Height.
 */
OTC_DECL(int)
otc_video_frame_get_plane_height(const otc_video_frame* frame,
                                 enum otc_video_frame_plane plane);

/**
    Copies a given video frame.

    @param frame A pointer to the video frame instance.
    @return A pointer to the new video frame instance.
 */
OTC_DECL(otc_video_frame*) otc_video_frame_copy(const otc_video_frame* frame);

/**
    Makes a mutable copy of a given video frame.

    @param frame A pointer to the video frame instance.
    @return A pointer to the new video frame instance.
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_mutable_copy(const otc_video_frame* frame);

/**
    Converts a given frame. All the video frames provided by the SDK callback
    functions are bound to this.
    If you plan to save the frame so you use it later, you'll need to copy it.
    In addition to that, if you want to convert a video frame to a different
    format, you'll use this function also
    If the destination format is the same as the source, a shallow copy (n
    buffer copy) will be made if possible.

    @param format The format of the video frame which will be returned.
    This function can convert frames to the following formats:
    `OTC_VIDEO_FRAME_FORMAT_ARGB32`, `OTC_VIDEO_FRAME_FORMAT_BGRA32`,
    `OTC_VIDEO_FRAME_FORMAT_ABGR32`, `OTC_VIDEO_FRAME_FORMAT_RGBA32`
    and `OTC_VIDEO_FRAME_FORMAT_YUV420P`.
    @param frame A pointer to the video frame instance.
    @return A copy of the video frame in the specified format. This can be null
   if there is an error. Keep in mind that you will need to destroy this frame
   later by yourself.
    @see otc_video_frame_format
 */
OTC_DECL(otc_video_frame*)
otc_video_frame_convert(enum otc_video_frame_format format,
                        const otc_video_frame* frame);

/**
    Converts a given frame in place. This changes the data of the input frame.

    @param format The format of the video frame. This function can convert
    frames to the following formats: `OTC_VIDEO_FRAME_FORMAT_ARGB32`,
    `OTC_VIDEO_FRAME_FORMAT_BGRA32`, `OTC_VIDEO_FRAME_FORMAT_ABGR32`,
    and `OTC_VIDEO_FRAME_FORMAT_YUV420P`.

    @param input_frame A pointer to the video frame instance.
    @param planes A pointer to a pointer to the video frame planes.
    @param strides A pointer to a pointer to the video frame stride.
    @return A return value indicating either error or success.
    @see otc_error_code
    @see otc_video_frame_format
 */
OTC_DECL(otc_status)
otc_video_frame_convert_inplace(enum otc_video_frame_format format,
                                uint8_t** planes,
                                const int* strides,
                                const otc_video_frame* input_frame);

/**
    Checks whether a video frame is packed or not.

    @param frame A pointer to the video frame instance.
    @return A Boolean value with the result.
 */
OTC_DECL(otc_bool) otc_video_frame_is_packed(const otc_video_frame* frame);

/**
    Checks whether a video frame is contiguous or not.

    @param frame A pointer to the video frame instance.
    @return A Boolean value with the result.
 */
OTC_DECL(otc_bool) otc_video_frame_is_contiguous(const otc_video_frame* frame);

/**
    Sets the metadata associated with a video frame.

    @param frame A pointer to the video frame instance.
    @param data A pointer to the metadata buffer to be copied into the frame.
    @param size The size of the metadata buffer to be copied.
    @return A return value indicating either error or success.
    @see otc_error_code
 */
OTC_DECL(otc_status)
otc_video_frame_set_metadata(otc_video_frame* frame,
                             const uint8_t* data,
                             size_t size);

/**
    Gets the metadata associated with the video frame.

    @param frame A pointer to the video frame instance.
    @param size The size of the metadata buffer associated with this frame.
    @return A pointer to the internal metadata buffer in the video frame or a
            null pointer if there is no metadata associated with it. This
            pointer will get invalidated after destroying the frame.
 */
OTC_DECL(const uint8_t*)
otc_video_frame_get_metadata(const otc_video_frame* frame, size_t* size);

OTC_END_DECL

#endif  // VIDEO_FRAME_H
