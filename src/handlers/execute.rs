use crate::state::AppState;
use serde::Deserialize;
use serde_json::json;
use tide::{Request, Response, StatusCode};
use wasmi::{Engine, Instance, Module, Store, TypedFunc};

#[derive(Deserialize)]
struct ExecRequest {
    #[serde(rename = "fn")]
    func: String,
    arg: [i32; 2],
}

pub async fn execute_fn(mut req: Request<AppState>) -> tide::Result {
    // Lê e valida o JSON do body
    let exec_req: ExecRequest = req.body_json().await.map_err(|_| {
        tide::Error::from_str(400, "Invalid JSON: esperado { fn: string, arg: [i32; 2] }")
    })?;

    // Busca o registro no estado global
    let id: u32 = match req.param("id") {
        Ok(s) => s
            .parse()
            .map_err(|_| tide::Error::from_str(400, "Invalid id"))?,
        Err(_) => return Err(tide::Error::from_str(400, "Missing id")),
    };
    let map = req.state().lock().unwrap();
    let entry = match map.get(&id) {
        Some(e) => e,
        None => return Err(tide::Error::from_str(404, "Not found")),
    };
    let wasm_bytes = &entry.bytecode;

    // Carrega e instancia o wasm (wasmi 0.47.0)
    let engine = Engine::default();
    let module = Module::new(&engine, wasm_bytes)
        .map_err(|e| tide::Error::from_str(StatusCode::BadRequest, format!("Invalid wasm: {e}")))?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).map_err(|e| {
        tide::Error::from_str(
            StatusCode::InternalServerError,
            format!("Wasm instantiation error: {e}"),
        )
    })?;

    // Busca a função exportada
    let func = instance
        .get_func(&mut store, &exec_req.func)
        .ok_or_else(|| {
            tide::Error::from_str(
                StatusCode::BadRequest,
                format!("Function not found: {}", exec_req.func),
            )
        })?;
    let typed: TypedFunc<(i32, i32), i32> = func.typed(&store).map_err(|e| {
        tide::Error::from_str(StatusCode::BadRequest, format!("Signature error: {e}"))
    })?;

    // Executa a função
    let result = typed
        .call(&mut store, (exec_req.arg[0], exec_req.arg[1]))
        .map_err(|e| {
            tide::Error::from_str(StatusCode::InternalServerError, format!("Call error: {e}"))
        })?;

    Ok(Response::builder(StatusCode::Ok)
        .body(serde_json::to_string(&json!({ "result": result }))?)
        .content_type(tide::http::mime::JSON)
        .build())
}