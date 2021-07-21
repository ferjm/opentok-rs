/** @file publisher.h
    @brief OpenTok Publisher.

    This file includes the type definition for an OpenTok publisher along with
    several function declarations.
*/
#ifndef PUBLISHER_H
#define PUBLISHER_H

#include "config.h"
#include "stream.h"
#include "video_frame.h"

OTC_BEGIN_DECL

#ifndef OTC_SESSION_FWD_DECL
#define OTC_SESSION_FWD_DECL
typedef struct otc_session otc_session;
#endif

/** OpenTok publisher type definition.

    A type representing a publisher of an audio-video stream to an OpenTok
   session.
 */
typedef struct otc_publisher otc_publisher;

/** OpenTok publisher settings type definition.
 */
typedef struct otc_publisher_settings otc_publisher_settings;

/** Error code enumeration for OpenTok publishers.

    This enumeration represents several error codes associated with a publisher.
 */
enum otc_publisher_error_code {
  OTC_PUBLISHER_INTERNAL_ERROR = 2000, /**< Internal error. */
  OTC_PUBLISHER_SESSION_DISCONNECTED =
      1010, /**< Error attempting to a session that you are not connected to. */
  OTC_PUBLISHER_TIMED_OUT = 1541, /**< Time-out attempting to publish. */
  OTC_PUBLISHER_UNABLE_TO_PUBLISH = 1500, /**< Unable to publish error. */
  OTC_PUBLISHER_WEBRTC_ERROR = 1610,      /**< WebRTC error. */
};

/** Publisher video type enumeration.

    This enumeration represents the different video types supported.
 */
enum otc_publisher_video_type {
  OTC_PUBLISHER_VIDEO_TYPE_CAMERA = 1, /**< Camera video stream. */
  OTC_PUBLISHER_VIDEO_TYPE_SCREEN = 2  /**< Screen capture video stream. */
};

/** Publisher audio stats.

    This structure represents the publisher audio stats.
    The otc_publisher_callbacks struct includes a function pointer to
    an on_audio_stats function, which is called periodically to report
    audio stats.
 */
struct otc_publisher_audio_stats {
  const char* connection_id; /**< The connection ID of the client subscribing to
                                the stream. */
  const char* subscriber_id; /**< The subscriber ID of the client subscribing to
                                the stream (in a relayed session). */
  int64_t
      packets_lost; /**< The total number of audio packets that did not reach
                       the subscriber (or the OpenTok Media Router). */
  int64_t packets_sent; /**< The total number of audio packets sent to the
                           subscriber (or to the OpenTok Media Router). */
  int64_t bytes_sent;   /**< The total number of audio bytes sent to the
                           subscriber (or to the OpenTok Media Router). */
  float audio_level;    /**< The audio level value, from 0 to 1.0. */
  double timestamp; /**< The timestamp, in milliseconds since the Unix epoch,
                       for when these stats were gathered. */
  double
      start_time; /**< The timestamp, in milliseconds since the Unix epoch, from
                     which the cumulative totals started accumulating. */
};

/** Publisher video stats.

    This structure represents the publisher video stats.
    The otc_publisher_callbacks struct includes a function pointer to
    an on_video_stats function, which is called periodically to report
    video stats.
 */
struct otc_publisher_video_stats {
  const char* connection_id; /**< The connection ID of the client subscribing to
                                the stream. */
  const char* subscriber_id; /**< The subscriber ID of the client subscribing to
                                the stream (in a relayed session). */
  int64_t
      packets_lost; /**< The total number of video packets packets that did not
                       reach the subscriber (or the OpenTok Media Router). */
  int64_t packets_sent; /**< The total number of video packets sent sent to the
                           subscriber (or to the OpenTok Media Router). */
  int64_t bytes_sent;   /**< The total number of video bytes bytes sent to the
                           subscriber (or to the OpenTok Media Router). */
  double timestamp; /**< The timestamp, in milliseconds since the Unix epoch,
                       for when these stats were gathered. */
  double
      start_time; /**< The timestamp, in milliseconds since the Unix epoch, from
                     which the cumulative totals started accumulating. */
};

