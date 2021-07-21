/** @file session.h
    @brief OpenTok Session.

    This file includes the type definition for an OpenTok session along with
    several function declarations useful when dealing with a session.
*/
#ifndef SESSION_H
#define SESSION_H

#include "capabilities.h"
#include "config.h"
#include "publisher.h"
#include "subscriber.h"
#include "stream.h"

OTC_BEGIN_DECL

#ifdef DOXYGEN_OTC_SESSION_TYPEDEF
/** OpenTok session type definition.
 */
typedef struct otc_session otc_session;
#endif

#ifndef OTC_SESSION_FWD_DECL
#define OTC_SESSION_FWD_DECL
typedef struct otc_session otc_session;
#endif

/** OpenTok session settings type definition.
 */
typedef struct otc_session_settings otc_session_settings;

/** Error code enumeration for OpenTok sessions.

    This enumeration represents several error codes associated with a session.
 */
enum otc_session_error_code {
  OTC_SESSION_AUTHORIZATION_FAILURE =
      1004, /**< An invalid API key or token was provided.*/
  OTC_SESSION_BLOCKED_COUNTRY =
      1026, /**< Unable to connect to the session. Terms of service violation:
               export compliance. */
  OTC_SESSION_CONNECTION_DROPPED =
      1022, /**< The connection to the OpenTok messaging server was dropped. */
  OTC_SESSION_CONNECTION_FAILED =
      1006, /**< Connecting to the session failed. */
  OTC_SESSION_CONNECTION_LIMIT_EXCEEDED =
      1027, /**< The client tried to connect to a session that has exceeded the
               limit for simultaneous connections. */
  OTC_SESSION_CONNECTION_REFUSED =
      1023, /**< A socket could not be opened to the messaging server. Check
               that outbound ports 443 and 8080 are accessible. */
  OTC_SESSION_CONNECTION_TIMED_OUT =
      1021, /**< The connection timed out while attempting to connect to the
               session */
  OTC_SESSION_FORCE_UNPUBLISH_OR_INVALID_STREAM = 1535, /**< Invalid stream. */
  OTC_SESSION_ILLEGAL_STATE =
      1015, /**< A method has been invoked at an illegal or inappropriate time
               for this session. For example, attempting to connect an already
               connected session will return this error. */
  OTC_SESSION_INTERNAL_ERROR =
      2000, /**< Thread dispatch failure, out of memory, parse error, etc. */
  OTC_SESSION_INVALID_SESSION =
      1005, /**< An invalid session ID was provided. */
  OTC_SESSION_INVALID_SIGNAL_TYPE =
      1461, /**< You attempted to send a signal with an invalid type. */
  OTC_SESSION_NOT_CONNECTED =
      1010, /**< The session is not connected, and the requested action requires
               an active session connection. */
  OTC_SESSION_NO_MESSAGING_SERVER =
      1503, /**< No messaging server is available for this session. */
  OTC_SESSION_NULL_OR_INVALID_PARAMETER =
      1011, /**< A parameter passed in is null or invalid. */
  OTC_SESSION_PUBLISHER_NOT_FOUND =
      1113, /**< The publisher is unknown to this session. This is usually the
               result of attempting to unpublish a publisher that is not
               associated with the session. */
  OTC_SESSION_SIGNAL_DATA_TOO_LONG =
      1413, /**< You attempted to send a signal with a data string that is
               greater than the maximum length (8KB). */
  OTC_SESSION_SIGNAL_TYPE_TOO_LONG =
      1414, /**< You attempted to send a signal with a type string that is
               greater than the maximum length. */
  OTC_SESSION_STATE_FAILED = 1020, /**< The connection timed out while
                                      attempting to get the session’s state. */
  OTC_SESSION_SUBSCRIBER_NOT_FOUND =
      1112, /**< The subscriber is unknown to this session. This is usually the
               result of attempting to unsubscribe a subscriber that is not
               associated with the session. */
  OTC_SESSION_UNEXPECTED_GET_SESSION_INFO_REPONSE =
      2001, /**< Unexpected response. */
};

/** Session capabilities.

    This struct represents the capabilities for a client in a given session.
 */
struct otc_session_capabilities {
  otc_bool publish; /**< Whether the client can publish streams to the session
    (OTC_TRUE) or not (OTC_FALSE). */
};

/** ICE server configuration.

    This struct represents the ICE server configuration for a given session.
    This is part of the
    <a href="https://tokbox.com/developer/guides/configurable-turn-servers">
    configurable TURN feature</a>.
 */
