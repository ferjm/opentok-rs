/** @file stream.h
    @brief Audio/Video stream.

    This file includes the type definition for an audio-video stream along with
    several function declarations useful when handling it.
*/
#ifndef STREAM_H
#define STREAM_H

#include "config.h"
#include "connection.h"

OTC_BEGIN_DECL

/** Type definition for a struct representing an audio-video stream in an
    OpenTok session.
 */
typedef struct otc_stream otc_stream;

/** Stream video type enumeration.

    This enumeration represents the different stream video types supported.
 */
enum otc_stream_video_type {
  OTC_STREAM_VIDEO_TYPE_CAMERA = 1, /**< Camera video stream. */
  OTC_STREAM_VIDEO_TYPE_SCREEN =
      2, /**< Screen video stream. This is used for screen-sharing. */
  OTC_STREAM_VIDEO_TYPE_CUSTOM = 3 /**< Custom video stream. */
};

/**
    Gets the unique identifier for this stream.

    @param stream The stream.

    @return The unique identifier for this stream.
 */
OTC_DECL(const char*) otc_stream_get_id(const otc_stream* stream);

/**
    Gets the name of the stream. The publisher of the stream can set this
    name to identify the stream.

    @param stream The stream being queried.
    @return The stream name.
 */
OTC_DECL(const char*) otc_stream_get_name(const otc_stream* stream);

/**
    Checks whether this stream contains a video track or not.

    @param stream The stream being queried.
    @return Return value indicating whether this stream contains a video track
   or not.
 */
OTC_DECL(otc_bool) otc_stream_has_video_track(const otc_stream* stream);

/**
    Checks whether this stream is currently publishing video or not.

    @param stream The stream being queried.
    @return Return value indicating whether this stream is publishing video or
   not.
 */
OTC_DECL(otc_bool) otc_stream_has_video(const otc_stream* stream);

/**
    Checks whether this stream contains an audio track or not.

    @param stream The stream being queried.
    @return Return value indicating whether this stream contains an audio track
   or not.
 */
OTC_DECL(otc_bool) otc_stream_has_audio_track(const otc_stream* stream);

/**
    Checks whether this stream is currently publishing audio or not.

    @param stream The stream being queried.
    @return Return value indicating whether this stream is publishing audio or
   not.
 */
OTC_DECL(otc_bool) otc_stream_has_audio(const otc_stream* stream);

/**
    Makes a copy of a stream.

    @param stream The stream to be copied.
    @return A copy of the stream. This can be null if there is an error.
 */
OTC_DECL(otc_stream*) otc_stream_copy(const otc_stream* stream);

/**
    Releases resources associated with the stream.

    @param stream The stream being queried.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status) otc_stream_delete(otc_stream* stream);

/**
    Returns the width, in pixels, of the video stream.

    @param stream The stream being queried.
    @return The width, in pixels, of the video stream.
 */
OTC_DECL(int) otc_stream_get_video_width(const otc_stream* stream);

/**
    Returns the height, in pixels, of the video stream.

    @param stream The stream being queried.
    @return The height, in pixels, of the video stream.
 */
OTC_DECL(int) otc_stream_get_video_height(const otc_stream* stream);

/**
    Gets the creation time of the stream.

    @param stream The stream being queried.
    @return The timestamp for the creation time of the stream.
 */
OTC_DECL(int64_t) otc_stream_get_creation_time(const otc_stream* stream);

/**
    Returns the video type of the stream.

    @param stream The stream being queried.
    @return The video type of the stream.
    @see otc_stream_video_type
 */
OTC_DECL(enum otc_stream_video_type)
otc_stream_get_video_type(const otc_stream* stream);

/**
    Get the connection associated with the client publishing the stream.

    @param stream The stream being queried.
    @return The connection associated with the client publishing the stream.
 */
OTC_DECL(const otc_connection*)
otc_stream_get_connection(const otc_stream* stream);

OTC_END_DECL

#endif  // STREAM_H
