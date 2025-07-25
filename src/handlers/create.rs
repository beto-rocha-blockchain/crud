use crate::models::DataEntry;
use crate::state::AppState;
use tide::Request;

pub async fn create_data(mut req: Request<AppState>) -> tide::Result {
    let entry: DataEntry = req.body_json().await?;
    let state = req.state();
    let mut map = state.lock().unwrap();
    let new_id = map.len() as u32 + 1;

    map.insert(new_id, entry);

    Ok(tide::Body::from_json(&serde_json::json!({ "id": new_id }))?.into())
}