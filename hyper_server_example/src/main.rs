
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::path::{Path};

use bytes::Bytes;
use futures_util::TryStreamExt;
use http_body_util::{combinators::BoxBody, BodyExt, Full, StreamBody};
use hyper::service::service_fn;
use hyper::body::Frame;
use hyper::{Method, Request, Response, Result, StatusCode};
use tokio::{fs::File, net::TcpListener};
use tokio_util::io::ReaderStream;

// Inspired by: https://github.com/hyperium/hyper/blob/master/examples/send_file.rs

async fn response_examples(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, std::io::Error>>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api")  => api_call(),
        (&Method::GET, file_name) => file_send(file_name).await,
        _ => not_found(),
    }
}

fn api_call() -> Result<Response<BoxBody<Bytes, std::io::Error>>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Full::new("Hello from API".into()).map_err(|e| match e {}).boxed())
        .unwrap())
}

/// HTTP status code 404
fn not_found() -> Result<Response<BoxBody<Bytes, std::io::Error>>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new("Not Found".into()).map_err(|e| match e {}).boxed())
        .unwrap())
}


async fn file_send(filename: &str) -> Result<Response<BoxBody<Bytes, std::io::Error>>> {
    let filename = filename.to_string();
    let filename = match filename.strip_prefix("/") {
        Some(name) => name,
        _ => &filename
    };
    let path = Path::new("www");
    let path = path.join(filename);
    let file = File::open(path).await;
    if file.is_err() {
        return not_found();
    }

    let file: File = file.unwrap();

    // Wrap to a tokio_util::io::ReaderStream
    let reader_stream = ReaderStream::new(file);

    // Convert to http_body_util::BoxBody
    let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));
    let boxed_body = stream_body.boxed();

    // Send response
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(boxed_body)
        .unwrap();

    Ok(response)
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to create TCP listener");

    loop {
        let (stream, _) = listener
            .accept()
            .await?;

        tokio::spawn(async move {
            if let Err(err) = hyper::server::conn::http1::Builder::new()
                .serve_connection(
                    TokioIo::new(stream),
                    service_fn(response_examples),
                )
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