/** Publisher callback functions.

    This structure is a set of function pointers to callback functions that can
   be called in response to events related to an OpenTok publisher.

    All callbacks will not be made on the application or main thread but on an
    internal thread. The application should return the callback as quickly as
    possible to avoid blocking the internal thread.

    Data passed into a callback function (other than `publisher` and
   `user_data`) will be released after the callback is called. Make a copy of
   the data if you need to retain it.
 */
struct otc_publisher_callbacks {
  /**
      Called when the publisher's stream is created.

      @param publisher A pointer to the publisher.
      @param user_data A pointer to the user_data you set for the publisher.
      @param stream A pointer to the stream.
   */
  void (*on_stream_created)(otc_publisher* publisher,
                            void* user_data,
                            const otc_stream* stream);

  /**
      Called when the publisher's stream is destroyed.

      @param publisher A pointer to the publisher.
      @param user_data A pointer to the user_data you set for the publisher.
      @param strem A pointer to the stream.
   */
  void (*on_stream_destroyed)(otc_publisher* publisher,
                              void* user_data,
                              const otc_stream* stream);

  /**
      Called when there is a new frame ready to be rendered by the publisher.

      @param publisher A pointer to the publisher.
      @param user_data A pointer to the user_data you set for the publisher.
      @param frame A pointer to the new video frame.
   */
  void (*on_render_frame)(otc_publisher* publisher,
                          void* user_data,
                          const otc_video_frame* frame);

  /**
      Called periodically to report the audio level of the publisher.

      @param publisher A pointer to the publisher.
      @param user_data A pointer to the user_data you set for the publisher.
      @param audio_level The audio level value, from 0 to 1.0.
   */
  void (*on_audio_level_updated)(otc_publisher* publisher,
                                 void* user_data,
                                 float audio_level);

  /**
      Called periodically to report audio statistics for the publisher.

      @param publisher A pointer to the publisher.
      @param user_data A pointer to the user_data you set for the publisher.
      @param audio_stats An array of publisher audio stats.
      @param number_of_stats The number of audio stats in the array.
   */
  void (*on_audio_stats)(otc_publisher* publisher,
                         void* user_data,
                         struct otc_publisher_audio_stats audio_stats[],
                         size_t number_of_stats);

  /**
      Called periodically to report video statistics for the publisher.

      @param publisher A pointer to the publisher.
      @param user_data A pointer to the user_data you set for the publisher.
      @param video_stats An array of publisher video stats.
      @param number_of_stats The number of video stats in the array.
   */
  void (*on_video_stats)(otc_publisher* publisher,
                         void* user_data,
                         struct otc_publisher_video_stats video_stats[],
                         size_t number_of_stats);

  /**
     Called when the publisher fails.

      @param publisher A pointer to the publisher.
      @param user_data A pointer to the user_data you set for the publisher.
      @param error_string A string containing the error message.
      @param error_code An error code enum value.
   */
  void (*on_error)(otc_publisher* publisher,
                   void* user_data,
                   const char* error_string,
                   enum otc_publisher_error_code error_code);

  /**
    A pointer to data you set related to the publisher.
   */
  void* user_data;

  /**
      A void pointer to a memory area holding reserved resources used for the
      internal implementation.
   */
  void* reserved;
};

/**
    Creates a new otc_publisher instance.

    @param name The name of the publisher. Other clients can get the names
                    for streams in the session.
    @param capturer Use this parameter if you want to provide a custom video
                    capturer. If it is set to null, the publisher uses a default
                    video capturer using the system's camera.
    @param callbacks A pointer to the structure with the publisher callback
   function pointers.
    @return A new otc_publisher instance. If there is an error, this is null.
 */
OTC_DECL(otc_publisher*)
otc_publisher_new(const char* name,
                  const struct otc_video_capturer_callbacks* capturer,
                  const struct otc_publisher_callbacks* callbacks);

