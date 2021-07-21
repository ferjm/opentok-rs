/** @file subscriber.h
    @brief OpenTok Subscriber.

    This file includes the type definition for an OpenTok subscriber along with
    several function declarations.
*/
#ifndef SUBSCRIBER_H
#define SUBSCRIBER_H

#include "config.h"
#include "stream.h"
#include "error.h"
#include "video_frame.h"

OTC_BEGIN_DECL

#ifndef OTC_SESSION_FWD_DECL
#define OTC_SESSION_FWD_DECL
typedef struct otc_session otc_session;
#endif

/** OpenTok subscriber type definition.

    A type representing the consumer of audio and video from a stream in the
   OpenTok session.
 */
typedef struct otc_subscriber otc_subscriber;

/** Enum for reasons for a video to be started, stopped, resumed, etc.
 */
enum otc_video_reason {
  OTC_VIDEO_REASON_PUBLISH_VIDEO =
      1, /**< The client publishing the stream stopped streaming video. */
  OTC_VIDEO_REASON_SUBSCRIBE_TO_VIDEO =
      2, /**< The local client stopped subscribing to video. */
  OTC_VIDEO_REASON_QUALITY = 3, /**< The video event was caused by a change to
                                   the video stream quality. */
  OTC_VIDEO_REASON_CODEC_NOT_SUPPORTED = 4 /**< The video event was caused when
    video in the subscriber stream was disabled because the stream uses a video
    codec that is not supported on the device. */
};

/** Error code enumeration for OpenTok subscribers.

    This enumeration represents several error codes associated with a
   subscriber.
 */
enum otc_subscriber_error_code {
  OTC_SUBSCRIBER_INTERNAL_ERROR = 2000, /**< Internal error when subscribing. */
  OTC_SUBSCRIBER_SESSION_DISCONNECTED = 1010, /**< Attempting to subscribe to
    a stream in a session that you have disconnected from. */
  OTC_SUBSCRIBER_SERVER_CANNOT_FIND_STREAM = 1604, /**< The subscriber failed
    because the stream is missing. This can happen if the subscriber is created
    at the same time the stream is removed from the session. */
  OTC_SUBSCRIBER_STREAM_LIMIT_EXCEEDED = 1605,     /**< The client tried to
        subscribe     to a stream in a session that has exceeded the limit for
        simultaneous streams. */
  OTC_SUBSCRIBER_TIMED_OUT =
      1542, /**< Timeout while attempting to subscribe. */
  OTC_SUBSCRIBER_WEBRTC_ERROR = 1600, /**< Subscriber WebRTC failure. */
};

/**
    This struct represents subscriber audio stats, reported periodically
    by the {@link otc_subscriber_callbacks.on_audio_stats} callback function.
 */
struct otc_subscriber_audio_stats {
  uint64_t packets_lost;     /**< The total number of audio packets lost by the
                                subscriber. */
  uint64_t packets_received; /**< The total number of audio packets lost by the
                                subscriber. */
  uint64_t bytes_received; /**< The total number of audio bytes received by the
                              subscriber. */
  float audio_level;       /**< The audio level value, from 0 to 1.0. */
  double timestamp; /**< The timestamp, in milliseconds since the Unix epoch,
                       for when these stats were gathered. */
};

/**
  This struct represents subscriber video stats, reported periodically
  by the {@link otc_subscriber_callbacks.on_video_stats} callback function.

  Data passed into a callback function (other than `subscriber` and `user_data`)
  will be released after the callback is called. Make a copy of the data if you
  need to retain it.
 */
struct otc_subscriber_video_stats {
  uint64_t packets_lost;     /**< The total number of video packets lost by the
                                subscriber. */
  uint64_t packets_received; /**< The total number of video bytes received by
                                the subscriber. */
  uint64_t bytes_received; /**< The total number of video bytes received by the
                              subscriber. */
  double timestamp; /**< The timestamp, in milliseconds since the Unix epoch,
                       for when these stats were gathered. */
};

/**
 Defines the struct for setting the subcriber RTC stats report callack.

 @see otc_subscriber_set_rtc_stats_report_cb.
 */
struct otc_subscriber_rtc_stats_report_cb {
    /**
     Pointer to user custom data bound to this struct.
     */
    void *user_data;

    /**
     Defines the callback function for getting subscriber RTC stats reports.
     @param subscriber The instance invoking this call.
     @param user_data Pointer to user custom data bound to this struct.
     @param json_array_of_reports Array of RTC stats reports for
        the subscriber's stream. The structure of the JSON array is similar to
        the format of the RtcStatsReport object implemented in web browsers (see
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
    void(*on_rtc_stats_report)(otc_subscriber* subscriber,
                         void *user_data,
                         const char* json_array_of_reports);

    void *reserved;
};

/** Subscriber callback functions.

    This struct is a set of function pointers to functions that can be
    invoked for events related to an OpenTok subscriber.

    All callbacks will not be made on the application or main thread but on an
    internal thread. The application should return the callback as quickly as
    possible to avoid blocking the internal thread.
 */
struct otc_subscriber_callbacks {
  /**
      Called when the instance has successfully connected to the stream and
     begins playing media.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
      @param stream The stream associated with this event.
   */
  void (*on_connected)(otc_subscriber* subscriber,
                       void* user_data,
                       const otc_stream* stream);

