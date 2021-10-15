#[cfg(test)]
mod tests {
    use futures::executor::LocalPool;
    use opentok::log::{self, LogLevel};
    use opentok::session::{Session, SessionCallbacks};
    use opentok_server::{OpenTok, SessionOptions, TokenRole};
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
}
