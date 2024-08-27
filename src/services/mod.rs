use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use gloo_net::http::Request;

pub async fn fetch_data() -> Result<(), gloo_net::Error> {
    let response = Request::get("https://api.example.com/data")
        .send()
        .await?;

    if response.ok() {
        let data = response.json::<serde_json::Value>().await?;
        console::log_1(&data.into());
    } else {
        console::log_1(&"Failed to fetch data".into());
    }

    Ok(())
}
