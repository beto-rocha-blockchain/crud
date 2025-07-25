use crate::state::AppState;
use tide::Request;

pub async fn read_all_data(req: Request<AppState>) -> tide::Result {
    // Pega o estado global
    let state = req.state();
    let map = state.lock().unwrap();

    // Retorna todos os registros como JSON
    Ok(tide::Body::from_json(&*map)?.into())
}

pub async fn read_data(req: Request<AppState>) -> tide::Result {
    // Extrai o id da URL (ex: /data/:id)
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => return Err(tide::Error::from_str(400, "Invalid id")),
    };

    // Pega o estado global
    let state = req.state();
    let map = state.lock().unwrap();

    // Procura o registro pelo id
    if let Some(entry) = map.get(&id) {
        Ok(tide::Body::from_json(entry)?.into())
    } else {
        Ok(tide::Response::new(404))
    }
}