struct otc_custom_ice_config {
  int num_ice_servers; /**< The number of custom TURN servers used. */
  char** ice_url;  /**< An array of strings specifying your ICE server URLs. **/
  char** ice_user; /**< An array of strings specifying specifying usernames for
                      the TURN servers. **/
  char** ice_credential; /**< An array of credentials for the TURN servers. */
  otc_bool force_turn;   /**< Whether the client will force connectivity through
       TURN always and ignore all other ICE candidates (OTC_TRUE). When set to
       OTC_FALSE,   the client will use all ICE routing types (such as host, srflx,
       and TURN)   to establish media connectivity. */
  otc_bool use_custom_turn_only; /**< Whether the client will use custom TURN
     servers only (OTC_TRUE). When set to OTC_FALSE, the client will use both
     OpenTok TURN servers and (if any are added) custom TURN servers. */
};

/** Session signal options.

    This struct represents options associated with an OpenTok signal.
    See {@link otc_session_send_signal_with_options} and
    {@link otc_session_send_signal_to_connection_with_options}.
 */
struct otc_signal_options {
  /**
    Upon reconnecting to the session, whether to send any signals that were
    initiated while disconnected. If your client loses its connection to
    the OpenTok session, due to a drop in network connectivity, the client
    attempts to reconnect to the session, and the {@link
    otc_session_callbacks.on_disconnected} callback function is invoked. By
    default, signals initiated while disconnected are sent when (and if) the
    client reconnects to the OpenTok session. You can prevent this by setting
    the retry_after_reconnect member to OTC_FALSE. (The default setting is
    OTC_TRUE.)
   */
  otc_bool retry_after_reconnect;
};

/** Session callback functions.

    This struct is a set of function pointers to callback functions for
    events related to an OpenTok session.

    All callbacks will not be made on the application or main thread but on an
    internal thread. The application should return the callback as quickly as
    possible to avoid blocking the internal thread.

    Data passed into a callback function (other than `session` and `user_data`)
    will be released after the callback is called. Make a copy of the data if
   you need to retain it.
 */
struct otc_session_callbacks {
  /**
      Called when the {@link otc_session_connect} function successfully connects
      the client to an OpenTok session.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.
   */
  void (*on_connected)(otc_session* session, void* user_data);

  /**
      Called when the client is no longer connected to the OpenTok session.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.
   */
  void (*on_disconnected)(otc_session* session, void* user_data);

  /**
      Called when a new connection (from another client) is created.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.

      @param connection The {@link otc_connection} struct corresponding to the
             client connecting to the session.
   */
  void (*on_connection_created)(otc_session* session,
                                void* user_data,
                                const otc_connection* connection);

  /**
      Called when another client's connection to the session is dropped.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.

      @param connection The {@link otc_connection} struct corresponding to the
             client disconnecting from the session.
   */
  void (*on_connection_dropped)(otc_session* session,
                                void* user_data,
                                const otc_connection* connection);

  /**
      Called when a there is a new stream in this OpenTok session.
      Call the {@link otc_session_subscribe} function to subscribe
      to the stream.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.

      @param stream The {@link otc_stream} struct representing the new stream.
   */
  void (*on_stream_received)(otc_session* session,
                             void* user_data,
                             const otc_stream* stream);

  /**
      Called when another client's stream is dropped from this OpenTok session.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.

      @param stream The {@link otc_stream} struct representing the stream.
   */
  void (*on_stream_dropped)(otc_session* session,
                            void* user_data,
                            const otc_stream* stream);
  /**
      Called when a stream toggles audio on or off.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.

      @param stream The {@link otc_stream} struct representing the new stream.

      @param has_audio Whether the stream now has audio (OTC_TRUE) or not
     (OTC_FALSE).
   */
  void (*on_stream_has_audio_changed)(otc_session* session,
                                      void* user_data,
                                      const otc_stream* stream,
                                      otc_bool has_audio);

  /**
      Called when a stream toggles video on or off.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.

      @param stream Whether the stream now has video (OTC_TRUE) or not
     (OTC_FALSE).
   */
  void (*on_stream_has_video_changed)(otc_session* session,
                                      void* user_data,
                                      const otc_stream* stream,
                                      otc_bool has_video);

  /**
      Called when the video dimensions of a stream in the session change.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.

      @param width The new width of the stream, in pixels.

      @param height The new beigbt of the stream, in pixels.
   */
  void (*on_stream_video_dimensions_changed)(otc_session* session,
                                             void* user_data,
                                             const otc_stream* stream,
                                             int width,
                                             int height);

