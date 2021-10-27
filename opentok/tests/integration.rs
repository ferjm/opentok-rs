#[cfg(test)]
mod tests {
    use futures::executor::LocalPool;
    use opentok::audio_device::{AudioDevice, AudioDeviceSettings};
    use opentok::log::{self, LogLevel};
    use opentok::publisher::{Publisher, PublisherCallbacks};
    use opentok::session::{Session, SessionCallbacks};
    use opentok::subscriber::{Subscriber, SubscriberCallbacks};
    use opentok::video_capturer::{VideoCapturer, VideoCapturerCallbacks, VideoCapturerSettings};
    use opentok::video_frame::VideoFrame;
    use opentok_server::{OpenTok, SessionOptions, TokenRole};
    use opentok_utils::capturer;
    use opentok_utils::common::Credentials;
    use opentok_utils::publisher::Publisher as UtilsPublisher;
    use std::env;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};

    fn setup_test() -> (String, String, String) {
        opentok::init().unwrap();
        let api_key = env::var("OPENTOK_KEY").unwrap();
        let api_secret = env::var("OPENTOK_SECRET").unwrap();
        let opentok = OpenTok::new(api_key.clone(), api_secret);
        let mut pool = LocalPool::new();
        let session_id = pool
            .run_until(opentok.create_session(SessionOptions::default()))
            .unwrap();
        assert!(!session_id.is_empty());
        let token = opentok.generate_token(&session_id, TokenRole::Publisher);
        (api_key, session_id, token)
    }

    fn test_teardown() {
        opentok::deinit().unwrap();
    }

    #[test]
    fn test_logger_callback() {
        opentok::init().unwrap();

        log::enable_log(LogLevel::All);

        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));

        log::logger_callback(Box::new(move |_| {
            if let Ok(sender) = sender.try_lock() {
                let _ = sender.send(());
            }
        }));

        receiver.recv().unwrap();

        opentok::deinit().unwrap();
    }

    #[test]
    fn test_session_connection() {
        let (api_key, session_id, token) = setup_test();

        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let session_id_ = session_id.clone();
        let on_connected_received = Arc::new(AtomicBool::new(false));
        let on_connected_received_ = on_connected_received.clone();
        let session_callbacks = SessionCallbacks::builder()
            .on_connected(move |session| {
                assert_eq!(session.id(), session_id_);
                on_connected_received.store(true, Ordering::Relaxed);
                session.disconnect().unwrap();
            })
            .on_disconnected(move |_| {
                assert!(on_connected_received_.load(Ordering::Relaxed));
                sender.lock().unwrap().send(()).unwrap();
            })
            .on_error(|_, error, _| {
                panic!("{:?}", error);
            })
            .build();

        let session = Session::new(&api_key, &session_id, session_callbacks).unwrap();

        session.connect(&token).unwrap();

        receiver.recv().unwrap();

        test_teardown();
    }

    #[test]
    fn test_session_connection_invalid_api_key() {
        let (_, session_id, token) = setup_test();

        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let session_callbacks = SessionCallbacks::builder()
            .on_connected(|_| {
                panic!("Unexpected on_connected callback");
            })
            .on_error(move |_, _, _| {
                sender.lock().unwrap().send(()).unwrap();
            })
            .build();

        let session = Session::new("banana", &session_id, session_callbacks).unwrap();

        session.connect(&token).unwrap();

        receiver.recv().unwrap();

        test_teardown();
    }

    #[test]
    fn test_session_connection_invalid_token() {
        let (api_key, session_id, _) = setup_test();

        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let session_callbacks = SessionCallbacks::builder()
            .on_connected(|_| {
                panic!("Unexpected on_connected callback");
            })
            .on_error(move |_, _, _| {
                sender.lock().unwrap().send(()).unwrap();
            })
            .build();

        let session = Session::new(&api_key, &session_id, session_callbacks).unwrap();

        session.connect("banana").unwrap();

        receiver.recv().unwrap();

        test_teardown();
    }

    #[test]
    fn test_session_connection_invalid_session_id() {
        let (api_key, _, token) = setup_test();

        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let session_callbacks = SessionCallbacks::builder()
            .on_connected(|_| {
                panic!("Unexpected on_connected callback");
            })
            .on_error(move |_, _, _| {
                sender.lock().unwrap().send(()).unwrap();
            })
            .build();

        let session = Session::new(&api_key, "banana", session_callbacks).unwrap();

        session.connect(&token).unwrap();

        receiver.recv().unwrap();

        test_teardown();
    }

    #[test]
    fn test_publisher() {
        let (api_key, session_id, token) = setup_test();

        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));

        let publisher_callbacks = PublisherCallbacks::builder()
            .on_stream_created(move |_, _| {
                sender.lock().unwrap().send(()).unwrap();
            })
            .on_error(|_, error, _| {
                println!("on_error {:?}", error);
            })
            .build();

        let audio_device = AudioDevice::get_instance();
        let audio_capture_thread_running = Arc::new(AtomicBool::new(true));
        let audio_capture_thread_running_ = audio_capture_thread_running.clone();
        std::thread::spawn(move || {
            let audio_capturer =
                capturer::AudioCapturer::new(&AudioDeviceSettings::default()).unwrap();
            loop {
                if !audio_capture_thread_running.load(Ordering::Relaxed) {
                    break;
                }
                if let Some(sample) = audio_capturer.pull_buffer() {
                    audio_device.lock().unwrap().push_audio_sample(sample);
                }
                std::thread::sleep(std::time::Duration::from_micros(10000));
            }
        });

        let render_thread_running = Arc::new(AtomicBool::new(false));
        let render_thread_running_ = render_thread_running.clone();
        let render_thread_running__ = render_thread_running.clone();
        let video_capturer_callbacks = VideoCapturerCallbacks::builder()
            .start(move |video_capturer| {
                let video_capturer = video_capturer.clone();
                render_thread_running.store(true, Ordering::Relaxed);
                let render_thread_running_ = render_thread_running.clone();
                std::thread::spawn(move || {
                    let settings = VideoCapturerSettings::default();
                    let capturer = capturer::Capturer::new(&settings).unwrap();
                    let mut buf: Vec<u8> = vec![];
                    loop {
                        if !render_thread_running_.load(Ordering::Relaxed) {
                            break;
                        }
                        if let Ok(buffer) = capturer.pull_buffer() {
                            buf.extend_from_slice((*buffer).as_ref());
                            let frame = VideoFrame::new(
                                settings.format,
                                settings.width,
                                settings.height,
                                buf.clone(),
                            );
                            video_capturer.provide_frame(0, &frame).unwrap();
                            buf.clear();
                        }
                        std::thread::sleep(std::time::Duration::from_micros(30 * 1_000));
                    }
                });
                Ok(())
            })
            .stop(move |_| {
                render_thread_running_.store(false, Ordering::Relaxed);
                Ok(())
            })
            .build();
        let video_capturer = VideoCapturer::new(Default::default(), video_capturer_callbacks);

        let publisher = Arc::new(Mutex::new(Publisher::new(
            "publisher",
            Some(video_capturer),
            publisher_callbacks,
        )));

        let publisher_ = publisher.clone();
        let session_callbacks = SessionCallbacks::builder()
            .on_connected(move |session| {
                let _ = session.publish(&*publisher_.lock().unwrap());
            })
            .on_error(|_, error, _| {
                panic!("{:?}", error);
            })
            .build();

        let session = Session::new(&api_key, &session_id, session_callbacks).unwrap();

        session.connect(&token).unwrap();

        receiver.recv().unwrap();

        audio_capture_thread_running_.store(false, Ordering::Relaxed);
        render_thread_running__.store(false, Ordering::Relaxed);

        test_teardown();
    }

    #[test]
    fn test_subscriber() {
        let (api_key, session_id, token) = setup_test();

        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));

        let credentials = Credentials {
            api_key: api_key.clone(),
            session_id: session_id.clone(),
            token: token.clone(),
        };

        let sender_ = sender.clone();
        let publisher = Arc::new(Mutex::new(UtilsPublisher::new(
            credentials,
            Some(Box::new(move |_, _| {
                sender_.lock().unwrap().send(()).unwrap();
            })),
            None,
        )));

        std::thread::spawn(move || {
            publisher.lock().unwrap().run().unwrap();
        });

        receiver.recv().unwrap();

        let audio_sample_received = Arc::new(AtomicBool::new(false));
        let video_frame_received = Arc::new(AtomicBool::new(false));
        let audio_sample_received_ = audio_sample_received.clone();
        let video_frame_received_ = video_frame_received.clone();
        let (done_sender, done_receiver) = mpsc::channel();
        let done_sender = Arc::new(Mutex::new(done_sender));
        let done_sender_ = done_sender.clone();

        let audio_device = AudioDevice::get_instance();
        audio_device
            .lock()
            .unwrap()
            .set_on_audio_sample_callback(Box::new(move |_| {
                audio_sample_received.store(true, Ordering::Relaxed);
                if video_frame_received.load(Ordering::Relaxed) {
                    done_sender.lock().unwrap().send(()).unwrap();
                }
            }));

        let subscriber_callbacks = SubscriberCallbacks::builder()
            .on_render_frame(move |_, _| {
                video_frame_received_.store(true, Ordering::Relaxed);
                if audio_sample_received_.load(Ordering::Relaxed) {
                    done_sender_.lock().unwrap().send(()).unwrap();
                }
            })
            .on_error(|_, error, _| {
                eprintln!("on_error {:?}", error);
            })
            .build();

        let subscriber = Arc::new(Subscriber::new(subscriber_callbacks));

        let session_callbacks = SessionCallbacks::builder()
            .on_stream_received(move |session, stream| {
                if subscriber.set_stream(stream).is_ok() {
                    session.subscribe(&subscriber).unwrap();
                }
            })
            .on_error(|_, error, _| {
                eprintln!("on_error {:?}", error);
            })
            .build();
        let session = Session::new(&api_key, &session_id, session_callbacks).unwrap();

        session.connect(&token).unwrap();

        done_receiver.recv().unwrap();

        test_teardown();
    }
}
