//TODO: add errors
//TODO: stop using unwrap as much
use deno_bindgen::deno_bindgen;
use once_cell::sync::Lazy;
use rusqlite::types::ValueRef;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
struct QueryParams {
    params: Vec<String>,
}

pub static HANDLES: Lazy<Arc<Mutex<HashMap<usize, Arc<Mutex<Connection>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
pub static RESULTS: Lazy<Arc<Mutex<HashMap<usize, Arc<Mutex<Vec<u8>>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
pub static ERRORS: Lazy<Arc<Mutex<HashMap<usize, Arc<Mutex<Vec<u8>>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[deno_bindgen(nonblocking)]
pub fn open(id: usize, connstr: &str) -> usize {
    match Some(connstr) {
        Some(":memory:") => match Connection::open_in_memory() {
            Ok(conn) => {
                HANDLES
                    .lock()
                    .unwrap()
                    .insert(id, Arc::new(Mutex::new(conn)));
                0
            }
            Err(_) => 1,
        },
        Some(connstr) => match Connection::open(connstr) {
            Ok(conn) => {
                HANDLES
                    .lock()
                    .unwrap()
                    .insert(id, Arc::new(Mutex::new(conn)));
                0
            }
            Err(_) => 1,
        },
        None => 1,
    }
}

#[deno_bindgen(nonblocking)]
pub fn close(id: usize) -> usize {
    HANDLES.lock().unwrap().remove(&id);
    RESULTS.lock().unwrap().remove(&id);
    ERRORS.lock().unwrap().remove(&id);
    0
}

#[deno_bindgen(nonblocking)]
pub fn get_result_length(id: usize) -> usize {
    match RESULTS.lock().unwrap().get(&id) {
        Some(vec) => vec.lock().unwrap().len(),
        None => 0,
    }
}

#[deno_bindgen(nonblocking)]
pub fn fill_result(id: usize, buffer: &mut [u8]) {
    match RESULTS.lock().unwrap().get(&id) {
        Some(vec) => buffer.swap_with_slice(&mut vec.lock().unwrap()),
        None => buffer.swap_with_slice(&mut Vec::new()),
    }
}

#[deno_bindgen(nonblocking)]
pub fn get_error_length(id: usize) -> usize {
    match ERRORS.lock().unwrap().get(&id) {
        Some(vec) => vec.lock().unwrap().len(),
        None => 0,
    }
}

#[deno_bindgen(nonblocking)]
pub fn fill_error(id: usize, buffer: &mut [u8]) {
    match ERRORS.lock().unwrap().get(&id) {
        Some(vec) => buffer.swap_with_slice(&mut vec.lock().unwrap()),
        None => buffer.swap_with_slice(&mut Vec::new()),
    }
}

#[deno_bindgen(nonblocking)]
pub fn execute(id: usize, query: &[u8], params: &[u8]) -> usize {
    let params: &str = match str::from_utf8(params) {
        Ok(v) => v,
        Err(_) => "",
    };
    let query: &str = match str::from_utf8(query) {
        Ok(v) => v,
        Err(_) => "",
    };
    //TODO: use match instead
    if query == "" {
        return 3;
    };
    match Some(params) {
        Some("") => 2,
        Some(params) => {
            let params: Vec<String> = serde_json::from_str::<QueryParams>(params).unwrap().params;
            match HANDLES
                .lock()
                .unwrap()
                .get(&id)
                .unwrap()
                .lock()
                .unwrap()
                .execute(query, rusqlite::params_from_iter(params))
            {
                Ok(_) => 0,
                Err(_) => 1,
            }
        }
        None => 4,
    }
}

//TODO: get rid of columns argument
#[deno_bindgen(nonblocking)]
pub fn query(id: usize, query: &[u8], params: &[u8], columns: usize) -> usize {
    let params: &str = match str::from_utf8(params) {
        Ok(v) => v,
        Err(_) => "",
    };
    let query: &str = match str::from_utf8(query) {
        Ok(v) => v,
        Err(_) => "",
    };
    //TODO: use match instead
    if query == "" {
        return 3;
    };
    match Some(params) {
        Some("") => 2,
        Some(params) => {
            let params: Vec<String> = serde_json::from_str::<QueryParams>(params).unwrap().params;
            let conn: std::sync::MutexGuard<HashMap<usize, Arc<Mutex<Connection>>>> =
                HANDLES.lock().unwrap();
            let conn: &Arc<Mutex<Connection>> = conn.get(&id).unwrap();
            let conn: std::sync::MutexGuard<Connection> = conn.lock().unwrap();
            let mut stmt: rusqlite::Statement = conn.prepare(query).unwrap();
            let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
                let mut to_add: Vec<Vec<u8>> = Vec::new();
                for column in 0..columns {
                    let add_to_to_add: Vec<u8> = match row.get_ref_unwrap(column) {
                        ValueRef::Null => Vec::new(),
                        ValueRef::Integer(i) => Vec::from(i.to_string().as_bytes()),
                        ValueRef::Real(f) => Vec::from(f.to_string().as_bytes()),
                        ValueRef::Text(t) => Vec::from(t),
                        ValueRef::Blob(b) => Vec::from(b),
                    };
                    to_add.push(add_to_to_add);
                }
                Ok(to_add)
            });
            match rows {
                Ok(result) => {
                    let mut encode_result: Vec<Vec<Vec<u8>>> = Vec::new();
                    for row in result {
                        encode_result.push(row.unwrap());
                    }
                    RESULTS.lock().unwrap().insert(
                        id,
                        Arc::new(Mutex::new(Vec::from(
                            serde_json::json!(encode_result).to_string().as_bytes(),
                        ))),
                    );
                    0
                }
                Err(_) => 1,
            }
        }
        None => 4,
    }
}
