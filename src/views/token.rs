use actix_web::dev::ServiceRequest;


fn check_password(password: String) -> Result<String, &'static str> {
    if password == "token" {
        return Ok(password);
    }
    Err("Incorrect token")
}


fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match request.headers().get("user-token") {
        Some(token) => {
            Ok(token.to_str().unwrap().to_string())
        },
        None => Err("There's no token")
    }
}


pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match extract_header_token(request) {
        Ok(token) => check_password(token),
        Err(message) => Err(message)
    }
}