use opentok_utils::common::Credentials;
use std::collections::HashMap;
use url::Url;

pub async fn parse_cli() -> Result<(Credentials, Option<u64>), anyhow::Error> {
    let yaml = yaml_rust::Yaml::Array(yaml_rust::YamlLoader::load_from_str(include_str!(
        "cli.yaml"
    ))?);
    let mut app = clap_serde::yaml_to_app(&yaml)?;

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
    } else if let Some(url) = matches.value_of("opentok_url") {
        let url = Url::parse(url).unwrap();
        let query_params: HashMap<_, _> = url.query_pairs().into_owned().collect();

        credentials.api_key = query_params.get("key").cloned().unwrap_or_default();
        credentials.token = query_params.get("token").cloned().unwrap_or_default();
        credentials.session_id = url.host().map(|s| s.to_string()).unwrap_or_default();
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

    let duration = matches
        .value_of("duration")
        .map(|s| s.parse::<u64>().unwrap());

    Ok((credentials, duration))
}
