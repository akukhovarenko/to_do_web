use std::fs;

pub fn read_file(file_path: &str) -> String {
    println!("file_path = {}", file_path);
    fs::read_to_string(file_path).expect("Unable to read file")
}

pub fn add_component(component_tag: String, html_data: String) -> String {
    let css_tag = component_tag.to_uppercase() + &String::from("_CSS");
    let html_tag = component_tag.to_uppercase() + &String::from("_HTML");
    let css_path = String::from("templates/components/")
        + &component_tag.to_lowercase()
        + &String::from(".css");
    let css_loaded = read_file(&css_path);

    let html_path = String::from("templates/components/")
        + &component_tag.to_lowercase()
        + &String::from(".html");
    let html_loaded = read_file(&html_path);

    let html_data = html_data.replace(html_tag.as_str(), &html_loaded);
    let html_data = html_data.replace(css_tag.as_str(), &css_loaded);

    html_data
}