  /**
      Called when the video type of a stream in the session changes.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.

      @param type The new stream type.
   */
  void (*on_stream_video_type_changed)(otc_session* session,
                                       void* user_data,
                                       const otc_stream* stream,
                                       enum otc_stream_video_type type);
  /**
     Called when a signal is received in the session. See the
     <a href="https://tokbox.com/developer/guides/signaling/">
     Signaling overview</a> documentation.

     @param session A pointer to the otc_session struct.

     @param user_data A pointer to the user_data you set for the session.

     @param type The type string for the signal (if one was provided when
            the signal was sent).

     @param signal The data string for the signal (if one was provided when
            the signal was sent).

     @param connect The {@link otc_connection} representing the client that
            sent the signal.
   */
  void (*on_signal_received)(otc_session* session,
                             void* user_data,
                             const char* type,
                             const char* signal,
                             const otc_connection* connection);

  /**
      Called when the local client has lost its connection to the OpenTok
     session and is trying to reconnect.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.
   */
  void (*on_reconnection_started)(otc_session* session, void* user_data);

  /**
      Called when the local client has reconnected to the OpenTok session
      after its network connection was lost temporarily.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.
   */
  void (*on_reconnected)(otc_session* session, void* user_data);

  /**
      Called when an archive of the session starts recording.
      See the <a href="https://tokbox.com/developer/guides/archiving/">
      Archiving developer guide</a>.

      @param session A pointer to the otc_session struct.

      @param archive_id The archive ID.

      @param name The name of the archive (if one was provided when
             the archive was created).

   */
  void (*on_archive_started)(otc_session* session,
                             void* user_data,
                             const char* archive_id,
                             const char* name);

  /**
      Called when an archive of the session stops being recorded.

      @param session A pointer to the otc_session struct.

      @param user_data A pointer to the user_data you set for the session.

      @param archive_id The archive ID.
   */
  void (*on_archive_stopped)(otc_session* session,
                             void* user_data,
                             const char* archive_id);
  /**
     Called when the session fails.

     @param session A pointer to the otc_session struct.

     @param user_data A pointer to the user_data you set for the session.

     @param error_string The error string.

     @param error The error code (of type {@link otc_session_error_code}).
   */
  void (*on_error)(otc_session* session,
                   void* user_data,
                   const char* error_string,
                   enum otc_session_error_code error);

  /**
      A pointer to data you set related to the session.
   */
  void* user_data;

  /**
      A void pointer to a memory area holding reserved resources used for the
      internal implementation.
   */
  void* reserved;
};

/**
  Creates a new {@link otc_session} struct instance. Pass this into the
  {@link otc_session_connect} function to connect to the OpenTok session.

  See also {@link otc_session_new_with_settings} for initializing a
  otc_session struct with advanced options.

  @param apikey The API key for the OpenTok project. See
         <a href="https://tokbox.com/account">See the your TokBox account
  page</a>.

   @param session_id The session ID.

   @param callbacks The otc_session_callbacks structure containing
          callback functions for events related to the session.

   @return The otc_session struct, representing the OpenTok session.
 */
OTC_DECL(otc_session*)
otc_session_new(const char* apikey,
                const char* session_id,
                const struct otc_session_callbacks* callbacks);

/**
  Creates a new {@link otc_session_settings} instance, used to set up advanced
  session settings. Call the following functions to configure these settings
  before calling the {@link otc_session_new_with_settings} function to create
  a session with these settings:

  <ul>
    <li> {@link otc_session_settings_set_connection_events_suppressed} &mdash;
         Suppress connection events.
    </li>
    <li> {@link otc_session_settings_set_custom_ice_config} &mdash
         Use custom TURN servers.
    </li>
    <li> {@link otc_session_settings_set_proxy_url} &mdash;
         Configure the IP proxy feature.
    </li>
    <li> {@link otc_session_settings_set_ip_whitelist} &mdash;
         Enable the IP whitelist feature.
    </li>
  </ul>
 */
OTC_DECL(otc_session_settings*) otc_session_settings_new();

/**
  Prevent connection events, to support large interactive video sessions.
  This prevents {@link otc_session_callbacks.on_connection_created} and
  {@link otc_session_callbacks.on_connection_dropped}
  callback functions from being invoked when other clients connect to or
  disconnect from the session. (Also, the OpenTok server does not send these
  events to the client.) For more information, see
  <a
  href="https://tokbox.com/developer/guides/broadcast/live-interactive-video/#suppressing-connection-events">
  Suppressing connection events</a> in the OpenTok developer guides.
 */
