use axum::Router;

use crate::auth_service;


pub async fn run_server() -> std::io::Result<()> {

    let app_data = Data::new(firebase_auth);

    let auth_service_routes = Router::new();
        
    Ok(())
}

