use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct PhpVersion {
    version: String,
    full_version: String,
}

#[tauri::command]
fn get_php_global_versions() -> Vec<PhpVersion> {
    // List PHP versions in /usr/bin
    let output = Command::new("bash")
        .arg("-c")
        .arg("ls /usr/bin/php* | grep -oP 'php\\d+\\.\\d+$' | sort -V")
        .output()
        .map_err(|e| e.to_string())
        .unwrap();

    let versions_str = String::from_utf8_lossy(&output.stdout).to_string();

    versions_str
        .split('\n')
        .filter(|v| !v.is_empty())
        .map(|version| {
            // Get full version information
            let full_version_output = Command::new(version)
                .arg("-v")
                .output()
                .map_err(|e| e.to_string())
                .unwrap();

            let full_version = String::from_utf8_lossy(&full_version_output.stdout).to_string();

            PhpVersion {
                version: version.to_string(),
                full_version,
            }
        })
        .collect()
}

#[tauri::command]
fn check_php_updates() -> Vec<String> {
    // Check for PHP updates from Ondřej Sury PPA
    let update_check = Command::new("bash")
        .arg("-c")
        .arg("sudo apt update && apt list --upgradable | grep php")
        .output()
        .map_err(|e| e.to_string())
        .unwrap();

    let updates_str = String::from_utf8_lossy(&update_check.stdout).to_string();

    updates_str
        .split('\n')
        .filter(|u| !u.is_empty())
        .map(|update| update.to_string())
        .collect()
}

#[tauri::command]
fn get_links() -> String {
    let paths = get_paths();
    let paths = serde_json::from_str(&paths).unwrap();
    let sites = get_sites(paths);
    println!("Sites: {:#?}", sites);

    let sites_json = serde_json::to_string(&sites).unwrap();

    return sites_json;
}

fn get_sites(paths: Vec<String>) -> Vec<String> {
    let mut sites = Vec::new();
    let mut exists = Vec::new();

    for path in paths {
        let output = Command::new("ls")
            .current_dir(path.clone())
            .output()
            .map_err(|e| e.to_string())
            .unwrap();

        let site = String::from_utf8_lossy(&output.stdout).to_string();

        for site in site.split("\n") {
            if site != "" && !exists.contains(&String::from(site)) {
                exists.push(String::from(site));
                sites.push(format!("{}/{}", path, site));
            }
        }
    }

    return sites;
}

fn get_paths() -> String {
    let output = Command::new("valet")
        .arg("paths")
        .output()
        .map_err(|e| e.to_string())
        .unwrap();

    return String::from_utf8_lossy(&output.stdout).to_string();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_links,
            get_php_global_versions,
            check_php_updates
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
