use crate::state::AppState;
use crate::auth::get_authenticated_user;
use tide::Request;

pub async fn delete_data(req: Request<AppState>) -> tide::Result {
    let id: u32 = req.param("id")?.parse().map_err(|_| tide::Error::from_str(400, "Invalid id"))?;
    let user = get_authenticated_user(&req).ok_or_else(|| tide::Error::from_str(401, "Unauthorized"))?;

    let state = req.state();
    let mut map = state.lock().unwrap();

    if let Some(entry) = map.get(&id) {
        if entry.owner != user {
            return Ok(tide::Response::new(403));
        }
    } else {
        return Ok(tide::Response::new(404));
    }

    map.remove(&id);
    Ok(tide::Response::new(204))
}
