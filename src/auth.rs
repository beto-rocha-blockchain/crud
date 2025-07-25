use tide::{Request};

pub fn get_authenticated_user(req: &Request<impl Send + Sync>) -> Option<String> {
    req.header("Authorization")
        .and_then(|h| h.get(0))
        .map(|value| value.as_str().to_string())
}
