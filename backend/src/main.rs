use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
struct Scope {
    description: String,
    id: String,
    name: String,
}

impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Scope")
            .field("description", &self.description)
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    let resp = reqwest::get("https://www.reddit.com/api/v1/scopes").await;

    
    let content = match resp {
        Ok(t) => {
            match t.json::<HashMap<String, Scope>>().await {
                Ok(t2) => t2,
                Err(e) => HashMap::new(),
            }
        },
        Err(e) => HashMap::new(),
    };
    //     .json::<HashMap<String, Scope>>()
    //     .await;
    // web::Json(
    //     resp
    // )

    web::Json(content)
    .with_header("x-version", "1.2.3")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
