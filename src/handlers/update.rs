use crate::models::DataEntry;
use crate::state::AppState;
use crate::auth::get_authenticated_user;
use tide::Request;

#[derive(serde::Deserialize)]
struct UpdatePayload {
    data1: Vec<String>,
    data2: Vec<u8>,
}

pub async fn update_data(mut req: Request<AppState>) -> tide::Result {
    let id: u32 = req.param("id")?.parse().map_err(|_| tide::Error::from_str(400, "Invalid id"))?;
    let payload: UpdatePayload = req.body_json().await?;
    let user = get_authenticated_user(&req).ok_or_else(|| tide::Error::from_str(401, "Unauthorized"))?;

    let state = req.state();
    let mut map = state.lock().unwrap();

    if let Some(existing) = map.get_mut(&id) {
        if existing.owner != user {
            return Ok(tide::Response::new(403));
        }

        existing.data1 = payload.data1;
        existing.data2 = payload.data2;

        Ok(tide::Response::new(200))
    } else {
        Ok(tide::Response::new(404))
    }
}
