use std::collections::HashMap;

#[tauri::command]
pub async fn request_package(value: &str) -> Result<String, String> {
    let _resp = reqwest::get("https://httpbin.org/ip")
        .await
        .map_err(|e| e.to_string())?
        .json::<HashMap<String, String>>()
        .await
        .map_err(|err| err.to_string());

    let a = "";
    return Ok(a.to_owned());
}
