use std::process::Command;

fn get_dependency_info(dep_name: &str) -> (String, String) {
    // Try to get git info from Cargo.lock
    if let Ok(lock_content) = std::fs::read_to_string("Cargo.lock") {
        // Parse Cargo.lock to find git dependencies
        let lines: Vec<&str> = lock_content.lines().collect();
        let mut in_package = false;
        let mut current_name = "";
        let mut version = "unknown".to_string();
        
        for line in lines {
            if line.starts_with("[[package]]") {
                in_package = true;
                current_name = "";
                version = "unknown".to_string();
            } else if in_package && line.starts_with("name = ") {
                current_name = line.trim_start_matches("name = ").trim_matches('"');
            } else if in_package && line.starts_with("version = ") && current_name == dep_name {
                version = line.trim_start_matches("version = ").trim_matches('"').to_string();
            } else if in_package && line.starts_with("source = ") && current_name == dep_name {
                let source = line.trim_start_matches("source = ").trim_matches('"');
                if source.starts_with("git+") {
                    // Extract git hash from source like "git+https://github.com/serenity-rs/serenity.git?branch=next#abc123def"
                    if let Some(hash_part) = source.split('#').nth(1) {
                        let git_hash = hash_part[..8.min(hash_part.len())].to_string(); // Take first 8 chars
                        return (version, git_hash);
                    }
                }
            } else if line.trim().is_empty() {
                in_package = false;
            }
        }
    }
    
    ("unknown".to_string(), "unknown".to_string())
}

fn main() {
    // Get git commit hash
    if let Ok(output) = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        let git_hash = String::from_utf8_lossy(&output.stdout);
        println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
    } else {
        println!("cargo:rustc-env=GIT_HASH=unknown");
    }

    // Get git branch
    if let Ok(output) = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        let git_branch = String::from_utf8_lossy(&output.stdout);
        println!("cargo:rustc-env=GIT_BRANCH={}", git_branch.trim());
    } else {
        println!("cargo:rustc-env=GIT_BRANCH=unknown");
    }

    // Get dependency git hashes and versions
    let (serenity_version, serenity_hash) = get_dependency_info("serenity");
    let (poise_version, poise_hash) = get_dependency_info("poise");
    
    println!("cargo:rustc-env=SERENITY_VERSION={}", serenity_version);
    println!("cargo:rustc-env=SERENITY_GIT_HASH={}", serenity_hash);
    println!("cargo:rustc-env=POISE_VERSION={}", poise_version);
    println!("cargo:rustc-env=POISE_GIT_HASH={}", poise_hash);

    // Get Rust compiler version
    if let Ok(output) = Command::new("rustc")
        .args(&["--version"])
        .output()
    {
        let rustc_version = String::from_utf8_lossy(&output.stdout);
        println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version.trim());
    } else {
        println!("cargo:rustc-env=RUSTC_VERSION=unknown");
    }

    // Get target triple
    let target = std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=TARGET={}", target);

    // Get build timestamp
    let timestamp = std::env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|epoch| epoch.parse::<i64>().ok())
        .map(|epoch| {
            use std::time::{UNIX_EPOCH, Duration};
            UNIX_EPOCH + Duration::from_secs(epoch as u64)
        })
        .unwrap_or_else(|| std::time::SystemTime::now());
    
    let formatted_time = format!("{}", humantime::format_rfc3339_seconds(timestamp));
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", formatted_time);

    // Rerun if git changes or Cargo.lock changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
    println!("cargo:rerun-if-changed=Cargo.lock");
}
