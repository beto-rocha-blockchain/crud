use crate::models::DataEntry;
use crate::state::AppState;
use crate::auth::get_authenticated_user;
use tide::{Request, Response, StatusCode};

#[derive(serde::Deserialize)]
struct CreatePayload {
    func_names: Vec<String>,
    bytecode: Vec<u8>,
}

pub async fn create_data(mut req: Request<AppState>) -> tide::Result {
    let payload: CreatePayload = req.body_json().await?;
    let user = get_authenticated_user(&req)
        .ok_or_else(|| tide::Error::from_str(401, "Unauthorized"))?;

    let state = req.state();
    let mut map = state.lock().unwrap();
    let new_id = map.len() as u32 + 1;

    let entry = DataEntry {
        func_names: payload.func_names,
        bytecode: payload.bytecode,
        owner: user,
    };

    map.insert(new_id, entry);

    let mut res = Response::new(StatusCode::Created);
    res.set_body(tide::Body::from_json(&serde_json::json!({ "id": new_id }))?);
    Ok(res)
}
