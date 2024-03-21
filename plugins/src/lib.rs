
use extism_pdk::{FnResult, host_fn, Json, plugin_fn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    age: u8,
    name: String
}

#[host_fn]
extern "ExtismHost"{
    fn alert(msg: String) -> String ;
}

#[plugin_fn]
pub fn hello(Json(person): Json<Person>) -> FnResult<String>  {
    let msg = unsafe { alert(String::from("Hey")) }.unwrap();
    
    Ok(format!("🤚{} 😃 je m'appelle {} et j'ai {} ans!!", msg,  person.name, person.age))
}