  /**
      Called when the subscriber's stream has been interrupted.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
   */
  void (*on_disconnected)(otc_subscriber* subscriber, void* user_data);

  /**
      Called when the subscriber's stream has resumed.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
   */
  void (*on_reconnected)(otc_subscriber* subscriber, void* user_data);

  /**
      Called when a new video frame for the subscriber is ready to be rendered.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
      @param frame The video frame.
   */
  void (*on_render_frame)(otc_subscriber* subscriber,
                          void* user_data,
                          const otc_video_frame* frame);

  /**
      Called when the subscriber's video is disabled.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
      @param reason Gives more details about why the video has been disabled.
   */
  void (*on_video_disabled)(otc_subscriber* subscriber,
                            void* user_data,
                            enum otc_video_reason reason);

  /**
      Called when the subscriber's video is enabled.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
      @param reason Gives more details about why the video has been enabled.
   */
  void (*on_video_enabled)(otc_subscriber* subscriber,
                           void* user_data,
                           enum otc_video_reason reason);

  /**
      Called when the subscribe's audio is disabled.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
   */
  void (*on_audio_disabled)(otc_subscriber* subscriber, void* user_data);

  /**
      Called when the subscriber's audio is enabled.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
   */
  void (*on_audio_enabled)(otc_subscriber* subscriber, void* user_data);

  /**
      Called when an subscriber initially receives video data.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
   */
  void (*on_video_data_received)(otc_subscriber* subscriber, void* user_data);

  /**
      Called when the OpenTok Media Router determines that the stream quality
      has degraded and the video will be disabled if the quality degrades
      further.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
   */
  void (*on_video_disable_warning)(otc_subscriber* subscriber, void* user_data);

  /**
      Called when the OpenTok Media Router determines that the stream quality
      has improved to the point at which the video being disabled is not an
      immediate risk.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
   */
  void (*on_video_disable_warning_lifted)(otc_subscriber* subscriber,
                                          void* user_data);

  /**
      Called periodically to report audio statistics for the subscriber.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
      @param audio_stats A pointer to the audio stats struct.
   */
  void (*on_audio_stats)(otc_subscriber* subscriber,
                         void* user_data,
                         struct otc_subscriber_audio_stats audio_stats);

  /**
      Called periodically to report video statistics for the subscriber.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
      @param video_stats A pointer to the video stats struct.
   */
  void (*on_video_stats)(otc_subscriber* subscriber,
                         void* user_data,
                         struct otc_subscriber_video_stats video_stats);

  /**
      Called periodically to report the audio level of the subscriber.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
      @param audio_level The audio level value, from 0 to 1.0.
   */
  void (*on_audio_level_updated)(otc_subscriber* subscriber,
                                 void* user_data,
                                 float audio_level);

  /**
     Called when the subscriber fails.

      @param subscriber The instance invoking this call.
      @param user_data Pointer to user custom data bound to this struct.
      @param error_string A string containing the error message.
      @param error_code Error code enum value.
   */
  void (*on_error)(otc_subscriber* subscriber,
                   void* user_data,
                   const char* error_string,
                   enum otc_subscriber_error_code error);

  /**
      This struct member can point to data related to the subscriber
      which the developer might need.
   */
  void* user_data;