/**
    Creates a new otc_publisher_settings instance.

    @return A new otc_publisher_settings instance
 */
OTC_DECL(otc_publisher_settings*) otc_publisher_settings_new();

/**
    Deletes an otc_publisher_settings instance.

    @param settings The otc_publisher_settings instance to be deleted.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_settings_delete(otc_publisher_settings* settings);

/**
    Sets the desired name for a publisher to be constructed with
    {@link otc_publisher_new_with_settings}.

    @param settings The otc_publisher_settings instance to be affected.
    @param name The name for the publisher. Other clients can get the names
                    for streams in the session.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_settings_set_name(otc_publisher_settings* settings,
                                const char* name);

/**
    Sets the video capturer for a publisher to be constructed with
    {@link otc_publisher_new_with_settings}.

    @param settings The otc_publisher_settings instance to be affected.
    @param capturer The video capturer.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_settings_set_video_capturer(
    otc_publisher_settings* settings,
    const struct otc_video_capturer_callbacks* capturer);

/**
    Enables an audio track for a publisher to be constructed with
    {@link otc_publisher_new_with_settings}.

    @param settings The otc_publisher_settings instance to be affected.
    @param enabled Enable/disable the existence of an audio track for
                   a publisher to be constructed with {@link
   otc_publisher_new_with_settings}.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_settings_set_audio_track(otc_publisher_settings* settings,
                                       otc_bool enabled);

/**
    Enables a video track for a publisher to be constructed with
    {@link otc_publisher_new_with_settings}.

    @param settings The otc_publisher_settings instance to be affected.
    @param enabled Enable/disable the existence of an video track for
                   a publisher to be constructed with {@link
   otc_publisher_new_with_settings}.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_settings_set_video_track(otc_publisher_settings* settings,
                                       otc_bool enabled);

/**
    Enables stereo audio for a publisher to be constructed with
    {@link otc_publisher_new_with_settings}.

    @param settings The otc_publisher_settings instance to be affected.
    @param enabled Enable/disable stereo audio support for a publisher
                   to be constructed with {@link
   otc_publisher_new_with_settings}.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_settings_set_stereo(otc_publisher_settings* settings,
                                  otc_bool enabled);

/**
    Creates a new otc_publisher instance.

    @param callbacks A pointer to the struct with publisher callback function
   pointers.
    @param settings The settings struct containing the desired settings for the
   publisher.
    @return A new otc_publisher instance. If there is an error, this is null.
 */
OTC_DECL(otc_publisher*)
otc_publisher_new_with_settings(const struct otc_publisher_callbacks* callbacks,
                                otc_publisher_settings* settings);

/**
    Releases a publisher instance, including all hardware resources bound to it.

    @param publisher The publisher instance to be deleted.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status) otc_publisher_delete(otc_publisher* publisher);

/**
    Returns the stream associated with the publisher.

    @param publisher The publisher instance to get the stream from.
    @return The stream associated with the publisher. If there is an error, this
   is null.
 */
OTC_DECL(otc_stream*) otc_publisher_get_stream(otc_publisher* publisher);

/**
    Whether to publish video or not. By default, streams publish both audio and
   video.

    @param publisher The publisher to be affected.
    @param publish_video Whether to publish video or not.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_set_publish_video(otc_publisher* publisher,
                                otc_bool publish_video);

/**
    Whether to publish audio or not. By default, streams publish both audio and
   video.

    @param publisher The publisher to be affected.
    @param publish_audio Whether to publish audio or not.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_set_publish_audio(otc_publisher* publisher,
                                otc_bool publish_audio);

/**
    Returns whether the publisher is publishing video or not.

    @param publisher The publisher instance.
    @return  Whether the publisher is publishing video or not.
 */
OTC_DECL(otc_bool)
otc_publisher_get_publish_video(const otc_publisher* publisher);

/**
    Returns whether the Publisher is publishing audio or not.

    @param publisher The publisher instance.
    @return  Whether the publisher is publishing audio or not.
 */
