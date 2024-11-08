mod banner;
mod patterns;
mod config;
mod api;

use crate::api::router::create_router;
use crate::api::state::AppState;
use crate::config::Config;
use crate::patterns::load_patterns;
use anyhow::Result;
use dotenv::dotenv;
use log::info;
use std::sync::Arc;
// can be useful if new pattern comes out
// fn convert_patterns() -> Result<()> {
//     let paths = fs::read_dir("./patterns")?;
//
//     for dir_path in paths {
//         let path = dir_path?.path();
//
//         let mut img = ImageReader::open(&path)?.decode()?;
//         let cropped = crop(&mut img, 1, 1, 20, 40);
//         cropped.to_image().save(&path)?;
//     }
//
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();
    env_logger::init();
    
    let config = Config::from_figment().expect("Failed to parse config");

    let patterns = load_patterns().expect("Failed to read patterns");
    let app_state = AppState {
        patterns: Arc::new(patterns)
    };

    let app = create_router().with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&config.bind_address).await?;

    info!("Listening on {}", &config.bind_address);
    axum::serve(listener, app).await?;

    Ok(())
}

