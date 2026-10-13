mod error;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod state;

use std::sync::Arc;

use repositories::users::UserRepository;
use state::AppState;

#[tokio::main]
async fn main() {
    // Creamos el estado compartido una sola vez
    let state = AppState {
        users: Arc::new(UserRepository::default()),
    };

    // Construimos la aplicación con todas sus rutas
    let app = routes::app(state);

    // Levantamos el servidor con hyper, escuchando en el puerto 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor escuchando en http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}