OTC_DECL(otc_status)
otc_session_settings_set_connection_events_suppressed(
    otc_session_settings* settings,
    otc_bool suppress);

/**
  Enables custom ICE sever configuration. This is part of the
  <a href="https://tokbox.com/developer/guides/configurable-turn-servers/">
  configurable TURN feature</a>.
 */
OTC_DECL(otc_status)
otc_session_settings_set_custom_ice_config(
    otc_session_settings* settings,
    const struct otc_custom_ice_config* custom_ice_config);

/**
  Sets the IP proxy URL. See the <a
  href="https://tokbox.com/developer/guides/ip-proxy/"> IP Proxy developer
  guide</a>.
 */
OTC_DECL(otc_status)
otc_session_settings_set_proxy_url(otc_session_settings* settings,
                                   const char* proxy_url);

/**
  Pass in OTC_TRUE to have the client use the IP address white list.
  <a href="https://tokbox.com/developer/enterprise/content/ip-addresses/">This
  feature</a> is available as an add-on feature for Enterprise partners.
 */
OTC_DECL(otc_status)
otc_session_settings_set_ip_whitelist(otc_session_settings* settings,
                                      otc_bool ip_whitelist);

/**
  Deletes an {@link otc_session_settings} instance.
 */
OTC_DECL(otc_status)
otc_session_settings_delete(otc_session_settings* settings);

/**
 Creates a new OpenTok session with advanced settings. These include settings
 for suppressing connection events, custom TURN servers, the IP proxy feature,
 and the IP whitelist feature.

 @param apikey The API key for the OpenTok project. See
        <a href="https://tokbox.com/account">See the your TokBox account
 page</a>.

  @param session_id The session ID.

  @param callbacks The {@link otc_session_callbacks} structure containing
         callback functions for events related to the session.

  @param settings The {@link otc_session_settings} struct With
         advanced settings for the session.

  @return The {@link otc_session struct}, representing the OpenTok session.

  @see otc_session_new.
 */
OTC_DECL(otc_session*)
otc_session_new_with_settings(const char* apikey,
                              const char* session_id,
                              const struct otc_session_callbacks* callbacks,
                              otc_session_settings* settings);

/**
    Releases resources associated with the session.

    @param session The {@link otc_session} instance.
 */
OTC_DECL(otc_status) otc_session_delete(otc_session* session);

/**
  Connects the client to an OpenTok session.

  @param session The {@link otc_session} struct representing
                 the OpenTok session to connect to.

  @param token The client token for connecting to the session.
               See <a href="https://tokbox.com/developer/guides/create-token/">
               Token Creation Overview</a>.

  @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_session_connect(otc_session* session, const char* token);

/**
 Disconnects the client from this session. All of the client's subscribers
 and publishers will also be will be disconnected from the session.

 @param session The {@link otc_session} struct representing
                the OpenTok session to disconnect from.

 @return Return value indicating either error or success.
 */
OTC_DECL(otc_status) otc_session_disconnect(otc_session* session);

