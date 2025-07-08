use poem::handler;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};

#[handler]
fn api() -> String {
    "Hello from API".to_string()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let app = Route::new().nest(
        "/",
        StaticFilesEndpoint::new("www").show_files_listing(),
    ).nest("api", api);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
