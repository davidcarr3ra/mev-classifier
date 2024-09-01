use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JitoBundleResponse {
    pub bundle_id: String,
    pub timestamp: String,
    pub tippers: Vec<String>,
    pub transactions: Vec<String>,
    pub landed_tip_lamports: u64,
}

pub fn fetch_jito_bundles() -> Result<Vec<JitoBundleResponse>, reqwest::Error> {
    let url = "https://explorer.jito.wtf/wtfrest/api/v1/bundles/recent?limit=1&sort=Time&asc=false&timeframe=Week";
    let response = get(url)?;
    let bundles: Vec<JitoBundleResponse> = response.json()?;
    Ok(bundles)
}
