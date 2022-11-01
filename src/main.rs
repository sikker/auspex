use std::process::Command;
use std::collections::HashMap;
use terminal_link::Link;
use serde_json::Value;

/// Fetch list of foreign packages in pacman
/// 
/// Any package installed manually with pacman, as in not installed using pacman -S,
/// is considered foreign. We're only interested in the ones installed from AUR, but
/// that's caught later.
fn pacman_packages() -> HashMap<String, String> {
    let output = Command::new("pacman")
                .arg("-Qm")
                .output()
                .expect("failed to execute process");

    let output_string = String::from_utf8(output.stdout).unwrap();
    let lines = output_string.split("\n");

    let mut packages = HashMap::new();

    for line in lines {
        if line == "" {
            continue;
        }
        let parts = line.split(" ").collect::<Vec<&str>>();
        packages.insert(String::from(parts[0]), String::from(parts[1]));
    }

    packages
}

/// Convert package list to AUR query string
/// 
/// The AUR api has a specific format it wants the list of packages to look up in,
/// so we need to massage the data a bit before we can make the request.
fn packages_to_query_string(packages: &HashMap<String, String>) -> String {
    let mut uri_args = String::from("");

    for (name, _version) in packages {
        uri_args = uri_args + "arg[]=" + &name + "&";
    }

    uri_args
}

/// Look for our foreign packages on AUR
/// 
/// Any packages we found in pacman earlier that did not come from the official repos
/// will be checked against the AUR api. Any that exist in AUR will be returned with
/// their latest version number. Foreign packages that do not exist in AUR will be
/// quietly ignored.
fn aur_lookup(uri: String) -> HashMap<String, String> {
    let mut packages = HashMap::new();
    let results;
    let json = reqwest::blocking::get(uri)
    .expect("invalid request")
    .json::<Value>()
    .expect("invalid JSON response");

    match json["results"].as_array() {
        Some(package_list) => results = package_list,
        None => return packages,
    }

    for package in results.iter() {
        let package_name: String = String::from(package["Name"].as_str().unwrap());
        let package_version: String = String::from(package["Version"].as_str().unwrap());
        packages.insert(package_name, package_version);
    }

    packages
}

/// Auspex, for reading AUR
/// 
/// Simple commandline tool for Arch Linux to list packages installed from AUR
/// that have updates ready. Useful helper for people who prefer to manually
/// operate AUR rather than use a frontend, but don't want to have to manually
/// _check_ AUR for updates.
fn main() {
    let packages = &pacman_packages();
    let uri = format!("https://aur.archlinux.org/rpc/?v=5&type=info&{}", packages_to_query_string(packages));    
    let results = aur_lookup(uri);

    for (name, version) in results {
        let aur_link_uri = "https://aur.archlinux.org/packages/".to_string() + &name;
        let aur_link = Link::new(&aur_link_uri, &aur_link_uri);

        if version != packages[&name] {
            println!("{} {} has new version {} available: {}", name, packages[&name], version, aur_link);
        }
    }
}
