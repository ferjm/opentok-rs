(function() {var implementors = {};
implementors["opentok"] = [{"text":"impl !Freeze for <a class=\"struct\" href=\"opentok/connection/struct.Connection.html\" title=\"struct opentok::connection::Connection\">Connection</a>","synthetic":true,"types":["opentok::connection::Connection"]},{"text":"impl Freeze for <a class=\"enum\" href=\"opentok/audio_device/enum.AudioDeviceError.html\" title=\"enum opentok::audio_device::AudioDeviceError\">AudioDeviceError</a>","synthetic":true,"types":["opentok::audio_device::AudioDeviceError"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/audio_device/struct.AudioDeviceCallbacks.html\" title=\"struct opentok::audio_device::AudioDeviceCallbacks\">AudioDeviceCallbacks</a>","synthetic":true,"types":["opentok::audio_device::AudioDeviceCallbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/audio_device/struct.AudioDeviceCallbacksBuilder.html\" title=\"struct opentok::audio_device::AudioDeviceCallbacksBuilder\">AudioDeviceCallbacksBuilder</a>","synthetic":true,"types":["opentok::audio_device::AudioDeviceCallbacksBuilder"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/audio_device/struct.AudioSampleData.html\" title=\"struct opentok::audio_device::AudioSampleData\">AudioSampleData</a>","synthetic":true,"types":["opentok::audio_device::AudioSampleData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/audio_device/struct.AudioSample.html\" title=\"struct opentok::audio_device::AudioSample\">AudioSample</a>","synthetic":true,"types":["opentok::audio_device::AudioSample"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/audio_device/struct.AudioDeviceSettings.html\" title=\"struct opentok::audio_device::AudioDeviceSettings\">AudioDeviceSettings</a>","synthetic":true,"types":["opentok::audio_device::AudioDeviceSettings"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/audio_device/struct.AudioDevice.html\" title=\"struct opentok::audio_device::AudioDevice\">AudioDevice</a>","synthetic":true,"types":["opentok::audio_device::AudioDevice"]},{"text":"impl Freeze for <a class=\"enum\" href=\"opentok/enum.OtcError.html\" title=\"enum opentok::OtcError\">OtcError</a>","synthetic":true,"types":["opentok::enums::OtcError"]},{"text":"impl Freeze for <a class=\"enum\" href=\"opentok/log/enum.LogLevel.html\" title=\"enum opentok::log::LogLevel\">LogLevel</a>","synthetic":true,"types":["opentok::log::LogLevel"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/log/struct.LOGGER_CALLBACKS.html\" title=\"struct opentok::log::LOGGER_CALLBACKS\">LOGGER_CALLBACKS</a>","synthetic":true,"types":["opentok::log::LOGGER_CALLBACKS"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/publisher/struct.INSTANCES.html\" title=\"struct opentok::publisher::INSTANCES\">INSTANCES</a>","synthetic":true,"types":["opentok::publisher::INSTANCES"]},{"text":"impl Freeze for <a class=\"enum\" href=\"opentok/publisher/enum.PublisherError.html\" title=\"enum opentok::publisher::PublisherError\">PublisherError</a>","synthetic":true,"types":["opentok::publisher::PublisherError"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/publisher/struct.PublisherCallbacks.html\" title=\"struct opentok::publisher::PublisherCallbacks\">PublisherCallbacks</a>","synthetic":true,"types":["opentok::publisher::PublisherCallbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/publisher/struct.PublisherCallbacksBuilder.html\" title=\"struct opentok::publisher::PublisherCallbacksBuilder\">PublisherCallbacksBuilder</a>","synthetic":true,"types":["opentok::publisher::PublisherCallbacksBuilder"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/publisher/struct.Publisher.html\" title=\"struct opentok::publisher::Publisher\">Publisher</a>","synthetic":true,"types":["opentok::publisher::Publisher"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/session/struct.INSTANCES.html\" title=\"struct opentok::session::INSTANCES\">INSTANCES</a>","synthetic":true,"types":["opentok::session::INSTANCES"]},{"text":"impl Freeze for <a class=\"enum\" href=\"opentok/session/enum.SessionError.html\" title=\"enum opentok::session::SessionError\">SessionError</a>","synthetic":true,"types":["opentok::session::SessionError"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/session/struct.SessionCallbacks.html\" title=\"struct opentok::session::SessionCallbacks\">SessionCallbacks</a>","synthetic":true,"types":["opentok::session::SessionCallbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/session/struct.SessionCallbacksBuilder.html\" title=\"struct opentok::session::SessionCallbacksBuilder\">SessionCallbacksBuilder</a>","synthetic":true,"types":["opentok::session::SessionCallbacksBuilder"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/session/struct.Session.html\" title=\"struct opentok::session::Session\">Session</a>","synthetic":true,"types":["opentok::session::Session"]},{"text":"impl Freeze for <a class=\"enum\" href=\"opentok/stream/enum.StreamVideoType.html\" title=\"enum opentok::stream::StreamVideoType\">StreamVideoType</a>","synthetic":true,"types":["opentok::stream::StreamVideoType"]},{"text":"impl !Freeze for <a class=\"struct\" href=\"opentok/stream/struct.Stream.html\" title=\"struct opentok::stream::Stream\">Stream</a>","synthetic":true,"types":["opentok::stream::Stream"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/subscriber/struct.INSTANCES.html\" title=\"struct opentok::subscriber::INSTANCES\">INSTANCES</a>","synthetic":true,"types":["opentok::subscriber::INSTANCES"]},{"text":"impl Freeze for <a class=\"enum\" href=\"opentok/subscriber/enum.SubscriberError.html\" title=\"enum opentok::subscriber::SubscriberError\">SubscriberError</a>","synthetic":true,"types":["opentok::subscriber::SubscriberError"]},{"text":"impl Freeze for <a class=\"enum\" href=\"opentok/subscriber/enum.VideoReason.html\" title=\"enum opentok::subscriber::VideoReason\">VideoReason</a>","synthetic":true,"types":["opentok::subscriber::VideoReason"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/subscriber/struct.SubscriberCallbacks.html\" title=\"struct opentok::subscriber::SubscriberCallbacks\">SubscriberCallbacks</a>","synthetic":true,"types":["opentok::subscriber::SubscriberCallbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/subscriber/struct.SubscriberCallbacksBuilder.html\" title=\"struct opentok::subscriber::SubscriberCallbacksBuilder\">SubscriberCallbacksBuilder</a>","synthetic":true,"types":["opentok::subscriber::SubscriberCallbacksBuilder"]},{"text":"impl !Freeze for <a class=\"struct\" href=\"opentok/subscriber/struct.Subscriber.html\" title=\"struct opentok::subscriber::Subscriber\">Subscriber</a>","synthetic":true,"types":["opentok::subscriber::Subscriber"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/video_capturer/struct.VideoCapturerSettings.html\" title=\"struct opentok::video_capturer::VideoCapturerSettings\">VideoCapturerSettings</a>","synthetic":true,"types":["opentok::video_capturer::VideoCapturerSettings"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/video_capturer/struct.VideoCapturerCallbacks.html\" title=\"struct opentok::video_capturer::VideoCapturerCallbacks\">VideoCapturerCallbacks</a>","synthetic":true,"types":["opentok::video_capturer::VideoCapturerCallbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/video_capturer/struct.VideoCapturerCallbacksBuilder.html\" title=\"struct opentok::video_capturer::VideoCapturerCallbacksBuilder\">VideoCapturerCallbacksBuilder</a>","synthetic":true,"types":["opentok::video_capturer::VideoCapturerCallbacksBuilder"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok/video_capturer/struct.VideoCapturer.html\" title=\"struct opentok::video_capturer::VideoCapturer\">VideoCapturer</a>","synthetic":true,"types":["opentok::video_capturer::VideoCapturer"]},{"text":"impl Freeze for <a class=\"enum\" href=\"opentok/video_frame/enum.FrameFormat.html\" title=\"enum opentok::video_frame::FrameFormat\">FrameFormat</a>","synthetic":true,"types":["opentok::video_frame::FrameFormat"]},{"text":"impl Freeze for <a class=\"enum\" href=\"opentok/video_frame/enum.FramePlane.html\" title=\"enum opentok::video_frame::FramePlane\">FramePlane</a>","synthetic":true,"types":["opentok::video_frame::FramePlane"]},{"text":"impl !Freeze for <a class=\"struct\" href=\"opentok/video_frame/struct.VideoFrame.html\" title=\"struct opentok::video_frame::VideoFrame\">VideoFrame</a>","synthetic":true,"types":["opentok::video_frame::VideoFrame"]}];
implementors["opentok_rs_sys"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_video_frame.html\" title=\"struct opentok_rs_sys::otc_video_frame\">otc_video_frame</a>","synthetic":true,"types":["opentok_rs_sys::otc_video_frame"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_video_frame_planar_memory_callbacks.html\" title=\"struct opentok_rs_sys::otc_video_frame_planar_memory_callbacks\">otc_video_frame_planar_memory_callbacks</a>","synthetic":true,"types":["opentok_rs_sys::otc_video_frame_planar_memory_callbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_video_capturer.html\" title=\"struct opentok_rs_sys::otc_video_capturer\">otc_video_capturer</a>","synthetic":true,"types":["opentok_rs_sys::otc_video_capturer"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_video_capturer_settings.html\" title=\"struct opentok_rs_sys::otc_video_capturer_settings\">otc_video_capturer_settings</a>","synthetic":true,"types":["opentok_rs_sys::otc_video_capturer_settings"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_video_capturer_callbacks.html\" title=\"struct opentok_rs_sys::otc_video_capturer_callbacks\">otc_video_capturer_callbacks</a>","synthetic":true,"types":["opentok_rs_sys::otc_video_capturer_callbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_audio_device.html\" title=\"struct opentok_rs_sys::otc_audio_device\">otc_audio_device</a>","synthetic":true,"types":["opentok_rs_sys::otc_audio_device"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_audio_device_settings.html\" title=\"struct opentok_rs_sys::otc_audio_device_settings\">otc_audio_device_settings</a>","synthetic":true,"types":["opentok_rs_sys::otc_audio_device_settings"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_audio_device_callbacks.html\" title=\"struct opentok_rs_sys::otc_audio_device_callbacks\">otc_audio_device_callbacks</a>","synthetic":true,"types":["opentok_rs_sys::otc_audio_device_callbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_connection.html\" title=\"struct opentok_rs_sys::otc_connection\">otc_connection</a>","synthetic":true,"types":["opentok_rs_sys::otc_connection"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_stream.html\" title=\"struct opentok_rs_sys::otc_stream\">otc_stream</a>","synthetic":true,"types":["opentok_rs_sys::otc_stream"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_session.html\" title=\"struct opentok_rs_sys::otc_session\">otc_session</a>","synthetic":true,"types":["opentok_rs_sys::otc_session"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_publisher.html\" title=\"struct opentok_rs_sys::otc_publisher\">otc_publisher</a>","synthetic":true,"types":["opentok_rs_sys::otc_publisher"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_publisher_settings.html\" title=\"struct opentok_rs_sys::otc_publisher_settings\">otc_publisher_settings</a>","synthetic":true,"types":["opentok_rs_sys::otc_publisher_settings"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_publisher_audio_stats.html\" title=\"struct opentok_rs_sys::otc_publisher_audio_stats\">otc_publisher_audio_stats</a>","synthetic":true,"types":["opentok_rs_sys::otc_publisher_audio_stats"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_publisher_video_stats.html\" title=\"struct opentok_rs_sys::otc_publisher_video_stats\">otc_publisher_video_stats</a>","synthetic":true,"types":["opentok_rs_sys::otc_publisher_video_stats"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_publisher_callbacks.html\" title=\"struct opentok_rs_sys::otc_publisher_callbacks\">otc_publisher_callbacks</a>","synthetic":true,"types":["opentok_rs_sys::otc_publisher_callbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_publisher_rtc_stats.html\" title=\"struct opentok_rs_sys::otc_publisher_rtc_stats\">otc_publisher_rtc_stats</a>","synthetic":true,"types":["opentok_rs_sys::otc_publisher_rtc_stats"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_publisher_rtc_stats_report_cb.html\" title=\"struct opentok_rs_sys::otc_publisher_rtc_stats_report_cb\">otc_publisher_rtc_stats_report_cb</a>","synthetic":true,"types":["opentok_rs_sys::otc_publisher_rtc_stats_report_cb"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_subscriber.html\" title=\"struct opentok_rs_sys::otc_subscriber\">otc_subscriber</a>","synthetic":true,"types":["opentok_rs_sys::otc_subscriber"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_subscriber_audio_stats.html\" title=\"struct opentok_rs_sys::otc_subscriber_audio_stats\">otc_subscriber_audio_stats</a>","synthetic":true,"types":["opentok_rs_sys::otc_subscriber_audio_stats"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_subscriber_video_stats.html\" title=\"struct opentok_rs_sys::otc_subscriber_video_stats\">otc_subscriber_video_stats</a>","synthetic":true,"types":["opentok_rs_sys::otc_subscriber_video_stats"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_subscriber_rtc_stats_report_cb.html\" title=\"struct opentok_rs_sys::otc_subscriber_rtc_stats_report_cb\">otc_subscriber_rtc_stats_report_cb</a>","synthetic":true,"types":["opentok_rs_sys::otc_subscriber_rtc_stats_report_cb"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_subscriber_callbacks.html\" title=\"struct opentok_rs_sys::otc_subscriber_callbacks\">otc_subscriber_callbacks</a>","synthetic":true,"types":["opentok_rs_sys::otc_subscriber_callbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_session_settings.html\" title=\"struct opentok_rs_sys::otc_session_settings\">otc_session_settings</a>","synthetic":true,"types":["opentok_rs_sys::otc_session_settings"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_session_capabilities.html\" title=\"struct opentok_rs_sys::otc_session_capabilities\">otc_session_capabilities</a>","synthetic":true,"types":["opentok_rs_sys::otc_session_capabilities"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_custom_ice_config.html\" title=\"struct opentok_rs_sys::otc_custom_ice_config\">otc_custom_ice_config</a>","synthetic":true,"types":["opentok_rs_sys::otc_custom_ice_config"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_signal_options.html\" title=\"struct opentok_rs_sys::otc_signal_options\">otc_signal_options</a>","synthetic":true,"types":["opentok_rs_sys::otc_signal_options"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_session_callbacks.html\" title=\"struct opentok_rs_sys::otc_session_callbacks\">otc_session_callbacks</a>","synthetic":true,"types":["opentok_rs_sys::otc_session_callbacks"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_rs_sys/struct.otc_media_utils_codecs.html\" title=\"struct opentok_rs_sys::otc_media_utils_codecs\">otc_media_utils_codecs</a>","synthetic":true,"types":["opentok_rs_sys::otc_media_utils_codecs"]}];
implementors["opentok_utils"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_utils/capturer/struct.CapturerBuffer.html\" title=\"struct opentok_utils::capturer::CapturerBuffer\">CapturerBuffer</a>","synthetic":true,"types":["opentok_utils::capturer::CapturerBuffer"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_utils/capturer/struct.Capturer.html\" title=\"struct opentok_utils::capturer::Capturer\">Capturer</a>","synthetic":true,"types":["opentok_utils::capturer::Capturer"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_utils/capturer/struct.AudioCapturer.html\" title=\"struct opentok_utils::capturer::AudioCapturer\">AudioCapturer</a>","synthetic":true,"types":["opentok_utils::capturer::AudioCapturer"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_utils/common/struct.MissingElement.html\" title=\"struct opentok_utils::common::MissingElement\">MissingElement</a>","synthetic":true,"types":["opentok_utils::common::MissingElement"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_utils/common/struct.Credentials.html\" title=\"struct opentok_utils::common::Credentials\">Credentials</a>","synthetic":true,"types":["opentok_utils::common::Credentials"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_utils/publisher/struct.Publisher.html\" title=\"struct opentok_utils::publisher::Publisher\">Publisher</a>","synthetic":true,"types":["opentok_utils::publisher::Publisher"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_utils/renderer/struct.Renderer.html\" title=\"struct opentok_utils::renderer::Renderer\">Renderer</a>","synthetic":true,"types":["opentok_utils::renderer::Renderer"]},{"text":"impl Freeze for <a class=\"struct\" href=\"opentok_utils/subscriber/struct.Subscriber.html\" title=\"struct opentok_utils::subscriber::Subscriber\">Subscriber</a>","synthetic":true,"types":["opentok_utils::subscriber::Subscriber"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()