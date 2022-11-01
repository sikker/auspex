use std::process::Command;
use std::collections::HashMap;

fn main() {
    let output = Command::new("pacman")
                .arg("-Qm")
                .output()
                .expect("failed to execute process");

    let output_string = String::from_utf8(output.stdout).unwrap();
    let lines = output_string.split("\n");

    let mut uri_args = Vec::new();
    let mut local_packages: HashMap<String, String> = HashMap::new();

    for line in lines {
        if line == "" {
            continue;
        }
        let parts = line.split(" ").collect::<Vec<&str>>();
        uri_args.push(("arg[]", parts[0]));
        local_packages.insert(String::from(parts[0]), String::from(parts[1]));
    }

    let uri = format!("https://aur.archlinux.org/rpc/?v=5&type=info&{}", querystring::stringify(uri_args));

    let json = reqwest::blocking::get(uri)
                 .expect("invalid request")
               .json::<serde_json::Value>()
                 .expect("invalid JSON response");
    
    let results = json["results"].as_array().expect("no results found in JSON");

    for package in results.iter() {
        let package_name: String = String::from(package["Name"].as_str().unwrap());
        let package_version: String = String::from(package["Version"].as_str().unwrap());

        if package_version != local_packages[&package_name] {
            println!("{} needs update to AUR version {}; installed version is {}", package_name, package_version, local_packages[&package_name]);
        }
    }
}
