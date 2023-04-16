#[derive(Debug)]
pub struct Json(pub String);

pub fn parse_rsource_to_json(_rsource: &str) -> Json {
    let rsource_json = r#"
    {
        "type": "RSource",
        "rsource": {
            "type": "File",
            "file": {
                "path": "/path/to/file"
            }
        }
    }
    "#;
    Json(rsource_json.into())
}
