use std::{fs, env};
use std::path::PathBuf;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use dirs::home_dir;

#[derive(Serialize, Deserialize)]
struct AppEntry {
    id: String,
    name: String,
    description: String,
    version: String,
    icon: String,
    execPath: String,
    downloadUrl: String,
    lastUpdated: String,
}

#[derive(Serialize, Deserialize)]
struct Catalog {
    apps: Vec<AppEntry>,
}

fn main() {
    println!("Hello from DummyApp");

    let exec_path = env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("unknown"))
        .display()
        .to_string();

    let catalog_path = home_dir()
        .unwrap()
        .join(".forgebox")
        .join("catalog.json");

    let content = fs::read_to_string(&catalog_path)
        .expect("Failed to read catalog.json");

    let mut catalog: Catalog = serde_json::from_str(&content)
        .expect("Invalid JSON");

    let now = Utc::now().to_rfc3339();

    for app in catalog.apps.iter_mut() {
        if app.id == "dummyapp" {
            app.execPath = exec_path.clone();
            app.lastUpdated = now.clone();
        }
    }

    fs::write(
        &catalog_path,
        serde_json::to_string_pretty(&catalog).unwrap()
    ).expect("Failed to write updated catalog");
}
