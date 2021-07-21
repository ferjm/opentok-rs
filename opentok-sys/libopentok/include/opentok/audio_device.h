/** @file audio_device.h
    @brief Audio device.

    This file includes the type definition for an audio device, along with
    structures and functions you use with it.

    Use an audio device to define a custom audio capturer for all publishers
    and a custom audio renderer for all publishers and subscribers.
    If you do not use a custom audio device, the application will use the
    default audio device using the system microphone and speaker.

    A custom audio is used for any publisher and subscriber in any session
    the client connects to. Once the audio device is set it acts globally.
    The capturing and rendering bits in the audio device being set are
    used for rendering and capturing audio for all participants in the session.
    You cannot set an audio device for a given publisher or subscriber alone.

    Instantiate an otc_audio_device_callbacks structure, setting callback
    functions for events related to the audio device.
    Then call otc_set_audio_device(const struct otc_audio_device_callbacks
   *callbacks) to associate the callbacks with the audio device to be used. You
   must call this function before you connect to a session. Additionally, this
   is a global operation that must persist throughout the lifetime of the
   session.

    Call the {@link otc_audio_device_read_render_data} function to retrieve
    unrendered audio samples.

    Call the {@link otc_audio_device_write_capture_data} function to write
    audio samples that will be included in streams you publish.
*/
#ifndef AUDIO_DEVICE_H
#define AUDIO_DEVICE_H

#include <stdlib.h>

#include "config.h"
#include "base.h"

OTC_BEGIN_DECL

/**
  Audio device type definition.
 */
typedef struct otc_audio_device otc_audio_device;

/**
    This structure represents the settings associated with an audio device.
 */
struct otc_audio_device_settings {
  int sampling_rate; /**< The sample rate for the audio device, in samples per
                        second. */
  int number_of_channels; /**< The number of audio channels in the device. */
};

/**
    This structure includes a set of function pointers to callback functions to
   use with an audio device. The SDK calls these functions when events related
    to the audio device occur.

    All callbacks will not be made on the application or main thread but on an
    internal thread. The application should return the callback as quickly as
    possible to avoid blocking the internal thread.

    In addition to the callbacks, the struct includes a user_data pointer,
    which points data you can set related to the audio device.
 */
struct otc_audio_device_callbacks {
  /**
      Called when the SDK requests the audio device to initialize itself.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*init)(const otc_audio_device* audio_device, void* user_data);

  /**
      Called when the SDK requests the audio device to be destroyed.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*destroy)(const otc_audio_device* audio_device, void* user_data);

  /**
      Called when the SDK requests the audio capturer for the audio device to be
     initialized.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*init_capturer)(const otc_audio_device* audio_device,
                            void* user_data);

  /**
      Called when the SDK is done capturing audio and it is time to destroy
      the audio capturer for the audio device.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*destroy_capturer)(const otc_audio_device* audio_device,
                               void* user_data);

  /**
      Called when the SDK requests the audio device to start capturing audio
     samples.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*start_capturer)(const otc_audio_device* audio_device,
                             void* user_data);

  /**
      Called when the SDK requests the audio capturer for the audio device
      to stop sampling audio.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*stop_capturer)(const otc_audio_device* audio_device,
                            void* user_data);

  /**
      Whether the device has initialized itself for audio sampling.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating whether the device is initialized or not.
      @see otc_bool
   */
  otc_bool (*is_capturer_initialized)(const otc_audio_device* audio_device,
                                      void* user_data);

  /**
      Whether the device has started audio sampling.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating whether it is started or not.
      @see otc_bool
   */
  otc_bool (*is_capturer_started)(const otc_audio_device* audio_device,
                                  void* user_data);

  /**
      Called when the SDK requests the estimated capturing delay for the audio
     device, in ms.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return The estimated capturing delay in ms.
   */
  int (*get_estimated_capture_delay)(const otc_audio_device* audio_device,
                                     void* user_data);

  /**
      Called when the SDK requests the audio capture settings used by the audio
     device.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @param settings A pointer to a struct holding the settings.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*get_capture_settings)(const otc_audio_device* audio_device,
                                   void* user_data,
                                   struct otc_audio_device_settings* settings);

  /**
      Called when the SDK requests the audio device to initialize itself for
      audio rendering.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*init_renderer)(const otc_audio_device* audio_device,
                            void* user_data);

  /**
      Called when the SDK is done rendering audio and it is time to destroy
      the audio renderer for the audio device.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*destroy_renderer)(const otc_audio_device* audio_device,
                               void* user_data);

  /**
      Called when the SDK requests the audio device to start rendering audio.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*start_renderer)(const otc_audio_device* audio_device,
                             void* user_data);

  /**
      Called when the SDK requests the audio device to stop rendering audio.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*stop_renderer)(const otc_audio_device* audio_device,
                            void* user_data);

  /**
      Whether the device has been initialized for audio rendering.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating whether it is initialized or not.
      @see otc_bool
   */
  otc_bool (*is_renderer_initialized)(const otc_audio_device* audio_device,
                                      void* user_data);

  /**
      Whether the device started audio rendering.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return Boolean value indicating whether it is started or not.
      @see otc_bool
   */
  otc_bool (*is_renderer_started)(const otc_audio_device* audio_device,
                                  void* user_data);

  /**
      Called when the SDK requests the estimated rendering delay for the audio
     device, in ms.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @return  The estimated rendering delay in ms.
   */
  int (*get_estimated_render_delay)(const otc_audio_device* audio_device,
                                    void* user_data);

  /**
      Called when the SDK requests the audio rendering settings used by the
     device.

      @param audio_device A pointer to the audio device instance.
      @param user_data A pointer to the user_data you set for the audio device.
      @param settings A pointer to a struct holding the settings.
      @return Boolean value indicating either error or success.
      @see otc_bool
   */
  otc_bool (*get_render_settings)(const otc_audio_device* audio_device,
                                  void* user_data,
                                  struct otc_audio_device_settings* settings);
  /**
      A pointer to data you set related to the audio device.
  */
  void* user_data;

  /**
      A void pointer to a memory area holding reserved resources used for the
      internal implementation.
   */
  void* reserved;
};

/**
    Retrieves unrendered audio samples from the session. These samples are
    mixed from the streams in the session you have subscribed to.

    @param buffer The buffer containing audio data.
    @param number_of_samples The number of samples requested.
    @return The number of samples copied out of the audio buffer.
 */
OTC_DECL(size_t)
otc_audio_device_read_render_data(int16_t* buffer, size_t number_of_samples);

/**
    Passes in audio data from the audio device to transmit to a session.
    This audio data is used by streams you publish to the session.

    @param buffer The buffer containing audio data.
    @param number_of_samples The number of samples available for copying.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_audio_device_write_capture_data(const int16_t* buffer,
                                    size_t number_of_samples);

/**
    Sets the audio device to be used. You must call this function before you
    connect to a session. Additionally, this is a global operation that must
    persist throughout the lifetime of a session.

    @param callbacks A pointer to a audio device function callback struct.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status)
otc_set_audio_device(const struct otc_audio_device_callbacks* callbacks);

OTC_END_DECL

#endif  // AUDIO_DEVICE_H
