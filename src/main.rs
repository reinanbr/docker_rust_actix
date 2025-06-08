use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Clone)]
struct Data {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct Status {
    status: &'static str,
}

const FILE_PATH: &str = "data.json";

// GET /data - Lê JSON do arquivo e retorna
async fn get_data() -> impl Responder {
    let file = match File::open(FILE_PATH) {
        Ok(f) => f,
        Err(_) => return HttpResponse::Ok().json(Vec::<Data>::new()),
    };

    let reader = BufReader::new(file);
    let data: Result<Vec<Data>, serde_json::Error> = serde_json::from_reader(reader);

    match data {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao ler JSON"),
    }
}

// POST /data - Recebe JSON e salva no arquivo
async fn post_data(new_data: web::Json<Data>) -> impl Responder {
    let mut all_data = match File::open(FILE_PATH) {
        Ok(file) => {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
        }
        Err(_) => Vec::new(),
    };

    all_data.push(new_data.into_inner());

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH);

    match file {
        Ok(f) => {
            let writer = BufWriter::new(f);
            if serde_json::to_writer_pretty(writer, &all_data).is_ok() {
                HttpResponse::Ok().body("Salvo com sucesso")
            } else {
                HttpResponse::InternalServerError().body("Erro ao salvar")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Erro ao abrir arquivo"),
    }
}

// GET /hello - Retorna status online
async fn hello() -> impl Responder {
    let status = Status { status: "online" };
    HttpResponse::Ok().json(status)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/data", web::get().to(get_data))
            .route("/data", web::post().to(post_data))
            .route("/hello", web::get().to(hello)) // <- nova rota aqui
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
