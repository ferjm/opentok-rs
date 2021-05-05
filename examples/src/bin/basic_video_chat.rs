extern crate opentok;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: basic_video_chat <api key> <session ID> <token>");
        std::process::exit(-1);
    }

    let api_key: &str = args[1].as_ref();
    let session_id: &str = args[2].as_ref();
    let token: &str = args[3].as_ref();

    let _ = opentok::init();

    let session = opentok::session::Session::new(api_key, session_id).unwrap();
    let _ = session.connect(token);

    let _ = opentok::deinit();
}
