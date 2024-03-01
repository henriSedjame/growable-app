
use axum::{Json, Router};
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::routing::post;
use dotenv::dotenv;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

type ApiResult = Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)>;

mod constants {
    pub const PLUGINS_DIR: &str = "plugins";
    pub const ALLOWED_ORIGINS: &str = "ALLOWED_ORIGINS";
    pub const ALLOWED_ORIGINS_NOT_FOUND: &str = "Variable ALLOWED_ORIGINS is not found";
    pub const WASM_EXTENSION: &str= ".wasm";
    pub const ADDR: &str = "127.0.0.1:8080";

    pub mod messages {
        pub const FAILED_GET_FILENAME: &str = "Failed to retrieve file name";
        pub const FAILED_READ_PLUGINS_DIR: &str = "Failed to read plugins directory";
        pub const FAILED_DEL_PLUGIN: &str = "Failed to remove plugin file";

        pub const EMPTY_FILE_SENT: &str = "Empty file sent";
        
        pub const PLUGIN_ALREADY_EXISTT: &str = "A plugin already exist with name";
        
        pub const PLUGIN_NOT_FOUND: &str = "Plugin not found with name";
        
    }
    
    pub mod routes {
        pub const DEFAULT: &str = "/";
        pub const PLUGINS : &str = "/plugins";
    }
}

mod handlers {
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    use axum::extract::{Multipart, Query};
    use axum::http::StatusCode;
    use axum::Json;
    use serde::Deserialize;
    use serde_json::{json};

    use crate::{ApiResult, constants, utils::get_plugins_dir};
    use crate::constants::{PLUGINS_DIR, WASM_EXTENSION, };

    #[derive(Deserialize)]
    pub struct DeleteQuery {
        name: String
    }
    pub async fn save_file(mut multipart: Multipart) -> ApiResult {
        upsert(&mut multipart, true).await
    }

    pub async fn update_file(mut multipart: Multipart) -> ApiResult {
        upsert(&mut multipart, false).await
    }

    pub async fn get_plugins() -> ApiResult {
        let plugins = fs::read_dir(get_plugins_dir())
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(constants::messages::FAILED_READ_PLUGINS_DIR))))?
            .filter_map(|p| {
                let entry = p.unwrap();
                if entry.file_type().unwrap().is_file() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.ends_with(WASM_EXTENSION) {
                        Some(file_name.replace(WASM_EXTENSION, ""))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        Ok(Json(json!(plugins)))
    }
    
    pub async fn delete_plugin(Query(query): Query<DeleteQuery>) -> ApiResult {
       
        let mut plugin_file = get_plugins_dir();
        plugin_file.push(format!("{}{}", query.name, WASM_EXTENSION));
        
        fs::remove_file(plugin_file)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(constants::messages::FAILED_DEL_PLUGIN))))?;
            
        Ok(Json(json!(true)))
    }
    
    async fn upsert(multipart: &mut Multipart, save: bool) -> ApiResult {
        if let Some(file) = multipart.next_field().await.unwrap() {
            
            let name = file.file_name()
                .ok_or((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(constants::messages::FAILED_GET_FILENAME))))?
                .to_string();

            if save && exist(name.clone()) {
                return Err((StatusCode::FORBIDDEN, Json(json!(format!("{} {}", constants::messages::PLUGIN_ALREADY_EXISTT, name.clone())))))
            }
            
            if !save && !exist(name.clone()) {
                return Err((StatusCode::FORBIDDEN, Json(json!(format!("{} {}", constants::messages::PLUGIN_NOT_FOUND, name.clone())))))
            }

            let data = file.bytes().await
                .map_err(|err| (err.status(), Json(json!(err.to_string()))))?;

            let mut wasm_file = File::create(format!("./{}/{}", PLUGINS_DIR, name))
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(err.to_string()))))?;

            wasm_file.write_all(&*data)
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(err.to_string()))))?;

            return Ok(Json(json!(true)));
        }

        Err((StatusCode::BAD_REQUEST, Json(json!(constants::messages::EMPTY_FILE_SENT))))
    }
    
    fn exist(plugin_name: String) -> bool {
        fs::read_dir(get_plugins_dir())
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(constants::messages::FAILED_READ_PLUGINS_DIR))))
            .unwrap()
            .find(|p| p.as_ref().unwrap().file_name().to_string_lossy().to_string() == plugin_name)
            .is_some()
    }
}

mod utils {
    use std::env::{current_dir, var};
    use std::path::PathBuf;

    use axum::http::HeaderValue;

    use crate::constants;
    use crate::constants::PLUGINS_DIR;

    pub fn get_plugins_dir() -> PathBuf {
        let mut buf = current_dir().unwrap();

        buf.push(PLUGINS_DIR);

        buf
    }

    pub fn get_allowed_origins() -> Vec<HeaderValue> {
        var(constants::ALLOWED_ORIGINS)
            .map(|origins|
                origins.split(",")
                    .into_iter()
                    .map(|origin| origin.parse::<HeaderValue>().unwrap())
                    .collect::<Vec<HeaderValue>>()
            ).expect(constants::ALLOWED_ORIGINS_NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(utils::get_allowed_origins());

    let app = Router::new()
        .route(constants::routes::PLUGINS, 
               post(handlers::save_file)
                   .put(handlers::update_file)
                   .delete(handlers::delete_plugin)
                   .get(handlers::get_plugins)
        )
        .layer(DefaultBodyLimit::disable())
        .nest_service(constants::routes::DEFAULT, ServeDir::new(utils::get_plugins_dir()))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(constants::ADDR)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap()
}