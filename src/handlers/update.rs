use crate::models::DataEntry;
use crate::state::AppState;
use tide::Request;

pub async fn update_data(mut req: Request<AppState>) -> tide::Result {
    // Extrai o id da URL (ex: /data/:id)
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => return Err(tide::Error::from_str(400, "Invalid id")),
    };

    // Lê o corpo da requisição como JSON
    let entry: DataEntry = req.body_json().await?;

    // Pega o estado global
    let state = req.state();
    let mut map = state.lock().unwrap();

    // Atualiza o registro se existir
    if let std::collections::hash_map::Entry::Occupied(mut e) = map.entry(id) {
        e.insert(entry);
        Ok(tide::Response::new(200))
    } else {
        Ok(tide::Response::new(404))
    }
}