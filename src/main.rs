use std::process::Command;
use std::collections::HashMap;
use terminal_link::Link;
use serde_json::Value;

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

fn packages_to_query_string(packages: &HashMap<String, String>) -> String {
    let mut uri_args = String::from("");

    for (name, _version) in packages {
        uri_args = uri_args + "arg[]=" + &name + "&";
    }

    uri_args
}

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