/**
  Starts a publisher streaming to the session.

  @param session The {@link otc_session} struct representing
                 the OpenTok session to publish to.

  @param publisher The {@link otc_publisher} struct representing
                   the publisher.

  @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_session_publish(otc_session* session, otc_publisher* publisher);

/**
  Causes a publisher from a session (causing its stream to stop).

  @param session The {@link otc_session} struct representing
                 the OpenTok session to unpublish from.

  @param publisher The {@link otc_publisher} struct representing
                   the publisher.

  @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_session_unpublish(otc_session* session, otc_publisher* publisher);

/**
 Starts subscribing to (receiving a stream for) a subscriber's
 audio-video stream in this session.

 @param session The {@link otc_session} struct representing
                the OpenTok session containing the subscriber's stream.

 @param subscriber The {@link otc_subscriber} struct representing
                  the subscriber.

 @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_session_subscribe(otc_session* session, otc_subscriber* subscriber);

/**
 Stops subscribing to (receiving a stream for) a specified subscriber
 in the session.

 @param session The {@link otc_session} struct representing
                the OpenTok session containing the subscriber's stream.

 @param subscriber The {@link otc_subscriber} struct representing
                  the subscriber.

 @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_session_unsubscribe(otc_session* session, otc_subscriber* subscriber);

/**
  Sends a signal to all clients connected to the session.
  See {@link otc_session_send_signal_with_options},
  {@link otc_session_send_signal_to_connection}, and
  {@link otc_session_send_signal_to_connection_with_options}.
  Also see the <a href="https://tokbox.com/developer/guides/signaling/">
  Signaling overview</a>.

  @param session The {@link otc_session} struct representing
                 the OpenTok session.

  @param type The optional type string value for the signal.

  @param signal The optional data string value for the signal.

  @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_session_send_signal(otc_session* session,
                        const char* type,
                        const char* signal);

/**
  Sends a signal to a specific client connected to the session.
  See {@link otc_session_send_signal_to_connection_with_options},
  {@link otc_session_send_signal}, and {@link
  otc_session_send_signal_with_options}. Also see the <a
  href="https://tokbox.com/developer/guides/signaling/"> Signaling overview</a>.

  @param session The {@link otc_session} struct representing
                 the OpenTok session.

  @param type The optional type string value for the signal.

  @param signal The optional data string value for the signal.

  @param connection The {@link otc_connection} struct corresponding to
         the client to receive the signal.

  @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_session_send_signal_to_connection(otc_session* session,
                                      const char* type,
                                      const char* signal,
                                      const otc_connection* connection);

/**
  Sends a signal to all clients connected to the session, with specified
  options. See {@link otc_session_send_signal}, {@link
  otc_session_send_signal_to_connection}, and {@link
  otc_session_send_signal_to_connection_with_options}. Also see the <a
  href="https://tokbox.com/developer/guides/signaling/"> Signaling overview</a>.

  @param session The {@link otc_session} struct representing
                 the OpenTok session.

  @param type The optional type string value for the signal.

  @param signal The optional data string value for the signal.

  @param signal_options The {@link otc_signal_options} struct with options
         for the signal.

  @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_session_send_signal_with_options(otc_session* session,
                                     const char* type,
                                     const char* signal,
                                     struct otc_signal_options signal_options);

/**
  Sends a signal to a specific client, with specified options.
  See {@link otc_session_send_signal}, {@link
  otc_session_send_signal_to_connection}, and {@link
  otc_session_send_signal_with_options}. Also see the <a
  href="https://tokbox.com/developer/guides/signaling/"> Signaling overview</a>.

  @param session The {@link otc_session} struct representing
                 the OpenTok session.

  @param type The optional type string value for the signal.

  @param signal The optional data string value for the signal.

  @param connection The {@link otc_connection} struct corresponding to
         the client to receive the signal.

  @param signal_options The {@link otc_signal_options} struct with options
         for the signal.

  @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_session_send_signal_to_connection_with_options(
    otc_session* session,
    const char* type,
    const char* signal,
    const otc_connection* connection,
    struct otc_signal_options signal_options);

/**
 Report that your app experienced an issue. You can use the issue ID with
 the <a href="https://tokbox.com/developer/tools/Inspector">Inspector</a> tool
 or when discussing an issue with the Vonage Video API support team.

  @param session The {@link otc_session} struct representing
                 the OpenTok session.

  @param issue_description A description string.

  @param issue_id A pointer to a string that will be set the unique identifier
         for the reported issue.

  @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_session_report_issue(struct otc_session* session,
                         const char* issue_description,
                         char** issue_id);

/**
    Gets the unique session ID for this session.
 */
OTC_DECL(char*) otc_session_get_id(const otc_session* session);

/**
    Gets the connection object associated with this session.

    @param session The {@link otc_session} struct representing
                   the OpenTok session.

    @return The {@link otc_connection} instance.
 */
OTC_DECL(otc_connection*)
otc_session_get_connection(const otc_session* session);

/**
  Gets the capabilities of the client connecxted to the session.
  All {@link otc_session_capabilities} members are undefined until
  {@link otc_session_callbacks.on_connected} has been called.

  @param session The {@link otc_session} struct representing
                 the OpenTok session.

  @return The {@link otc_session_capabilities} instance.
 */
OTC_DECL(struct otc_session_capabilities)
otc_session_get_capabilities(const otc_session* session);

/**
  Gets the user data associated with the session. See
  {@link otc_session_callbacks.user_data}.

  @param session The {@link otc_session} struct representing
                 the OpenTok session.

  @return A pointer to the user data.
 */
OTC_DECL(void*) otc_session_get_user_data(const otc_session* session);

OTC_END_DECL

#endif  // SESSION_H