OTC_DECL(otc_bool)
otc_publisher_get_publish_audio(const otc_publisher* publisher);

/**
    Returns the {@link otc_session} associated this publisher instance.

    @param publisher The publisher instance.
    @return The {@link otc_session} associated this instance. If there is an
   error, this is null.
 */
OTC_DECL(otc_session*)
otc_publisher_get_session(const otc_publisher* publisher);

/**
    Sets the max audio bitrate for the publisher.

    @param publisher The publisher to be affected.
    @param bitrate The desired bitrate.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_set_max_audio_bitrate(otc_publisher* publisher, uint32_t bitrate);

/**
  Represents RTC statistics for a media stream published by the publisher.

  @see otc_publisher_get_rtc_stats_report
  @see otc_publisher_set_rtc_stats_report_cb
 */
struct otc_publisher_rtc_stats {
  /**
   For a relayed session (in which a publisher sends individual media streams
   to each subscriber), this is the unique ID of the client's connection.
  */
  const char* connection_id;
  /**
  A JSON array of RTC stats reports for the media stream.
  The structure of the JSON array is similar to the format of
  the RtcStatsReport object implemented in web browsers (see
  <a href="https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport">Mozilla
  docs</a>). Also see <a href="https://w3c.github.io/webrtc-stats/#summary">this
  W3C documentation</a>.

  Here is a partial sample:

  <pre>
   [
      {
          "audioLevel": 0,
          "id": "RTCAudioSource_1",
          "kind": "audio",
          "timestamp": 1603448671532842,
          "totalAudioEnergy": 0,
          "totalSamplesDuration": 4.249999999999954,
          "trackIdentifier": "4nP5yeIDzbUm6IJho5jkTps1lnfabsFvTXjH00",
          "type": "media-source"
      },
      {
          "base64Certificate": "MIIBFjCB...QIhAMIfr/NgvhNp16zaoHxGQGrID1STFmBSSSB4V1bxBrTU",
          "fingerprint": "E7:5F:...:FA:5A:1F:A7:E0:55:60",
          "fingerprintAlgorithm": "sha-256",
          "id": "RTCCertificate_E7:5F:E5...F:1D:FA:5A:1F:A7:E0:55:60",
          "timestamp": 1603448671532842,
          "type": "certificate"
      },
      {
          "channels": 1,
          "clockRate": 8000,
          "id": "RTCCodec_audio_Inbound_0",
          "mimeType": "audio/PCMU",
          "payloadType": 0,
          "timestamp": 1603448671532842,
          "type": "codec"
      },
      {
          "channels": 2,
          "clockRate": 48000,
          "id": "RTCCodec_audio_Inbound_111",
          "mimeType": "audio/opus",
          "payloadType": 111,
          "sdpFmtpLine": "minptime=10;useinbandfec=1",
          "timestamp": 1603448671532842,
          "type": "codec"
      },
   ]
   </pre>
  */
  const char* json_array_of_reports;
};

/**
 Defines the struct for setting the publisher RTC stats report callack.

 @see otc_publisher_set_rtc_stats_report_cb.
 */
struct otc_publisher_rtc_stats_report_cb {
  /**
   Pointer to user custom data bound to this struct.
   */
  void *user_data;
  /**
   Defines the callback function for getting subscriber RTC stats reports.

   @param publisher The instance invoking this call.

   @param stats A pointer to an array defining the RTC statistics for the publisher.
     For a routed session (a seesion that uses the
     <a href="https://tokbox.com/developer/guides/create-session/#media-mode">OpenTok
     Media Router</a>), this array includes one element, defining the statistics
     for the single video media stream that is sent to the OpenTok Media Router.
     In a relayed session, the array includes an object for each subscriber
     to the published stream.

   @param entries The size of the stats array.
   */
  void(*on_rtc_stats_report)(otc_publisher* publisher,
                             void *user_data,
                             const struct otc_publisher_rtc_stats* stats,
                             size_t entries);
  void *reserved;
};

