use clap::{load_yaml, App};

#[derive(Debug, Default)]
pub struct Credentials {
    pub api_key: String,
    pub session_id: String,
    pub token: String,
}

pub async fn parse_cli() -> Result<Credentials, anyhow::Error> {
    let yaml = load_yaml!("cli.yaml");
    let mut app = App::from(yaml);
    let matches = app.clone().get_matches();

    let mut credentials = Credentials::default();
    if let Some(room_url) = matches.value_of("url") {
        let info_url = format!("{}/info", room_url);
        let payload = surf::get(&info_url)
            .recv_string()
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        let json = json::parse(&payload).expect("Invalid JSON");
        credentials.api_key = json["apiKey"].as_str().unwrap().into();
        credentials.session_id = json["sessionId"].as_str().unwrap().into();
        credentials.token = json["token"].as_str().unwrap().into();
    } else {
        if let Some(api_key) = matches.value_of("api_key") {
            credentials.api_key = api_key.into();
        }
        if let Some(session_id) = matches.value_of("session_id") {
            credentials.session_id = session_id.into();
        }
        if let Some(token) = matches.value_of("token") {
            credentials.token = token.into();
        }
    }

    if credentials.api_key.is_empty()
        || credentials.session_id.is_empty()
        || credentials.token.is_empty()
    {
        app.print_help().unwrap();
        return Err(anyhow::anyhow!("Failed to parse arguments"));
    }

    Ok(credentials)
}
