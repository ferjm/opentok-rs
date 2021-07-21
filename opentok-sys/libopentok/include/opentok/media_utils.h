/** @file media_utils.h
    @brief Media utility functions.

    This file includes an
    otc_media_utils_get_supported_codecs(otc_media_utils_codecs**
   supported_codecs) function, which you can use to find the video codecs
   supported on the device.
*/
#ifndef MEDIA_UTILS_H
#define MEDIA_UTILS_H

#include "config.h"

OTC_BEGIN_DECL

/** Video codec type enum.
 */
enum otc_video_codec_type {
  OTC_VIDEO_CODEC_VP8 = 1, /**< Video codec type for VP8. */
  OTC_VIDEO_CODEC_H264 = 2 /**< Video codec type for H.264. */
};

/** Video codec type type definition.
 */
typedef enum otc_video_codec_type otc_video_codec_type;

/** A structure representing the supported codecs for encoding and decoding
 * video.
 */
struct otc_media_utils_codecs {
  size_t number_encoder_video_codecs; /**< The number of video codecs supported
                                         by the video encoder. */
  otc_video_codec_type*
      encoder_video_codecs; /**< An array of video codecs supported by the video
                               encoder. */
  size_t number_decoder_video_codecs; /**< The number of video codecs supported
                                         by the video decoder. */
  otc_video_codec_type*
      decoder_video_codecs; /**< An array of video codecs supported by the video
                               decoder. */
};

/** Media codecs type definition.

    A type representing which video codecs are supported on the device.
 */
typedef struct otc_media_utils_codecs otc_media_utils_codecs;

/**
    Updates a otc_media_utils_codecs struct with the media codecs that are
   available on the device.

    @param supported_codecs Output parameter for a structure representing the
                            media codecs available on the device. The developer
                            is responsible for releasing the memory allocated
   after using it.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_media_utils_get_supported_codecs(otc_media_utils_codecs** supported_codecs);

/**
    Deletes an otc_media_utils_codecs instance.

    @param supported_codecs The otc_media_utils_codecs instance to be deleted.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_media_utils_codecs_delete(otc_media_utils_codecs* supported_codecs);

OTC_END_DECL

#endif  // MEDIA_UTILS_H
