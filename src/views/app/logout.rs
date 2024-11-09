use actix_web::{HttpResponse, http::header::ContentType};

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(
    "<html>\
        <script>\
        localStorage.removeItem('user-token'); \
        window.location.replace(document.location.origin);\
        </script>\
    </html>"
    )
}
