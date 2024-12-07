use std::process::Command;

fn get_paths() -> String {
    let output = Command::new("valet")
        .arg("paths")
        .output()
        .map_err(|e| e.to_string())
        .unwrap();

    return String::from_utf8_lossy(&output.stdout).to_string();
}

#[tauri::command]
fn get_php_global_version() -> String {
    let output = Command::new("php")
        .arg("-v")
        .output()
        .map_err(|e| e.to_string())
        .unwrap();

    return String::from_utf8_lossy(&output.stdout).to_string();
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_links, get_php_global_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