  /**
      A void pointer to a memory area holding reserved resources used for the
      internal implementation.
   */
  void* reserved;
};

/**
    Creates a new Subscriber for a given Stream.

    @param stream The Stream object corresponding to the stream you will
   subscribe to.
    @param callbacks Struct with function pointers of the subscriber's
   callbacks.
    @return The new Subscriber instance. This can be null if there is an error.
 */
OTC_DECL(otc_subscriber*)
otc_subscriber_new(const otc_stream* stream,
                   const struct otc_subscriber_callbacks* callbacks);

/**
    Releases a subscriber instance, including all hardware and UI resources
   bound to it.

    @param subscriber The subcriber to be deleted.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status) otc_subscriber_delete(otc_subscriber* subscriber);

/**
    Returns the stream associated with the subscriber.

    @param subscriber The subscriber instance to get the stream from.
    @return The stream associated with the subscriber. This can be null if there
   is an error.
 */
OTC_DECL(otc_stream*)
otc_subscriber_get_stream(const otc_subscriber* subscriber);

/**
    Subscribes to the stream's video.

    @param subscriber The subscriber instance which will be affected.
    @param subscribe_to_video Whether to subscribe to video or not.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_subscriber_set_subscribe_to_video(otc_subscriber* subscriber,
                                      otc_bool subscribe_to_video);

/**
    Subscribes to the stream's audio.

    @param subscriber The subscriber instance which will be affected.
    @param subscribe_to_audio Whether to subscribe to audio or not.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_subscriber_set_subscribe_to_audio(otc_subscriber* subscriber,
                                      otc_bool subscribe_to_audio);

/**
    Returns whether the subscriber is subscribed to the stream's video or not.

    @param subscriber The subscriber instance.
    @return Whether the subscriber is subscribed to the stream's video or not.
 */
OTC_DECL(otc_bool)
otc_subscriber_get_subscribe_to_video(const otc_subscriber* subscriber);

/**
    Returns whether the subscriber is subscribed to the stream's audio or not.

    @param subscriber The subscriber instance.
    @return Whether the subscriber is subscribed to the stream's audio or not.
 */
OTC_DECL(otc_bool)
otc_subscriber_get_subscribe_to_audio(const otc_subscriber* subscriber);

/**
    Returns the {@link otc_session} associated this instance.

    @param subscriber The subscriber instance.
    @return The {@link otc_session} associated this instance. This can be null
   if there is an error.
 */
OTC_DECL(otc_session*)
otc_subscriber_get_session(const otc_subscriber* subscriber);

/**
    Sets the preferred resolution for the subscriber's stream.

    @param subscriber The subscriber instance which will be affected.
    @param preferred_width The preferred width of the subscriber.
    @param preferred_height The preferred height of the subscriber.
    @return Return value indicating either error or success.
    @see otc_error_code
 */
OTC_DECL(otc_status)
otc_subscriber_set_preferred_resolution(otc_subscriber* subscriber,
                                        uint32_t preferred_width,
                                        uint32_t preferred_height);
/**
    Returns the preferred resolution for the subscriber's stream.

    @param subscriber The subscriber instance.
    @param preferred_width The preferred width of the subscriber.
    @param preferred_height The preferred height of the subscriber.
    @return Return value indicating either error or success.
    @see otc_error_code
 */
OTC_DECL(otc_status)
otc_subscriber_get_preferred_resolution(const otc_subscriber* subscriber,
                                        uint32_t* preferred_width,
                                        uint32_t* preferred_height);

/**
    Sets the preferred frame rate for the subscriber's stream.

    @param subscriber The subscriber instance which will be affected.
    @param preferred_framerate The preferred framerate of the subscriber.
    @return Return value indicating either error or success.
    @see otc_error_code
 */
OTC_DECL(otc_status)
otc_subscriber_set_preferred_framerate(otc_subscriber* subscriber,
                                       float preferred_framerate);

/**
    Returns the preferred frame rate for the subscriber's stream.

    @param subscriber The subscriber instance.
    @param preferred_framerate The preferred framerate associated with the
   subscriber.
    @return Return value indicating either error or success.
    @see otc_error_code
 */
OTC_DECL(otc_status)
otc_subscriber_get_preferred_framerate(const otc_subscriber* subscriber,
                                       float* preferred_framerate);

/**
    Gets a unique identifier for a given subscriber.

    @param subscriber The subscriber instance.
    @return A unique identifier for the subscriber. This can be null if there is
   an error.
 */
OTC_DECL(const char*)
otc_subscriber_get_subscriber_id(const otc_subscriber* subscriber);

/**
    Gets user data for a given subscriber.

    @param subscriber The subscriber instance.
    @return A pointer to the user data in memory.
 */
OTC_DECL(void*) otc_subscriber_get_user_data(const otc_subscriber* subscriber);

/**
    Gets the RTC stats report for the subscriber. This is an asynchronous operation.
    create an otc_subscriber_rtc_stats_report_cb struct and pass it into the
    {@link otc_subscriber_set_rtc_stats_report_cb} function prior to calling
    this function. When the stats are available, the
    {@link otc_subscriber_set_rtc_stats_report_cb} callback function is called.

    Also see {@link otc_subscriber_callbacks.on_audio_stats},
    {@link otc_subscriber_callbacks.on_video_stats}, and
    {@link otc_publisher_get_rtc_stats_report}.
    
    @param subscriber The subscriber.
 */
OTC_DECL(otc_status) otc_subscriber_get_rtc_stats_report(const otc_subscriber *subscriber);

/**
   Sets the RTC stats report callback the subscriber. See otc_subscriber_get_rtc_stats_report.

   @param subscriber The subscriber instance.

   @param cb The otc_subscriber_rtc_stats_report_cb struct that includes the callback function
   for the RTC stats report.
 */
OTC_DECL(otc_status) otc_subscriber_set_rtc_stats_report_cb(otc_subscriber *subscriber, struct otc_subscriber_rtc_stats_report_cb cb);

OTC_END_DECL

#endif  // SUBSCRIBER_H
