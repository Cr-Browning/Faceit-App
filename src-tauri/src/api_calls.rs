use reqwest::Client;
use dotenv::dotenv;

#[tauri::command]
async fn get_faceit_player_stats(player_name: String) -> Result<String, String> {
    dotenv().ok();

    let api_key = env::var("FACEIT_API_KEY").unwrap();
    let client = Client::new();
    let url = format!("https://open.faceit.com/data/v4/players?nickname={}", player_name);

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let body = res.text().await.map_err(|e| e.to_string())?;
        Ok(body)
    } else {
        Err(format!("Failed to fetch stats: {}", res.status()))
    }
}