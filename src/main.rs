use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Client {
    id: u32,
    name: String,
    document: String,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    server: ServerConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

fn load_config() -> Result<AppConfig, ConfigError> {
    let mut cfg = Config::new();
    cfg.merge(File::with_name("Actix"))?;
    cfg.try_into()
}

#[derive(Serialize)]
struct Message {
    mensagem: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().json(Message { mensagem: "Hello rust".to_string() })
}

async fn list_clients() -> impl Responder {
    let clients = show_clients_list();
    HttpResponse::Ok().json(clients)
}

async fn create_clients(client: web::Json<Client>) -> impl Responder {
    let mut clients = show_clients_list();
    let cli = client.into_inner();
    clients.push(cli.clone());

    HttpResponse::Created().json(cli)
}

async fn get_client(path: web::Path<(String,)>) -> impl Responder {
    let clients = show_clients_list();
    if let Some(client) = clients.iter().find(|c| c.id.to_string() == path.0){

        HttpResponse::Ok().json(client)
    }
    else {
        HttpResponse::NotFound().json(Message { mensagem: "Hello rust 2".to_string() })
    }
    
}

async fn update_client(path: web::Path<(String,)>, client: web::Json<Client>) -> impl Responder {
    let mut clients = show_clients_list();
    let id = &path.0;
    if let Some(c) = clients.iter_mut().find(|c| c.id.to_string() == *id) {
        c.name = client.name.clone();
        c.document = client.document.clone();
        HttpResponse::Ok().json(c)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_clients(path: web::Path<(String,)>) -> impl Responder {
    let mut clients = show_clients_list();
    let id = &path.0;

    if let Some(index) = clients.iter().position(|c| c.id.to_string() == *id) {
        clients.remove(index);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

fn show_clients_list() -> Vec<Client> {
    let client1 = Client {
        id: 1,
        name: String::from("Client 1"),
        document: String::from("111.111.111-11"),
    };
    
    let client2 = Client {
        id: 2,
        name: String::from("Client 2"),
        document: String::from("222.222.222-22"),
    };
    
    let mut client_list = Vec::new();
    client_list.push(client1);
    client_list.push(client2);
    
    client_list
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = load_config().expect("Failed to load configuration.");
    let server_cfg = cfg.server;
    let address = format!("0.0.0.0:{}", server_cfg.port);

    println!("Starting server on http://{}", address);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/clients", web::get().to(list_clients))
            .route("/clients", web::post().to(create_clients))
            .route("/clients/{id}", web::get().to(get_client))
            .route("/clients/{id}", web::put().to(update_client))
            .route("/clients/{id}", web::delete().to(delete_clients))

    })
    .bind(&address)?
    .run()
    .await

}