use crate::state::AppState;
use tide::Request;

pub async fn delete_data(req: Request<AppState>) -> tide::Result {
    // Extrai o id da URL (ex: /data/:id)
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => return Err(tide::Error::from_str(400, "Invalid id")),
    };

    // Pega o estado global
    let state = req.state();
    let mut map = state.lock().unwrap();

    // Remove o registro se existir
    if map.remove(&id).is_some() {
        Ok(tide::Response::new(204))
    } else {
        Ok(tide::Response::new(404))
    }
}