/**
    Sets the RTC stats report callback the publisher.

    @param subscriber The subscriber instance.

    @param cb The otc_publisher_rtc_stats_report_cb struct that includes the callback function
    for the RTC stats report.

    @see otc_publisher_get_rtc_stats_report.
 */
OTC_DECL(otc_status) otc_publisher_set_rtc_stats_report_cb(otc_publisher *publisher,
                                                           const struct otc_publisher_rtc_stats_report_cb cb);
/**
    Gets the RTC stats report for the subscriber. This is an asynchronous operation.
    create an otc_publisher_rtc_stats_report_cb struct and pass it into the
    {@link otc_publisher_set_rtc_stats_report_cb} function prior to calling
    this function. When the stats are available, the
    {@link otc_publisher_set_rtc_stats_report_cb} callback function is called.

    Also see {@link otc_publisher_callbacks.on_audio_stats},
    {@link otc_publisher_callbacks.on_video_stats}, and
    {@link otc_subscriber_get_rtc_stats_report}.
    
    @param publisher The publisher.
 */
OTC_DECL(otc_status) otc_publisher_get_rtc_stats_report(const otc_publisher *publisher);

/**
    Enables or disables the audio fallback feature.

    The audio-fallback feature is available in sessions that use
    the OpenTok Media Router. With the audio-fallback feature enabled (the
   default), when the OpenTok Media Router determines that a stream's quality
   has degraded significantly for a specific subscriber to the stream, it
   disables the video in that subscriber in order to preserve audio quality.

    To turn off the audio-fallback feature, call the
    {@link otc_publisher_set_audio_fallback_enabled} function (and pass in
    OTC_FALSE) before calling the {@link otc_session_publish} function.

    @param publisher The publisher to be affected.
    @param enabled Whether we want to enable the audio fallback feature or not.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_set_audio_fallback_enabled(otc_publisher* publisher,
                                         otc_bool enabled);

/**
    Returns whether the publisher audio fallback feature is enabled or not.
    See the {@link otc_publisher_set_audio_fallback_enabled} function.

    @param publisher The publisher instance.
    @return Whether the publisher audio fallback feature is enabled or not.
 */
OTC_DECL(otc_bool)
otc_publisher_get_audio_fallback_enabled(const otc_publisher* publisher);

/**
    Gets a unique identifier for a publisher.

    @param publisher The publisher instance.
    @return A unique identifier for the publisher. If there is an error, this is
   null.
 */
OTC_DECL(const char*)
otc_publisher_get_publisher_id(const otc_publisher* publisher);

/**
    Sets the publisher video type. By default, videos have the video type set
    to OTC_PUBLISHER_VIDEO_TYPE_CAMERA (indicating the source of the video is a
   camera). Set this to OTC_PUBLISHER_VIDEO_TYPE_SCREEN to indicate that the
   video source is screen sharing. Other clients can detect the video type for
   streams in the session (to determine the video source type).

    @param publisher The publisher to be affected.
    @param video_type The video type for the publisher.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_publisher_set_video_type(struct otc_publisher* publisher,
                             enum otc_publisher_video_type video_type);
/**
    Gets the publisher video type.
    See the {@link otc_publisher_set_video_type} function.

    @param publisher The publisher instance.
    @return The publisher video type.
 */
OTC_DECL(enum otc_publisher_video_type)
otc_publisher_get_video_type(const struct otc_publisher* publisher);

/**
    Gets user data for a given publisher.

    @param publisher The publisher instance.
    @return A pointer to the user data in memory.
 */
OTC_DECL(void*)
otc_publisher_get_user_data(const struct otc_publisher* publisher);

/**
    Gets the name for a given publisher.

    @param publisher The publisher instance.
    @return The name.
 */
OTC_DECL(const char*)
otc_publisher_get_name(const struct otc_publisher* publisher);

OTC_END_DECL

#endif  // PUBLISHER_H
