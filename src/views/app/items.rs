use super::content_loader;
use actix_web::{http::header::ContentType, HttpResponse};

pub async fn items() -> HttpResponse {
    let mut html_data = content_loader::read_file("templates/main.j2");
    let javascript_data = content_loader::read_file("javascript/main.js");
    let base_css_data = content_loader::read_file("css/base.css");
    let css_data = content_loader::read_file("css/main.css");
    html_data = content_loader::add_component(String::from("header"), html_data);

    HttpResponse::Ok().content_type(ContentType::html()).body(
        html_data
            .replace("{% JAVASCRIPT %}", &javascript_data)
            .replace("{% BASE_CSS %}", &base_css_data)
            .replace("{% CSS %}", &css_data),
    )
}
