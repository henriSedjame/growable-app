
use extism_pdk::{FnResult, Json, plugin_fn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    age: u8,
    name: String
}


#[plugin_fn]
pub fn hello(Json(person): Json<Person>) -> FnResult<String> {
    Ok(format!("🤚Hello 😃 je m'appelle {} et j'ai {} ans!!", person.name, person.age))
}


