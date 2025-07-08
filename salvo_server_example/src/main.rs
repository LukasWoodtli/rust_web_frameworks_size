use salvo::prelude::*;
use salvo::serve_static::StaticDir;

// Handler for English greeting
#[handler]
async fn api() -> &'static str {
    "Hello from API"
}

#[tokio::main]
async fn main() {

    let acceptor = TcpListener::new("0.0.0.0:3000").bind().await;

    let router = Router::new().append(
        vec![
            Router::with_path("api").get(api),
            Router::with_path("{*path}").get(StaticDir::new(["www"])),
        ]
        .as_mut(),
    );

    // Print router structure for debugging
    println!("{router:?}");

    // Start serving requests
    Server::new(acceptor).serve(router).await;
}
